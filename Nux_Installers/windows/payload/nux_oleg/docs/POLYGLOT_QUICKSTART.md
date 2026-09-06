# Nux Multi-Language Integration - Quick Start

## What is Polyglot Programming in Nux?

Nux now supports **polyglot programming**, allowing you to use multiple programming languages in a single `.nux` file. This means you can:

- Use Python's NumPy for data science
- Use JavaScript's Puppeteer for web scraping
- Use Rust's crypto libraries for performance
- Use C libraries for system programming

All within the same Nux program!

## Quick Examples

### Example 1: Python Integration

```nux
# Import Python library
import python:numpy as np

# Or use a Python code block
@python {
    import numpy as np
    
    def matrix_multiply(a, b):
        return np.dot(a, b)
}

# Call Python function from Nux
let result = python.matrix_multiply([[1,2],[3,4]], [[5,6],[7,8]])
print(result)
```

### Example 2: JavaScript Integration

```nux
# Import JavaScript library
import javascript:lodash as _

# Or use a JavaScript code block
@javascript {
    function processData(arr) {
        return arr.map(x => x * 2).filter(x => x > 10);
    }
}

# Call JavaScript function from Nux
let data = [1, 5, 10, 15, 20]
let processed = javascript.processData(data)
print(processed)  # Output: [20, 30, 40]
```

### Example 3: Rust Integration

```nux
# Use Rust for high-performance code
@rust {
    fn fibonacci(n: u64) -> u64 {
        match n {
            0 => 0,
            1 => 1,
            _ => fibonacci(n-1) + fibonacci(n-2)
        }
    }
}

let fib = rust.fibonacci(20)
print(fib)  # Output: 6765
```

### Example 4: C Integration

```nux
# Import C math library
import c:m

# Call C functions
let sqrt_result = m.sqrt(16.0)
let sin_result = m.sin(3.14159 / 2)

print("sqrt(16) = " + str(sqrt_result))  # Output: 4.0
print("sin(π/2) = " + str(sin_result))   # Output: ~1.0
```

## Inline Foreign Expressions

For quick one-liners:

```nux
let sum = @python(sum([1, 2, 3, 4, 5]))
let upper = @javascript("hello".toUpperCase())
let hash = @rust(std::collections::hash_map::DefaultHasher::new())
```

## Type Conversion

Nux automatically converts types between languages:

```nux
# Nux array → Python list
let nux_array = [1, 2, 3, 4, 5]
let py_result = python.process_list(nux_array)

# Python dict → Nux map
let py_dict = python.get_data()
print(py_dict["key"])  # Access like a Nux map
```

## Async/Await Support

```nux
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

## Security

Configure sandbox for untrusted code:

```nux
import std.polyglot

set_sandbox_config({
    "level": "restricted",
    "allowed_paths": ["/tmp"],
    "allow_network": true,
    "max_memory_mb": 512
})
```

## Full Examples

See `examples/polyglot/` for complete examples:

- **ml_pipeline.nux**: Machine learning with Python NumPy/scikit-learn
- **web_scraper.nux**: Web scraping with JavaScript Puppeteer
- **crypto_service.nux**: High-performance crypto with Rust

## Documentation

Read the full guide: [POLYGLOT_GUIDE.md](file:///home/aakash/Downloads/Nux_Lang/nux/docs/POLYGLOT_GUIDE.md)

## Installation

To use polyglot features, install the required language runtimes:

```bash
# Python
sudo apt install python3 python3-pip
pip install numpy pandas scikit-learn

# Node.js (for JavaScript)
sudo apt install nodejs npm
npm install -g lodash puppeteer

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# C compiler (usually pre-installed)
sudo apt install gcc
```

## Next Steps

1. Try the examples in `examples/polyglot/`
2. Read the comprehensive guide in `docs/POLYGLOT_GUIDE.md`
3. Experiment with your own multi-language programs!

---

**Happy Polyglot Programming! 🚀**
