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

# For loops
for (var i = 0; i < 10; i++) {
    # code
}

# While loops
while (condition) {
    # code
}

# Match expressions
match (value) {
    pattern1 => expression1,
    pattern2 => expression2,
    _ => default_expression
}
```

### **4. Classes and Interfaces**
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

### **5. Indentation Rules**

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

### **6. Comments**
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
| For | `for (init; cond; inc) { }` | `for (var i = 0; i < 10; i++) { }` |
| While | `while (cond) { }` | `while (running) { update(); }` |
| Class | `class Name { fields; methods; }` | `class Point { var x: int; var y: int; }` |
| Interface | `interface Name { methods; }` | `interface Drawable { func draw(); }` |
| Match | `match (val) { pat => expr }` | `match (x) { 1 => "one", _ => "other" }` |
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
