# Nux Polyglot Programming Guide

## Introduction

Nux supports **polyglot programming**, allowing you to seamlessly integrate code from multiple programming languages within a single `.nux` file. This powerful feature enables you to:

- **Leverage existing ecosystems**: Use Python's NumPy, JavaScript's npm packages, Rust's crates, and C libraries
- **Choose the right tool**: Use Python for ML, JavaScript for web scraping, Rust for performance, C for system programming
- **Maximize productivity**: Combine the strengths of different languages in one cohesive application

## Supported Languages

- **Python** (via PyO3)
- **JavaScript** (via V8/QuickJS)
- **Rust** (via dynamic libraries)
- **C/C++** (via libffi)
- **Java** (via JNI) - Coming soon
- **Go** (via CGO) - Coming soon

---

## Language Block Syntax

### Basic Syntax

Embed foreign language code using the `@language { ... }` syntax:

```nux
@python {
    def hello(name):
        return f"Hello, {name}!"
}

@javascript {
    function greet(name) {
        return `Greetings, ${name}!`;
    }
}

@rust {
    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n-1) + fibonacci(n-2)
        }
    }
}

# Call foreign functions from Nux
let msg1 = python.hello("Alice")
let msg2 = javascript.greet("Bob")
let fib = rust.fibonacci(10)

print(msg1)  # Output: Hello, Alice!
print(msg2)  # Output: Greetings, Bob!
print(fib)   # Output: 55
```

### Multiple Blocks

You can have multiple blocks of the same language:

```nux
@python {
    import numpy as np
    
    def create_array(size):
        return np.zeros(size)
}

@python {
    def process_array(arr):
        return arr * 2 + 1
}

let arr = python.create_array(10)
let result = python.process_array(arr)
```

---

## External Library Imports

### Import Entire Modules

```nux
import python:numpy as np
import javascript:lodash as _
import rust:serde_json

# Use imported libraries
let arr = np.array([1, 2, 3, 4, 5])
let chunked = _.chunk([1, 2, 3, 4, 5, 6], 2)
let json = serde_json.to_string({"key": "value"})
```

### Import Specific Items

```nux
from python:pandas import DataFrame, Series
from javascript:express import Router, middleware
from rust:tokio import runtime::Runtime

# Use imported items
let df = DataFrame({"col1": [1, 2, 3], "col2": [4, 5, 6]})
let router = Router()
let runtime = Runtime.new()
```

### System Libraries (C/C++)

```nux
import c:m      # Math library
import c:pthread  # POSIX threads
import c:sqlite3  # SQLite

# Call C functions
let result = m.sqrt(16.0)  # Returns 4.0
```

---

## Inline Foreign Expressions

For quick one-liners, use the `@lang(...)` syntax:

```nux
let sum = @python(sum([1, 2, 3, 4, 5]))
let upper = @javascript("hello world".toUpperCase())
let hash = @rust(std::collections::hash_map::DefaultHasher::new())

print(sum)    # Output: 15
print(upper)  # Output: HELLO WORLD
```

---

## Type Marshalling

Nux automatically converts types between languages:

| Nux Type | Python | JavaScript | Rust | C |
|----------|--------|------------|------|---|
| `int` | `int` | `number` | `i64` | `int64_t` |
| `float` | `float` | `number` | `f64` | `double` |
| `string` | `str` | `string` | `String` | `char*` |
| `bool` | `bool` | `boolean` | `bool` | `bool` |
| `array` | `list` | `Array` | `Vec<T>` | `T*` + length |
| `map` | `dict` | `Object` | `HashMap<K,V>` | `struct` |
| `null` | `None` | `null` | `Option::None` | `NULL` |

### Custom Type Conversion

```nux
import std.polyglot

let converter = TypeConverter()

# Get type mapping
let py_type = converter.get_type_mapping("python", "array")
print(py_type)  # Output: list

# Manual conversion
let nux_value = [1, 2, 3]
let py_value = converter.convert_to(nux_value, "python")
```

---

