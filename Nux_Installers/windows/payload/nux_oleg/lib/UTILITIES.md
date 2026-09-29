# Nux Convenience Libraries

These modules provide Python-like and Rust-like everyday building blocks while
the richer generic collection system is still being implemented.

## Numeric Arrays

`std.array_basic` provides managed-memory arrays:

```nux
var values = array_alloc(5);
array_set(values, 0, 42);
println(array_get(values, 0));
println(array_sum(values, 5));
array_free(values);
```

Available helpers include `array_fill`, `array_max`, and `array_reverse`.

## Algorithms

`algorithms.search_basic` includes `search_linear`, `search_binary`, and
`sort_selection`, plus `array_min`, `array_count`, and `array_reduce_sum`.
These operate on arrays created with `array_alloc`.

## Data Structures and Numeric Helpers

- `data_structures.ring_basic`: fixed-capacity ring storage for queues, audio,
  network packets, and game events.
- `std.numeric_basic`: absolute value, min/max, clamp, GCD, and integer lerp.

Loop temporaries in these basic modules are deliberately declared outside hot
loops. That keeps them compatible with the current VM while the compiler's
fixed frame-slot allocator is being developed.

## AI Helpers

`ai.activations` includes ReLU, step, leaky-ReLU, clamp, and a buffer-wide
ReLU sum. These are fixed-point-friendly primitives for small models and can
be replaced by native/LLVM kernels later without changing application code.

## Crypto and Testing

- `crypto.hash_basic`: deterministic word and byte hashes for IDs and caches.
- `testing.assert_basic`: lightweight equality, truth, nonzero, and reporting helpers.

The `*_basic` suffix is deliberate: these modules use only features currently
implemented by the standalone compiler. Rich generic `Vec`, `Map`, `Result`,
and `Option` libraries remain available as the next type-system milestone.