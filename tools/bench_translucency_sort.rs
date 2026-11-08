//! Standalone CPU microbenchmark; needs no game assets or GPU.
//! rustc --edition=2024 -O tools/bench_translucency_sort.rs -o /tmp/bench-translucency
//! /tmp/bench-translucency
use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

#[allow(dead_code)]
#[path = "../crates/horizon-client-render/src/gfx/translucency.rs"]
mod production;

struct Counted;
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);
// This developer-only executable counts allocations around the real sorter. The allocator
// delegates the unchanged layouts and pointers to System; it is not part of the game.
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, size: usize) -> *mut u8 {
        ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, size) }
    }
}
#[global_allocator]
static ALLOCATOR: Counted = Counted;

#[derive(Default)]
struct Baseline {
    order: Vec<(u32, f32)>,
    indices: Vec<u32>,
}
impl Baseline {
    // The previous production algorithm, including its stable distance-only ordering.
    fn sort(&mut self, centers: &[[f32; 3]], camera: [f32; 3]) -> &[u32] {
        self.order.clear();
        self.order.extend(centers.iter().enumerate().map(|(i, c)| {
            let [x, y, z] = std::array::from_fn(|j| camera[j] - c[j]);
            (i as u32, x * x + y * y + z * z)
        }));
        self.order.sort_by(|a, b| b.1.total_cmp(&a.1));
        self.indices.clear();
        for &(quad, _) in &self.order {
            let v = quad * 4;
            self.indices.extend([v, v + 1, v + 2, v + 2, v + 3, v]);
        }
        &self.indices
    }
}

fn measure(mut sort: impl FnMut([f32; 3]), iterations: usize) -> (u128, usize) {
    let before = ALLOCATIONS.load(Ordering::Relaxed);
    let start = Instant::now();
    for i in 0..iterations {
        sort(black_box([i as f32 * 0.03125, 24.0, -7.25]));
    }
    let elapsed = start.elapsed().as_nanos() / iterations as u128;
    (elapsed, ALLOCATIONS.load(Ordering::Relaxed) - before)
}

fn main() {
    println!(
        "quads,pattern,iterations,baseline_ns,corizon_ns,baseline_allocations,corizon_allocations"
    );
    for n in [256, 2048, 8192] {
        for tied in [false, true] {
            let mut seed = 0x1234_5678_u32;
            let centers: Vec<[f32; 3]> = (0..n)
                .map(|_| {
                    if tied {
                        [1.0; 3]
                    } else {
                        std::array::from_fn(|_| {
                            seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
                            (seed >> 8) as f32 / 1048576.0
                        })
                    }
                })
                .collect();
            let mut old = Baseline::default();
            let mut new = production::Sorter::default();
            assert_eq!(old.sort(&centers, [0.0; 3]), new.sort(&centers, [0.0; 3]));
            let iterations = 4096;
            let mut samples = Vec::new();
            for round in 0..7 {
                let mut baseline = || {
                    measure(
                        |cam| {
                            black_box(old.sort(black_box(&centers), cam));
                        },
                        iterations,
                    )
                };
                let mut corizon = || {
                    measure(
                        |cam| {
                            black_box(new.sort(black_box(&centers), cam));
                        },
                        iterations,
                    )
                };
                let (a, b) = if round % 2 == 0 {
                    (baseline(), corizon())
                } else {
                    let b = corizon();
                    (baseline(), b)
                };
                samples.push((a, b));
            }
            let mut old_times: Vec<_> = samples.iter().map(|x| x.0.0).collect();
            let mut new_times: Vec<_> = samples.iter().map(|x| x.1.0).collect();
            old_times.sort_unstable();
            new_times.sort_unstable();
            assert!(samples.iter().all(|x| x.1.1 == 0));
            println!(
                "{n},{},{iterations},{},{},{},{}",
                if tied { "ties" } else { "mixed" },
                old_times[3],
                new_times[3],
                samples[0].0.1,
                samples[0].1.1
            );
        }
    }
}
