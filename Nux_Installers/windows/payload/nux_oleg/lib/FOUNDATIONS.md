# Nux Foundation Libraries

These modules are written in Nux and target the currently supported runtime primitives.
They are intentionally small, composable foundations for larger AI, ML, OS, application,
and game libraries.

## AI and ML

- `ai.tensor`: managed-memory tensor allocation, indexed access, fill, dot product, ReLU, and argmax.
- `ml.linear`: scalar linear prediction, error calculation, and gradient-style update steps.

```nux
import "ai.tensor";
import "ml.linear";

var values = tensor_alloc(3);
tensor_set(values, 0, 2);
tensor_set(values, 1, 3);
tensor_set(values, 2, 4);
println(tensor_argmax(values, 3));
println(linear_predict(2, 10, 1));
tensor_free(values);
```

## Operating System and Applications

- `os.runtime`: platform code, VM memory capacity, address checks, and status requirements.
- `app.result`: status helpers, clamping, and conditional selection.

Platform codes are `1` for Windows, `2` for Linux, `3` for macOS, and `0` for other targets.

## Games

- `game.vector2`: allocation-free 2D vector arithmetic, dot products, distance, and movement.

## Design Rule

Keep domain logic in Nux. Put platform-specific operations behind small runtime or native
backends. This lets AI, ML, application, and game code share the same source while the
runtime chooses VM, native, LLVM, or a hardware backend.
