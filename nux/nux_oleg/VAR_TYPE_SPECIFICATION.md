# Nux `var` - Customizable Range-Constrained Type

## 🎯 Revolutionary Feature: Custom-Sized Variables

Unlike traditional languages where `int`, `float`, `char` have **fixed sizes**, Nux's `var` allows you to define **custom value ranges** to save memory and enforce constraints!

## 📐 Syntax

```nux
// Traditional (fixed size)
int x;        // Always 32 or 64 bits
char c;       // Always 8 bits

// Nux var (customizable range)
var<range> name;
```

## 💡 Examples

### **1. Letter Ranges**

```nux
// Only lowercase a-d (2 bits needed: 4 values)
var<'a'..'d'> letter1;
letter1 = 'a';  // ✓ Valid
letter1 = 'b';  // ✓ Valid
letter1 = 'e';  // ✗ Compile error!

// Only uppercase A-D
var<'A'..'D'> letter2;

// Mixed case a-D (requires more bits)
var<'a'..'D'> letter3;
```

### **2. Number Ranges**

```nux
// Only 1-c (hexadecimal, 12 values)
var<1..0xc> hex_digit;

// Only 5-E (5 to 14 in hex)
var<5..0xE> custom_range;

// Age (0-120, 7 bits)
var<0..120> age;

// Percentage (0-100, 7 bits)
var<0..100> percentage;

// Dice roll (1-6, 3 bits)
var<1..6> dice;
```

### **3. Custom Enumerations**

```nux
// Days of week (0-6, 3 bits)
var<0..6> day_of_week;

// Month (1-12, 4 bits)
var<1..12> month;

// Hour (0-23, 5 bits)
var<0..23> hour;
```

## 🔧 Implementation

### **Compiler Optimization**

The compiler automatically calculates the minimum bits needed:

```nux
var<0..3> x;      // 2 bits (4 values: 0,1,2,3)
var<0..7> y;      // 3 bits (8 values)
var<0..15> z;     // 4 bits (16 values)
var<0..255> w;    // 8 bits (256 values)
```

### **Memory Savings**

```nux
// Traditional approach
struct OldWay {
    int day;      // 32 bits
    int month;    // 32 bits
    int year;     // 32 bits
}
// Total: 96 bits

// Nux var approach
struct NewWay {
    var<1..31> day;      // 5 bits
    var<1..12> month;    // 4 bits
    var<0..9999> year;   // 14 bits
}
// Total: 23 bits (76% memory savings!)
```

## 🎨 Advanced Features

### **1. Character Sets**

```nux
// Hexadecimal digits
var<'0'..'9', 'A'..'F'> hex_char;

// Vowels only
var<'a', 'e', 'i', 'o', 'u'> vowel;

// Alphanumeric
var<'0'..'9', 'a'..'z', 'A'..'Z'> alphanum;
```

### **2. Named Ranges**

```nux
// Define reusable range types
type DiceRoll = var<1..6>;
type Percentage = var<0..100>;
type RGB = var<0..255>;

// Use them
var roll: DiceRoll = 4;
var opacity: Percentage = 75;
var red: RGB = 255;
```

### **3. Bit-Packing**

```nux
// Compiler automatically packs these together
struct PackedData {
    var<0..1> flag1;        // 1 bit
    var<0..1> flag2;        // 1 bit
    var<0..3> state;        // 2 bits
    var<0..15> counter;     // 4 bits
}
// Total: 8 bits (1 byte) instead of 16 bytes!
```

## 🚀 Real-World Use Cases

### **1. Game Development**

```nux
struct Player {
    var<0..100> health;           // 7 bits
    var<0..100> mana;             // 7 bits
    var<1..99> level;             // 7 bits
    var<0..999999> score;         // 20 bits
    var<0..3> team;               // 2 bits
}
// Total: 43 bits vs 160 bits (73% savings)
```

### **2. Network Protocols**

```nux
struct PacketHeader {
    var<0..15> version;           // 4 bits
    var<0..3> type;               // 2 bits
    var<0..65535> length;         // 16 bits
    var<0..255> checksum;         // 8 bits
}
// Perfectly packed into 30 bits
```

### **3. Embedded Systems**

```nux
struct SensorData {
    var<-50..50> temperature;     // 7 bits (signed)
    var<0..100> humidity;         // 7 bits
    var<0..1023> light_level;     // 10 bits
    var<0..1> motion_detected;    // 1 bit
}
// Total: 25 bits (3.125 bytes)
```

### **4. Database Optimization**

```nux
struct UserRecord {
    var<0..150> age;              // 8 bits
    var<'M', 'F', 'O'> gender;    // 2 bits
    var<0..4> rating;             // 3 bits
    var<0..1> active;             // 1 bit
}
// Fits in 2 bytes instead of 16 bytes
```

## 🔍 Comparison

| Type | Traditional | Nux var | Savings |
|------|-------------|---------|---------|
| Boolean | 8 bits | 1 bit | 87.5% |
| Dice (1-6) | 32 bits | 3 bits | 90.6% |
| Percentage | 32 bits | 7 bits | 78.1% |
| RGB Color | 32 bits | 8 bits | 75% |
| Day of Month | 32 bits | 5 bits | 84.4% |

## 💻 Compiler Implementation

```cpp
// Compiler calculates minimum bits needed
int calculate_bits_needed(int min, int max) {
    int range = max - min + 1;
    return ceil(log2(range));
}

// Example:
// var<0..3>   → 2 bits
// var<0..7>   → 3 bits
// var<0..15>  → 4 bits
// var<1..100> → 7 bits
```

## ⚡ Performance

**Benefits:**
- ✅ Reduced memory usage (up to 90% savings)
- ✅ Better cache utilization
- ✅ Faster data transfer
- ✅ Compile-time range checking
- ✅ No runtime overhead

**Trade-offs:**
- Bit manipulation for access (minimal cost)
- Alignment considerations

## 🎯 Best Practices

```nux
// ✅ Good: Use var for constrained ranges
var<0..100> percentage;
var<1..12> month;
var<0..23> hour;

// ❌ Bad: Use regular types for unconstrained values
var<-2147483648..2147483647> x;  // Just use int!

// ✅ Good: Pack related fields
struct Flags {
    var<0..1> is_active;
    var<0..1> is_verified;
    var<0..1> is_admin;
}

// ✅ Good: Use named types for clarity
type Age = var<0..120>;
type Grade = var<'A'..'F'>;
```

## 🌟 Why This is Revolutionary

**No other mainstream language has this feature!**

- C/C++: Fixed-size types only
- Java: Fixed-size primitives
- Python: Dynamic typing (wastes memory)
- Rust: Has ranges but not automatic bit-packing
- **Nux: Custom ranges + automatic optimization!**

This makes Nux **perfect for:**
- Embedded systems
- Game development
- Network protocols
- Database systems
- Memory-constrained environments

**Nux `var`: The most memory-efficient type system ever!** 🚀
