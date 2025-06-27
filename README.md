running on M2 Max MBP

## gather

```
Timer precision: 41 ns
gather                    fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ bench_neon             223.6 ns      │ 642.9 ns      │ 228.8 ns      │ 234.4 ns      │ 1000    │ 32000
├─ bench_scalar           320 ns        │ 1.062 µs      │ 327.8 ns      │ 342.4 ns      │ 1000    │ 16000
╰─ bench_scalar_unrolled  218.4 ns      │ 1.114 µs      │ 226.2 ns      │ 240.7 ns      │ 1000    │ 16000
```

## isnan

```
isnan            fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ multiversion  139 ns        │ 146.8 ns      │ 141.6 ns      │ 142.2 ns      │ 100     │ 3200
├─ neon          119.4 ns      │ 178.1 ns      │ 126 ns        │ 129 ns        │ 100     │ 3200
╰─ scalar        139 ns        │ 204.1 ns      │ 141.6 ns      │ 147.3 ns      │ 100     │ 3200
```