## Async/Await Interop

### JavaScript Promises

```nux
import std.interop

@javascript {
    async function fetchData(url) {
        const response = await fetch(url);
        return await response.json();
    }
}

# Await JavaScript Promise in Nux
let data = await javascript.fetchData("https://api.example.com/data")
print(data)
```

### Python Asyncio

```nux
@python {
    import asyncio
    
    async def async_task():
        await asyncio.sleep(1)
        return "Task complete"
}

let result = await python.async_task()
```

---

## Callbacks and Events

### Nux Callbacks in Foreign Code

```nux
import std.interop

# Register Nux callback
let callback_id = register_callback(fn(data) {
    print("Callback received: " + str(data))
})

@javascript {
    function processWithCallback(callback_id, data) {
        // Call Nux callback from JavaScript
        nux.call_callback(callback_id, data);
    }
}

javascript.processWithCallback(callback_id, {"status": "success"})
```

### Event Bridge

```nux
import std.interop

# Listen for events
on_event("data_received", fn(data) {
    print("Event data: " + str(data))
})

@python {
    def emit_event(event_name, data):
        nux.emit_event(event_name, data)
}

python.emit_event("data_received", {"value": 42})
```

---

## Security and Sandboxing

### Sandbox Levels

1. **Trusted** (default for user code):
   - Full system access
   - No restrictions

2. **Restricted** (for external libraries):
   - Limited file system access
   - Network requires permission
   - No system calls

3. **Isolated** (for untrusted code):
   - Separate process
   - IPC communication
   - Resource limits

### Setting Permissions

```nux
import std.polyglot

# Configure sandbox
set_sandbox_config({
    "level": "restricted",
    "allowed_paths": ["/tmp", "/home/user/data"],
    "allow_network": true,
    "allow_system_calls": false,
    "max_memory_mb": 512,
    "max_execution_time_ms": 30000
})

# Import with restrictions
import python:requests  # Requires network permission
```

### Permission Declarations

```nux
# Declare required permissions at the top of the file
@permissions {
    python: ["network", "filesystem:/tmp"]
    javascript: ["network"]
    rust: ["filesystem:/var/lib/myapp"]
}

import python:requests
import javascript:puppeteer
```

---

## Best Practices

### 1. Choose the Right Language for the Task

```nux
# Python for data science and ML
@python {
    import pandas as pd
    import sklearn
}

# JavaScript for web scraping and browser automation
@javascript {
    const puppeteer = require('puppeteer');
}

# Rust for performance-critical code
@rust {
    use rayon::prelude::*;
}

# C for system-level operations
@c {
    #include <sys/socket.h>
}
```

### 2. Minimize Cross-Language Calls

**Bad:**
```nux
for i in range(1000) {
    let result = python.process_item(i)  # 1000 FFI calls!
}
```

**Good:**
```nux
let results = python.process_items(range(1000))  # 1 FFI call
```

### 3. Use Shared Memory for Large Data

```nux
import std.interop

# Create shared memory buffer
let shared_mem = create_shared_memory(1024 * 1024)  # 1 MB

# Write data
shared_mem.write(0, large_data_array)

# Pass to foreign code (zero-copy)
python.process_shared_memory(shared_mem)
```

### 4. Handle Errors Gracefully

```nux
try {
    let result = python.risky_operation()
} catch (error) {
    print("Python error: " + str(error))
    # Fallback to Nux implementation
    result = nux_fallback_operation()
}
```

### 5. Clean Up Resources

```nux
import std.polyglot

# Initialize runtimes
init_runtime("python")
init_runtime("javascript")

# ... do work ...

# Shutdown when done
shutdown_all_runtimes()
```

---

## Performance Considerations

### FFI Call Overhead

- **Python (PyO3)**: ~100-200ns per call
- **JavaScript (V8)**: ~50-100ns per call
- **Rust (dynamic lib)**: ~10-20ns per call
- **C (libffi)**: ~5-10ns per call

### Optimization Tips

