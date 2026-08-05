# Nux User Guide

Welcome to Nux! Because Nux is designed as a "Write Once, Run Anywhere" language, the syntax and core concepts are exactly the same whether you are on Windows, Mac, Linux, or BSD.

## Writing Your First Program

Create a file called `hello.nux`:

```nux
func main() {
    println("Hello, World from Nux!");
}
```

## Running Nux

To execute a `.nux` file, use the Nux compiler/runner from your terminal:

```sh
nux run hello.nux
```

## Compiling to Executable

To compile your code into a standalone binary:

```sh
nux build hello.nux
```

## Core Libraries

Nux comes with a powerful standard library. Simply import what you need:

```nux
import "std/math";
import "std/io";

func main() {
    var x = math_sqrt(16.0);
    println(x);
}
```

Enjoy building with Nux!
