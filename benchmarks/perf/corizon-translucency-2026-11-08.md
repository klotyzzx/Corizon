# Corizon transparency sorting microbenchmark

Date: 2026-11-08. Baseline: the stable distance-only sorter at `15ceacba6e320bff1043da9790b021c7c4830f8a`.

Environment: Linux x86-64, Intel Xeon Platinum 8573C, rustc 1.99.1, `rustc -O`, System allocator with allocation counting.

Reproduce from the repository root:

```sh
rustc --edition=2024 -O tools/bench_translucency_sort.rs -o /tmp/bench-translucency
/tmp/bench-translucency
```

Seven rounds of 4,096 sorts per case, alternating which implementation goes first; median nanoseconds per sort. Buffers are warmed before timing. Mixed centers are deterministic points in a 16-block cube; the camera moves. The ties case puts every center at the same point to exercise stable-order and already-ordered behavior.

| Quads | Pattern | Baseline ns | Corizon ns | Time change | Baseline allocations / round | Corizon allocations / round |
| ---: | --- | ---: | ---: | ---: | ---: | ---: |
| 256 | mixed | 4,644 | 3,144 | -32.3% | 0 | 0 |
| 256 | ties | 921 | 1,015 | +10.2% | 0 | 0 |
| 2048 | mixed | 56,603 | 35,259 | -37.7% | 4096 | 0 |
| 2048 | ties | 9,585 | 8,254 | -13.9% | 4096 | 0 |
| 8192 | mixed | 321,746 | 186,744 | -42.0% | 4096 | 0 |
| 8192 | ties | 34,635 | 34,836 | +0.6% | 4096 | 0 |

The mixed cases took 32–42% less CPU time in this run. The 256-quad all-ties case was about 0.1 microsecond slower, and the large all-ties case was approximately unchanged; this is not a universal speedup for every input. At 2,048 and 8,192 quads, one temporary allocation per old sort disappeared. The old small sorter already used no heap scratch.

The optimized code packs the descending `f32::total_cmp` distance and original quad index into a 64-bit integer key. An in-place integer sort preserves exact stable tie order. Already-ordered input skips sorting. Vertex data, triangle winding, viewpoint caching, graphics settings and gameplay are unchanged.

Validation: tests compare every emitted index with the previous stable sorter over thousands of centers, duplicate distances, infinities, NaNs, tiny/overflowing values and several camera positions. Existing cache/viewpoint tests remain in place.

Limits: this is a synthetic CPU microbenchmark, not an in-game FPS or GPU benchmark. The game normally uses mimalloc; this benchmark uses System. Results vary by machine and scene. Minecraft resources are unavailable on this build host, so the Java/vanilla parity oracle and real-world FPS comparisons were not run. The Windows build workflow repeats this microbenchmark and includes its own CSV in the downloadable game archive.
