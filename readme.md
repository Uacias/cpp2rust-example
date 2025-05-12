# C++ → Rust FFI Minimal Example

End-to-end example of calling a C++ function from Rust using a `cc` build script.

---

## Repository Structure

```
├── cpp
│ ├── include
│ │ └── native_lib.h # C++ header for JNI-style function
│ └── src
│ └── native_lib.cpp # C++ implementation (add(a, b))
├── rust
│ ├── build.rs # Rust build script (invokes cc::Build)
│ ├── Cargo.toml # Rust project manifest
│ └── src
│ └── main.rs # Rust code calling the C++ add function
└── .gitignore # Ignored build artifacts
```

---

## Prerequisites

- **Rust** toolchain (1.56+)
- **C++** compiler (GCC/Clang with C++17 support)
- **CMake** (for the C++ side, optional if you use your own build)

---

## How It Works

1. **C++ Side**

   - `native_lib.cpp` defines:
     ```cpp
     int add(int a, int b) { return a + b; }
     ```
   - Exposed via `extern "C"` in `native_lib.h`.

2. **Rust Side**
   - `build.rs` uses the [`cc`](https://crates.io/crates/cc) crate to compile the C++ code into a static library.
   - `main.rs` declares the foreign function:
     ```rust
     unsafe extern "C" { fn add(a: i32, b: i32) -> i32; }
     ```
   - Calls it inside an `unsafe {}` block and prints the result.

---

## Building & Running

```bash
# 1. Build the C++ library via Rust's build script:
cd rust
cargo clean
cargo run
You should see:

`5 + 7 = 12`

```
