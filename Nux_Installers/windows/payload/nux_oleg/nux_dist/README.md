# Nux Programming Language

A lightweight, embeddable programming language designed for the Ainux kernel.

## Features

- **Simple Syntax**: Easy to learn and use
- **Compiled**: Compiles to bytecode for efficient execution
- **Type System**: Support for int, float, string, and custom types
- **Functions & Classes**: Object-oriented programming support
- **Built-in Intrinsics**: Math, I/O, and system operations

## Building

```bash
cargo build --release
```

## Usage

### Compile and view assembly:
```bash
./target/release/nux script.nux
```

### Compile to bytecode:
```bash
./target/release/nux compile script.nux output.nuxi
```

## Library Usage

Add to your `Cargo.toml`:
```toml
[dependencies]
nux = { path = "../nux_dist" }
```

Use in your code:
```rust
use nux::{compile, compile_to_asm};

fn main() {
    let source = "println(\"Hello, World!\");";
    
    // Compile to assembly
    match compile_to_asm(source) {
        Ok(asm) => println!("{}", asm),
        Err(errors) => {
            for err in errors {
                eprintln!("{}", err);
            }
        }
    }
}
```

## Language Syntax

### Variables
```nux
var x = 10;
var name = "Alice";
```

### Functions
```nux
func add(a, b) {
    return a + b;
}
```

### Classes
```nux
class Point {
    var x;
    var y;
    
    func init(px, py) {
        x = px;
        y = py;
    }
}
```

## License

MIT
