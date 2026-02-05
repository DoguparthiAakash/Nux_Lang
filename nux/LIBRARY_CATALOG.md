# Nux Library Catalog
**Complete Reference Guide - 79 Libraries**

## 📚 Standard Libraries (`lib/std/`) - 47 Libraries

### Core Utilities
- **`io.nux`** - Input/Output wrappers
- **`math.nux`** - Mathematical functions
- **`string.nux`** - String manipulation
- **`collections.nux`** - List and Map data structures
- **`datetime.nux`** - Date and time utilities

### File & Data
- **`file.nux`** - File system operations
- **`oo_file.nux`** - Object-oriented File I/O
- **`json.nux`** - JSON parsing and serialization
- **`config.nux`** - Configuration file management (INI, TOML)
- **`serialization.nux`** - Binary serialization (MessagePack)
- **`database.nux`** - In-memory database with B-Tree indexing

### Graphics & Multimedia
- **`graphics.nux`** - Image & Vision wrappers
- **`oo_graphics.nux`** - OO Graphics (Image class)
- **`gfxdriver.nux`** - VGA text mode, framebuffer, VBE
- **`audio.nux`** - AC97 and Intel HD Audio drivers
- **`multimedia.nux`** - Audio/video encoding, H.264, MP4
- **`arvr.nux`** - AR/VR, 3D graphics, spatial audio
- **`gamedev.nux`** - 2D physics, sprites, particles

### Game Development 🎮 (NEW)
- **`gameengine.nux`** - ECS (Entity-Component-System), input, audio, resources, scenes
- **`physics2d.nux`** - 2D physics engine (Box2D-like): rigid bodies, shapes, collision, joints
- **`physics3d.nux`** - 3D physics: Vec3, Quaternion, GJK collision, 3D rigid bodies
- **`animation.nux`** - Tweening, keyframes, skeletal animation, state machines, timelines
- **`pathfinding.nux`** - A*, Dijkstra, steering behaviors (seek, flee, flocking), path following
- **`tilemap.nux`** - Tile-based levels, layers, parallax scrolling, autotiling

### Networking & Web
- **`network.nux`** - Socket operations and HTTP
- **`netstack.nux`** - Full TCP/IP stack (Ethernet, IPv4, TCP, UDP)
- **`webframework.nux`** - HTTP server, routing, sessions
- **`iot.nux`** - MQTT, CoAP, LoRaWAN protocols
- **`cloud.nux`** - Load balancing, distributed cache, MapReduce

### Hardware & System
- **`memory.nux`** - Direct memory access, DMA, cache control
- **`cpu.nux`** - CPUID, MSR, control registers, I/O ports
- **`atomic.nux`** - Lock-free operations, spinlocks
- **`interrupt.nux`** - IDT, PIC/APIC, IRQ handling
- **`pci.nux`** - PCI/PCIe device enumeration
- **`usb.nux`** - XHCI controller, USB HID
- **`disk.nux`** - ATA, AHCI, NVMe drivers
- **`filesystem.nux`** - Ext4 and FAT32 drivers
- **`acpi.nux`** - ACPI tables, power management
- **`system.nux`** - OS interaction, environment, processes

### Concurrency & Threading
- **`thread.nux`** - Thread management, Mutex, Semaphore, RWLock
- **`simd.nux`** - SSE/AVX vector operations

### Security & Cryptography
- **`crypto.nux`** - Hashing, AES encryption, Base64, UUID
- **`security.nux`** - TLS/SSL, OAuth 2.0, JWT, TOTP
- **`blockchain.nux`** - Proof-of-work, transactions, Merkle trees

### Scientific & Engineering
- **`dsp.nux`** - FFT, FIR/IIR filters, spectrograms
- **`scientific.nux`** - Numerical integration, ODE solvers, linear algebra
- **`robotics.nux`** - PID control, Kalman filter, A* planning, SLAM
- **`bioinformatics.nux`** - DNA/RNA operations, sequence alignment

### Development Tools
- **`compiler.nux`** - Lexer, parser, AST, code generation, JIT
- **`testing.nux`** - Test suites, assertions, mocking, benchmarks
- **`logging.nux`** - Structured logging, metrics, tracing
- **`cli.nux`** - Argument parsing, progress bars, tables
- **`regex.nux`** - Regular expression engine

### Finance & Business
- **`finance.nux`** - Time series, trading strategies, Black-Scholes, VaR

### Miscellaneous
- **`random.nux`** - Random number generation (LCG, Xorshift)

---

## 🤖 AI/ML Libraries (`lib/ai/`) - 15 Libraries