1. **Batch operations**: Process arrays instead of individual items
2. **Use native Nux**: For simple operations, Nux may be faster than FFI
3. **Cache foreign objects**: Reuse objects instead of recreating
4. **Profile your code**: Use `debug.CPUProfiler` to find bottlenecks

---

## Common Patterns

### Pattern 1: Data Pipeline

```nux
# Load data (Python)
let df = python.load_csv("data.csv")

# Transform (Nux)
let filtered = filter_data(df)

# Analyze (Python)
let stats = python.compute_statistics(filtered)

# Visualize (JavaScript)
javascript.create_chart(stats)
```

### Pattern 2: Web Service with Performance

```nux
import std.webframework

# Rust for crypto
@rust {
    fn hash_password(password: &str) -> String {
        // Fast bcrypt implementation
    }
}

# Nux for web server
let server = WebServer(8080)

server.post("/login", fn(req, res) {
    let password = req.body["password"]
    let hash = rust.hash_password(password)
    # ... authenticate ...
})
```

### Pattern 3: Multi-Language Testing

```nux
import std.testing

@python {
    def python_implementation(x):
        return x ** 2
}

@rust {
    fn rust_implementation(x: i64) -> i64 {
        x * x
    }
}

# Test both implementations
let suite = TestSuite("Cross-language tests")

suite.test("Python vs Rust", fn() {
    for i in range(100) {
        let py_result = python.python_implementation(i)
        let rust_result = rust.rust_implementation(i)
        assert_equal(py_result, rust_result)
    }
})

suite.run()
```

---

## Troubleshooting

### Issue: "Runtime not found"

**Solution**: Ensure the language runtime is installed:

```bash
# Python
python3 --version

# Node.js
node --version

# Rust
rustc --version

# GCC/Clang
gcc --version
```

### Issue: "Module not found"

**Solution**: Install the required package:

```bash
# Python
pip install numpy pandas

# JavaScript
npm install lodash puppeteer

# Rust
cargo install <crate-name>
```

### Issue: "Type conversion error"

**Solution**: Use explicit type conversion:

```nux
# Instead of:
let result = python.func(my_map)

# Use:
let converter = TypeConverter()
let py_dict = converter.convert_to(my_map, "python")
let result = python.func(py_dict)
```

---

## Examples

See the `examples/polyglot/` directory for complete examples:

- `ml_pipeline.nux`: Machine learning with Python NumPy/scikit-learn
- `web_scraper.nux`: Web scraping with JavaScript Puppeteer
- `crypto_service.nux`: High-performance crypto with Rust
- `data_analysis.nux`: Multi-language data pipeline
- `game_engine.nux`: C++ physics + Nux game logic

---

## API Reference

### `std.polyglot`

- `LanguageRuntime(language: string)`: Create runtime instance
- `init_runtime(language: string)`: Initialize language runtime
- `execute_foreign(language: string, code: string)`: Execute code
- `load_external_module(language: string, path: string)`: Load module
- `call_foreign_function(lang: string, module: string, func: string, args: array)`: Call function
- `set_sandbox_config(config: map)`: Configure security sandbox
- `shutdown_all_runtimes()`: Clean up all runtimes

### `std.interop`

- `CallbackBridge()`: Manage cross-language callbacks
- `SharedMemory(size: int)`: Create shared memory buffer
- `EventBridge()`: Cross-language event system
- `AsyncBridge()`: Promise/async interop
- `create_promise(executor: function)`: Create Promise
- `await_promise(promise_id: int)`: Wait for Promise

---

## Future Enhancements

- **JVM Support**: Java, Kotlin, Scala integration
- **Go Support**: CGO-based integration
- **WebAssembly**: Compile Nux to WASM, run WASM modules
- **GPU Compute**: CUDA/OpenCL integration
- **Distributed Computing**: Multi-node polyglot execution

---

## Contributing

We welcome contributions to expand polyglot support! See `CONTRIBUTING.md` for guidelines.

## License

Nux Polyglot features are licensed under the MIT License.
