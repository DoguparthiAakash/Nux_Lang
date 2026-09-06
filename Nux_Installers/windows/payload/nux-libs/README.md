# Nux Domain-Specific Libraries

High-level Nux libraries for different domains, using the C++ libraries through FFI.

## 📁 Structure

```
nux-libs/
├── data/           # Data science
│   └── dataframe.nux
├── ai/             # AI/ML
│   └── neural.nux
├── gui/            # GUI applications
│   └── window.nux
├── os/             # System programming
│   └── system.nux
├── blockchain/     # Blockchain & crypto
│   └── chain.nux
├── quantum/        # Quantum computing
│   └── circuit.nux
├── crypto/         # Cryptography
│   └── hash.nux
└── web/            # Web development
    └── server.nux
```

## 🎯 Purpose

These libraries provide **idiomatic Nux interfaces** to the C++ libraries, making them easy to use in Nux applications.

## 💡 Examples

### Data Science
```nux
import "nux-libs/data/dataframe";

var df = DataFrame.from_csv("data.csv");
df.head().print();
df.plot("x", "y", "scatter");
```

### AI/ML
```nux
import "nux-libs/ai/neural";

var model = new NeuralNetwork();
model.add(new Linear(784, 128));
model.add(new ReLU());
model.compile("adam", 0.001);
model.train(X, y, epochs=10);
```

### GUI
```nux
import "nux-libs/gui/window";

var window = new Window("My App", 800, 600);
var button = new Button("Click me", 10, 10, 100, 30);
button.on_click(func() {
    println("Clicked!");
});
window.add_widget(button);
window.run();
```

### System Programming
```nux
import "nux-libs/os/system";

var server = new Socket("tcp");
server.bind("0.0.0.0", 8080);
server.listen(10);

while (true) {
    var client = server.accept();
    Thread.spawn(func() {
        handle_client(client);
    });
}
```

### Blockchain
```nux
import "nux-libs/blockchain/chain";

var chain = new Blockchain(4);
var wallet = new Wallet();

chain.add_transaction(wallet.get_address(), "Bob", 50);
chain.mine_pending_transactions(wallet.get_address());
```

### Quantum Computing
```nux
import "nux-libs/quantum/circuit";

var circuit = new QuantumCircuit(2);
circuit.h(0);
circuit.cnot(0, 1);
var state = circuit.execute();
```

## 🔧 FFI Integration

All libraries use FFI to call C++ functions:

```nux
// Nux code
var result = ffi_call("nux_array_add", handle1, handle2);

// Maps to C++ function
extern "C" void* nux_array_add(void* a, void* b) {
    auto* arr_a = static_cast<Tensor*>(a);
    auto* arr_b = static_cast<Tensor*>(b);
    auto* result = new Tensor(*arr_a + *arr_b);
    return result;
}
```

## 📚 Complete Domain Coverage

| Domain | Library | Use Cases |
|--------|---------|-----------|
| Data Science | data/ | Data analysis, visualization |
| AI/ML | ai/ | Neural networks, ML models |
| GUI | gui/ | Desktop applications |
| System | os/ | Servers, file systems, networking |
| Blockchain | blockchain/ | Cryptocurrencies, NFTs, DeFi |
| Quantum | quantum/ | Quantum algorithms, cryptography |
| Crypto | crypto/ | Encryption, hashing, signatures |
| Web | web/ | Web servers, APIs |

## 🚀 Benefits

1. **High-level API** - Easy to use, Nux-idiomatic
2. **Performance** - C++ backend for speed
3. **Safety** - Nux's safety features + C++ validation
4. **Productivity** - Write less code, do more
5. **Interoperability** - Seamless C++ integration

## 🎯 Usage

```bash
# Run data science script
nux run examples/data_analysis.nux

# Run AI training
nux run examples/train_model.nux

# Run GUI app
nux run examples/calculator.nux

# Run blockchain node
nux run examples/crypto_node.nux
```
