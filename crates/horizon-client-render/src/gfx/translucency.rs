//! Vanilla 26.3 MeshData.SortState / VertexSorting.byDistance.
//! Keep vertex order intact; only the quad indices change as the camera moves.

/// MeshData.decodeQuadCentroids uses the opposite corners, not all four vertices.
pub fn quad_center(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    std::array::from_fn(|i| (a[i] + b[i]) / 2.0)
}

#[derive(Default)]
pub struct Sorter {
    order: Vec<u64>,
    indices: Vec<u32>,
}

impl Sorter {
    pub fn sort(&mut self, centers: &[[f32; 3]], camera: [f32; 3]) -> &[u32] {
        self.order.clear();
        let mut already_ordered = true;
        let mut previous = 0;
        self.order.extend(centers.iter().enumerate().map(|(i, c)| {
            let [x, y, z] = std::array::from_fn(|j| camera[j] - c[j]);
            let bits = (x * x + y * y + z * z).to_bits();
            // Map all f32 bit patterns to their total_cmp order, then reverse that order
            // for back-to-front drawing. The low half preserves input order for ties.
            let sign = ((bits as i32) >> 31) as u32;
            let descending = !(bits ^ (sign | 0x8000_0000));
            let key = (u64::from(descending) << 32) | u64::from(i as u32);
            already_ordered &= previous <= key;
            previous = key;
            key
        }));
        // VertexSorting preserves mesh order for equal distances. The input is in ascending
        // quad-index order. Packed keys encode the same stable result while comparing plain
        // integers, without floating-point comparisons or merge-sort scratch on each resort.
        if !already_ordered {
            self.order.sort_unstable();
        }
        self.indices.clear();
        for &key in &self.order {
            let v = (key as u32) * 4;
            self.indices.extend([v, v + 1, v + 2, v + 2, v + 3, v]);
        }
        &self.indices
    }
}

pub struct SortState {
    centers: Vec<[f32; 3]>,
    last: Option<([i32; 3], [i32; 3])>,
}

impl SortState {
    pub fn new(centers: Vec<[f32; 3]>) -> Self {
        Self {
            centers,
            last: None,
        }
    }

    pub fn is_sorted(&self) -> bool {
        self.last.is_some()
    }

