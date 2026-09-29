# Nux Language Syntax Specification

## ✅ Standard Syntax

### **1. Function Declaration**
```nux
# Use 'func' keyword (consistent with modern languages)
func function_name(param1: Type1, param2: Type2) -> ReturnType {
    # function body
}

# Generic functions
func generic_function<T>(value: T) -> T {
    return value;
}

# Methods
class MyClass {
    func method_name(self, param: Type) -> ReturnType {
        # method body
    }
}
```

### **2. Variable Declaration**
```nux
# Use 'var' for mutable variables
var x: int = 10;
var name: string = "Hello";
var inferred = 42;  # Type inference

# Use 'const' for immutable
const PI: float = 3.14159;
```

### **3. Control Flow**
```nux
# If statements
if (condition) {
    # code
} else if (other_condition) {
    # code
} else {
    # code
}

### **4. Low-Level Memory and Pointers**

Nux provides explicit pointer-like memory handles for systems code. Allocated blocks
must be released with `mem_free`; byte and 64-bit access is bounds-checked.

```nux
var buffer = mem_alloc(64);
mem_write64(buffer, 1234);
var value = mem_read64(buffer);

mem_set(buffer, 0, 64);
var other = mem_alloc(64);
mem_copy(other, buffer, 64);

mem_free(buffer);
mem_free(other);
```

Available operations:

- `mem_alloc(bytes)` and `mem_free(pointer)`
- `mem_read8(pointer)` and `mem_write8(pointer, value)`
- `mem_read64(pointer)` and `mem_write64(pointer, value)`
- `mem_copy(destination, source, bytes)`
- `mem_set(pointer, byte, bytes)`
- `mem_size()`

`peek` and `poke` remain raw VM-memory operations for kernel/assembly work. Use
the `mem_*` API for ordinary manual allocation because it tracks ownership and
rejects invalid or double frees.

### **5. Native C and C++**

The CUX native driver accepts `.c`, `.cpp`, `.cc`, and `.cxx` files. C files use
`gcc` and C++ files use `g++`, allowing platform-specific drivers, FFI shims,
and hardware integrations to be built alongside Nux code.

The standalone compiler can bypass the Nux VM for the portable native subset:

```powershell
$env:NUX_CC = "zig"       # or gcc, clang, or a cross compiler
nux native program.nux program.o
```

For the LLVM path, use `clang` or set an LLVM-based compiler explicitly:

```powershell
$env:NUX_CLANG = "clang"  # zig also works when LLVM is bundled through Zig
nux llvm program.nux program.o
```

This emits the host toolchain's object format. The native backend currently
supports C-like functions, variables, expressions, control flow, printing, and
the `mem_*` APIs. VM-specific operations such as `peek`, `poke`, graphics, and
imports need a target runtime or a platform backend before they can be native.

# For loops
for (var i = 0; i < 10; i++) {
    # code
}

# A typed counter is also allowed
for (int i = 0; i < 10; i++) {
    # code
}

# Reuse a variable declared earlier
var i: int = 0;
for (i = 0; i < 10; i++) {
    # code
}

# Simple range loop: values are 0 through n - 1
for (i in rangeOf(10)) {
    # code
}

# While loops
while (condition) {
    # code
}

# Match expressions
match (value) {
    case 1: {
        println("one");
    }
    case 2: {
        println("two");
    }
    default: {
        println("something else");
    }
}

# Repeat until a condition becomes false
do {
    println("runs at least once");
} while (condition);

# Leave or skip the current loop
while (x < 10) {
    if (x == 5) { continue; }
    if (x == 8) { break; }
    x++;
}
```

### **6. Classes and Interfaces**
```nux
class ClassName {
    var field1: Type1;
    var field2: Type2;
    
    func new(param: Type) -> ClassName {
        return ClassName {
            field1: value1,
            field2: value2
        };
    }
    
    func method(self) -> ReturnType {
        return this.field1;
    }
}

interface InterfaceName {
    func method_name(param: Type) -> ReturnType;
}
```

### **7. Indentation Rules**

**Nux is NOT indentation-sensitive!**

```nux
# ✅ Valid (well-formatted)
func example() {
    var x = 10;
    if (x > 5) {
        println("Greater");
    }
}

# ✅ Also valid (poor style, but legal)
func example(){var x=10;if(x>5){println("Greater");}}

# ✅ Also valid (mixed indentation)
func example() {
  var x = 10;
    if (x > 5) {
        println("Greater");
    }
}
```

**Recommendation:** Use **4 spaces** for indentation (not tabs)

### **8. Comments**
```nux
# Single-line comment

/*
 * Multi-line comment
 */

#/ Documentation comment
func documented_function() {
    # ...
}
```

## 🔧 Migration Guide

### **Old Syntax (lexer.nux style) → New Syntax**

```nux
# OLD
func lexer_create(source) {
    var lexer = {
        source: source,
        pos: 0
    };
    return lexer;
}

# NEW
func lexer_create(source: string) -> Lexer {
    var lexer = Lexer {
        source: source,
        pos: 0
    };
    return lexer;
}
```

## 📋 Complete Syntax Summary

| Feature | Syntax | Example |
|---------|--------|---------|
| Function | `func name(params) -> Type { }` | `func add(a: int, b: int) -> int { }` |
| Variable | `var name: Type = value;` | `var x: int = 10;` |
| Constant | `const NAME: Type = value;` | `const PI: float = 3.14;` |
| If | `if (cond) { } else { }` | `if (x > 0) { println("positive"); }` |
| For | `for (init; cond; step) { }` | `for (int i = 0; i < 10; i++) { }` |
| Range For | `for (item in rangeOf(n)) { }` | `for (i in rangeOf(10)) { }` |
| While | `while (cond) { }` | `while (running) { update(); }` |
| Do-while | `do { } while (cond);` | `do { x++; } while (x < 3);` |
| Break | `break;` | `if (done) { break; }` |
| Continue | `continue;` | `if (skip) { continue; }` |
| Class | `class Name { fields; methods; }` | `class Point { var x: int; var y: int; }` |
| Interface | `interface Name { methods; }` | `interface Drawable { func draw(); }` |
| Match | `match (val) { case n: { } default: { } }` | `match (x) { case 1: { println("one"); } default: { println("other"); } }` |
| Comment | `# text` or `/* text */` | `# This is a comment` |
| Import | `import "module";` | `import "std/io";` |

## 🎯 Action Items

1. **Standardize lexer.nux** - Convert from `func`/`var` to `func`/`var`
2. **Update parser** - Ensure it accepts both styles (for backward compatibility)
3. **Create linter** - Warn about old-style syntax
4. **Update documentation** - Use only new syntax in examples
5. **Migration tool** - Auto-convert old code to new syntax

## 💡 Why This Matters

**Consistency = Clarity**
- Easier to learn
- Better tooling support
- Fewer bugs
- Professional appearance

**Nux should have ONE clear syntax, not multiple competing styles!**
