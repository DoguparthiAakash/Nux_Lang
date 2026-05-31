# Nux Programming Language — Syntax & Developer Guide

Welcome to **Nux**, a high-performance, memory-safe systems programming language designed for AI, ML, quantum computing, and OS development. Nux is designed to be highly readable for developers, while providing low-level memory control comparable to Rust and C/C++, and seamless multi-language interoperability via **LAG (Language Agnostic Gateway)**.

This guide provides a comprehensive visual and textual reference to help you write advanced Nux code with deep-level control and complete safety.

---

## 1. Core Principles

1. **Memory Safety without Overhead**: Compiles down to highly compressed natively interpretable bytecode (`.nxb`) or native machine code via a tiered JIT compiler.
2. **Explicit but Ergonomic**: No indentation-based block syntax (braces are required), eliminating whitespace ambiguity while keeping code highly structured.
3. **No Garbage Collection Overhead**: Uses clear ownership annotations (`@own` and borrows `&`, `&mut`) to manage memory deterministically.
4. **Frictionless Interoperability**: Embed Python, Rust, C++, Java, Zig, and Go directly in your code using `var.alias { ... }` blocks.

---

## 2. Variables and Primitive Types

Variables are mutable by default. Use `const` for compile-time constants.

```nux
# Mutable variable with type inference
var x = 42;             # Inferred as i32

# Explicit type annotation
var message: &str = "Hello, Nux!"; 

# Compile-time constants
const PI: f64 = 3.141592653589793;
```

### Supported Primitives
* **Signed Integers**: `i8`, `i16`, `i32`, `i64`, `i128`, `isize` (pointer size)
* **Unsigned Integers**: `u8`, `u16`, `u32`, `u64`, `u128`, `usize` (pointer size, standard alias: `index`)
* **Floats**: `f16`, `bf16` (brain float), `f32` (alias: `float`), `f64`, `f128`
* **Others**: `bool` (`true` / `false`), `char` (Unicode rune), `byte` (alias to `u8`), `void` (empty return), `never` (infinite loops or panics)

---

## 3. Data Structures and Compounds

Nux provides rich built-in types for collections, both stack-allocated (fixed size) and heap-allocated.

```nux
# Array (fixed-size, stack-allocated)
var coordinates: [i32; 3] = [10, 20, 30];

# Vec (dynamic, heap-allocated)
var numbers: @own Vec[i32] = Vec.new();
numbers.push(10);
numbers.push(20);

# Slice (borrowed view of an array/vec)
var slice: &[i32] = coordinates[0..2];

# HashMap
var scores: HashMap[String, i32] = HashMap.new();
scores.insert(String.from("Alice"), 95);

# Option Type (safe handling of null values)
var maybe_value: Option[i32] = Option.Some(42);
```

---

## 4. Ownership & Borrowing System

Nux guarantees memory safety at compile-time by enforcing ownership rules. 

* **`@own`**: Marks a variable as the sole owner of a resource. The resource is automatically freed when the owner's scope ends.
* **`&T`**: An immutable borrow (shared reference). Multiple components can read the value simultaneously.
* **`&mut T`**: A mutable borrow (exclusive reference). Only one component can write to the value at a time, preventing data races.

```nux
var data: @own Vec[i32] = Vec.new();

# Borrowing immutably
func read_data(v: &Vec[i32]) {
    println("Vector length: " + int_to_str(v.len));
}

# Borrowing mutably
func modify_data(v: &mut Vec[i32]) {
    v.push(100);
}

read_data(&data);       # OK: Passes an immutable reference
modify_data(&mut data); # OK: Passes an exclusive mutable reference
```

### Move Semantics
Assigning an `@own` value to another variable transfers ownership. The original variable becomes invalid.

```nux
var original = String.from("Nux");
var duplicate = original;  # ownership moved to 'duplicate'

# println(original);       # ERROR: use of moved value!
println(duplicate);        # OK
```

---

## 5. Control Flow

### If-Else Statements
Braces `{}` are mandatory. Parentheses `()` for conditions are optional but recommended.
```nux
if (x > 100) {
    println("Large");
} else if (x > 50) {
    println("Medium");
} else {
    println("Small");
}
```

### Loops
Nux supports standard `while` loops, infinite `loop` blocks, C-style `for` loops, and `for-in` iterators.
```nux
# While Loop
while (x > 0) {
    x -= 1;
}

# C-style For Loop
for (var i = 0; i < 10; i += 1) {
    println(int_to_str(i));
}

# Iterator/Range For Loop
for (var item in 0..100) {
    println("Item: " + int_to_str(item));
}
```

### Match Expressions (Exhaustive Pattern Matching)
Match ensures all potential options are covered.
```nux
match (value) {
    0          => { println("Zero"); },
    1..10      => { println("Small positive range"); },
    n if n < 0 => { println("Negative: " + int_to_str(n)); },
    _          => { println("Other value"); }
}
```

---

## 6. Object-Oriented Programming (Classes)

Nux provides dynamic classes with fields and member functions.

```nux
class Point {
    var x: f32;
    var y: f32;

    # Constructor
    func init(x: f32, y: f32) {
        this.x = x;
        this.y = y;
    }

    # Instance method taking an immutable reference to 'this'
    func distance(&self, other: &Point) -> f32 {
        var dx = this.x - other.x;
        var dy = this.y - other.y;
        return sqrt_f32(dx * dx + dy * dy);
    }
}

# Instantiation
var p1 = new Point(0.0, 0.0);
var p2 = new Point(3.0, 4.0);
var dist = p1.distance(&p2);  # 5.0
```

---

## 7. Language Agnostic Gateway (LAG)

One of Nux's most powerful features is **LAG**, allowing developers to run code from other languages natively in Nux without manual FFI overhead.

```nux
# Define a foreign execution binding
nux.lag python.3.13.7 = py;

# Create a variable to capture the foreign execution results
var script_output = new var;

# Execute Python block directly and bind output to variable
script_output.py {
    import math
    import sys
    print(f"Hello from Python {sys.version}!")
    # Output is automatically captured in script_output
}
```

---

## 8. Compiler Attributes and SIMD Acceleration

You can decorate your functions to guide the JIT and AOT compilers for raw metal performance.

```nux
# Vectorize using AVX2, ARM NEON, or RISC-V Vector Extensions (RVV)
@simd(avx2, neon, rvv)
func fast_matrix_add(a: &[f32], b: &[f32], out: &mut [f32]) {
    for (var i = 0; i < a.len; i += 1) {
        out[i] = a[i] + b[i];
    }
}

# Run in an isolated zone without garbage collector overhead
@nogc
func real_time_interrupt() {
    # System execution here
}
```