### Core ML
- **`tensor.nux`** - Multi-dimensional arrays, matrix operations
- **`neuralnet.nux`** - Dense, Conv2D, pooling, dropout, batch norm
- **`optimizer.nux`** - SGD, Adam, RMSprop, learning rate schedulers
- **`loss.nux`** - MSE, cross-entropy, hinge, focal, contrastive

### Applications
- **`vision.nux`** - Image processing, ResNet, YOLO, augmentation
- **`nlp.nux`** - Tokenization, embeddings, LSTM, transformers
- **`rl.nux`** - Reinforcement learning, DQN, replay buffer

### Advanced Deep Learning 🧠 (NEW)
- **`autograd.nux`** - Automatic differentiation, computation graph, backward pass, optimizers (SGD, Adam)
- **`transformer.nux`** - Multi-head attention, positional encoding (sinusoidal, RoPE), GPT with generation
- **`gan.nux`** - Generators (Linear, DCGAN), discriminators, BCE/Wasserstein/hinge loss, WGAN-GP
- **`diffusion.nux`** - Noise scheduling, U-Net, DDPM/DDIM samplers, text/class conditioning
- **`agents.nux`** - Memory systems, tool registry, RAG (vector store), ReAct/CoT/ToT planning
- **`rl_advanced.nux`** - Prioritized replay, Double/Dueling DQN, PPO, SAC, TD3, MBPO

### Scientific
- **`datascience.nux`** - Statistics, regression, K-means, PCA
- **`quantum.nux`** - Quantum state simulation, gates, Grover's algorithm

---

## 🖥️ Operating System Libraries (`lib/os/`) - 6 Libraries (NEW)

### Kernel & System
- **`kernel.nux`** - IDT/interrupt handling, paging, frame allocator, spinlock/mutex/semaphore, syscalls
- **`scheduler.nux`** - PCB, scheduling algorithms (Round Robin, Priority, MLFQ, CFS), process/thread managers
- **`memman.nux`** - Heap allocators (bump, first-fit, buddy, slab), VMA, mmap/munmap, page fault handling

### Hardware & I/O
- **`driver.nux`** - Device abstraction, block/char devices, IRQ handling, PCI bus, DMA, example drivers
- **`vfs.nux`** - Inodes, dentries, file operations, superblocks, mount/unmount, ramfs implementation
- **`bootloader.nux`** - Multiboot parsing, GDT setup, A20 line, protected/long mode switching, VGA console

---

## ⚡ Embedded/IoT Libraries (`lib/embedded/`) - 3 Libraries

- **`gpio.nux`** - GPIO control
- **`time.nux`** - Timing functions
- **`analog.nux`** - Analog I/O

---

## 📊 Library Statistics

**Total Libraries:** 61
- Standard: 41
- AI/ML: 9  
- Embedded: 3
- Examples: 8+

**Lines of Code:** ~15,000+
**Coverage Areas:** 20+ domains

---

## 🎯 Domain Coverage

### Systems Programming
✅ OS Development, Kernel, Drivers, Hardware Control

### Artificial Intelligence
✅ Deep Learning, Computer Vision, NLP, Reinforcement Learning

### Web & Cloud
✅ HTTP Servers, Distributed Systems, Microservices

### IoT & Embedded
✅ MQTT, Edge Computing, Sensor Networks

### Finance & Trading
✅ Algorithmic Trading, Risk Analysis, Portfolio Management

### Scientific Computing
✅ Numerical Methods, Simulations, Data Analysis

### Security
✅ Cryptography, Authentication, Blockchain

### Multimedia
✅ Audio/Video Processing, 3D Graphics, AR/VR

### Development
✅ Testing, Logging, CLI Tools, Compilers

---

## 🚀 Quick Start Examples

### Web Server
```nux
import "std.webframework";

var server = new WebServer();
server.init(8080);

server.router.get("/", func(req) {
    var res = new HTTPResponse();
    res.body = "Hello, World!";
    return res;
});

server.listen();
```

### Machine Learning
```nux
import "ai.neuralnet";
import "ai.optimizer";

var model = new DenseLayer();
model.init(784, 10, 1);

var optimizer = new Adam();
optimizer.init(0.001, 0.9, 0.999, 1e-8);
```

### Hardware Control
```nux
import "std.cpu";
import "std.memory";

var tsc = rdtsc();
var cr3 = read_cr3();

var mem = mem_alloc_aligned(4096, 4096);
mem_write64(mem, 0xDEADBEEF);
```

---

**Nux: From Bare Metal to AI** 🔥
