running on M2 Max MBP

```
Timer precision: 41 ns
gather                    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_neon             223.6 ns      │ 642.9 ns      │ 228.8 ns      │ 234.4 ns      │ 1000    │ 32000
├─ bench_scalar           320 ns        │ 1.062 µs      │ 327.8 ns      │ 342.4 ns      │ 1000    │ 16000
╰─ bench_scalar_unrolled  218.4 ns      │ 1.114 µs      │ 226.2 ns      │ 240.7 ns      │ 1000    │ 16000
```