    /// Large tiles cannot use the section viewpoint shortcut: the camera may
    /// cross a quad's plane while remaining on the same side of the first 16³.
    pub fn update_lod<'a>(
        &mut self,
        camera: [f64; 3],
        origin: [i32; 3],
        sorter: &'a mut Sorter,
    ) -> Option<&'a [u32]> {
        let block = camera.map(|v| v.floor() as i32);
        if self.last.is_some_and(|(last, _)| last == block) {
            return None;
        }
        self.last = Some((block, [0; 3]));
        Some(sorter.sort(
            &self.centers,
            std::array::from_fn(|i| (camera[i] - f64::from(origin[i])) as f32),
        ))
    }

    /// TranslucencyPointOfView and ResortTransparencyTask: diagonal views keep
    /// their ordering until the point of view changes; aligned views also
    /// resort on crossing a block boundary. Sub-block movement needs no upload.
    pub fn update<'a>(
        &mut self,
        camera: [f64; 3],
        origin: [i32; 3],
        sorter: &'a mut Sorter,
    ) -> Option<&'a [u32]> {
        let block = camera.map(|v| v.floor() as i32);
        let pov = std::array::from_fn(|i| ((block[i] >> 4) - (origin[i] >> 4)).clamp(-1, 1));
        if let Some((last_block, last_pov)) = self.last
            && last_pov == pov
            && (last_block == block || !pov.contains(&0))
        {
            return None;
        }
        self.last = Some((block, pov));
        Some(sorter.sort(
            &self.centers,
            std::array::from_fn(|i| (camera[i] - f64::from(origin[i])) as f32),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equal_distances_keep_mesh_order_and_triangle_winding() {
        let mut sorter = Sorter::default();
        assert_eq!(
            sorter.sort(
                &[[1.0, 0.0, 0.0], [-1.0, 0.0, 0.0], [4.0, 0.0, 0.0]],
                [0.0; 3]
            ),
            &[8, 9, 10, 10, 11, 8, 0, 1, 2, 2, 3, 0, 4, 5, 6, 6, 7, 4]
        );
        assert!(sorter.sort(&[], [0.0; 3]).is_empty());
    }

    #[test]
    fn movement_reverses_faces_without_moving_vertices() {
        let centers = [[0.5, 0.5, 0.0], [0.5, 0.5, 1.0]];
        let mut sorter = Sorter::default();
        assert_eq!(sorter.sort(&centers, [0.5, 0.5, -2.0])[0], 4);
        assert_eq!(sorter.sort(&centers, [0.5, 0.5, 3.0])[0], 0);
    }

    #[test]
    fn in_place_order_matches_stable_reference_across_camera_moves() {
        let mut centers: Vec<[f32; 3]> = (0..4096)
            .map(|i| {
                [
                    ((i * 37) % 127) as f32 - 63.0,
                    ((i * 13) % 31) as f32 - 15.0,
                    ((i * 19) % 127) as f32 - 63.0,
                ]
            })
            .collect();
        // Repeated centers and non-finite distances exercise the stable tie semantics,
        // including total_cmp's ordering of infinities and NaN payloads.
        centers.extend([[1.0; 3]; 64]);
        centers.extend([
            [f32::INFINITY, 0.0, 0.0],
            [f32::NEG_INFINITY, 0.0, 0.0],
            [f32::from_bits(0x7fc0_0001), 0.0, 0.0],
            [f32::from_bits(0x7fc0_0002), 0.0, 0.0],
            [f32::from_bits(0xffc0_0001), 0.0, 0.0],
            [f32::MAX, 0.0, 0.0],
            [f32::MIN_POSITIVE, 0.0, 0.0],
            [-0.0; 3],
        ]);
        let mut sorter = Sorter::default();
        for camera in [[0.0; 3], [0.5, -12.0, 63.0], [-64.0, 15.0, -0.0]] {
            let mut reference: Vec<_> = centers
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    let dx = camera[0] - c[0];
                    let dy = camera[1] - c[1];
                    let dz = camera[2] - c[2];
                    (i as u32, dx * dx + dy * dy + dz * dz)
                })
                .collect();
            reference.sort_by(|a, b| b.1.total_cmp(&a.1));
            let expected: Vec<u32> = reference
                .into_iter()
                .flat_map(|(quad, _)| {
                    let v = quad * 4;
                    [v, v + 1, v + 2, v + 2, v + 3, v]
                })
                .collect();
            assert_eq!(sorter.sort(&centers, camera), expected);
        }
    }

    #[test]
    fn ordered_mesh_keeps_ties_and_resorts_after_the_camera_moves() {
        let centers = [
            [3.0, 0.0, 0.0],
            [2.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ];
        let mut sorter = Sorter::default();
        let first: Vec<_> = sorter
            .sort(&centers, [0.0; 3])
            .chunks_exact(6)
            .map(|q| q[0])
            .collect();
        assert_eq!(first, [0, 4, 8, 12]);
        let moved: Vec<_> = sorter
            .sort(&centers, [100.0, 0.0, 0.0])
            .chunks_exact(6)
            .map(|q| q[0])
            .collect();
        assert_eq!(moved, [8, 12, 4, 0]);
    }

    #[test]
    fn cached_viewpoint_handles_negative_and_large_coordinates() {
        let origin = [-30_000_000, -64, 30_000_000];
        let mut state = SortState::new(vec![[0.5; 3]]);
        let mut sorter = Sorter::default();
        let camera = [-29_999_999.5, -63.5, 30_000_000.5];
        assert!(state.update(camera, origin, &mut sorter).is_some());
        assert!(state.update(camera, origin, &mut sorter).is_none());
        assert!(
            state
                .update([-29_999_999.1, -63.1, 30_000_000.9], origin, &mut sorter)
                .is_none()
        );
        assert!(
            state
                .update([-29_999_998.9, -63.1, 30_000_000.9], origin, &mut sorter)
                .is_some()
        );
    }
    #[test]
    fn large_tiles_resort_inside_the_same_section_viewpoint() {
        let mut state = SortState::new(vec![[32.0, 0.0, 0.0], [96.0, 0.0, 0.0]]);
        let mut sorter = Sorter::default();
        assert_eq!(
            state
                .update_lod([48.0, 20.0, 20.0], [0; 3], &mut sorter)
                .unwrap()[0],
            4
        );
        assert!(
            state
                .update_lod([48.5, 20.0, 20.0], [0; 3], &mut sorter)
                .is_none()
        );
        // Both views have positive x/y/z signs relative to the first section,
        // so the section-sized shortcut would incorrectly keep the old order.
        assert_eq!(
            state
                .update_lod([80.0, 20.0, 20.0], [0; 3], &mut sorter)
                .unwrap()[0],
            0
        );
    }
}
