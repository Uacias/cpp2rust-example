# C++ → Rust FFI Minimal Example (using `.so` dynamic library)

End-to-end example of calling a C++ function from Rust using a shared `.so` library.

---

## 🗂️ Repository Structure

```

├── cpp
│ ├── include
│ │ └── native_lib.h # C++ header exposing extern "C" function
│ └── src
│ └── native_lib.cpp # C++ implementation (add(a, b))
├── rust
│ ├── build.rs # Rust build script linking against libnative_lib.so
│ ├── Cargo.toml # Rust project manifest
│ └── src
│ └── main.rs # Rust code calling the C++ function
└── readme.md # This file
```

---

## ✅ Prerequisites

- **Rust** toolchain (1.56+)
- **G++** or compatible C++ compiler (with C++17 or newer)
- Linux or Unix-based system with `LD_LIBRARY_PATH` support

---

## ⚙️ How It Works

### 1. C++ Side

- **`native_lib.cpp`** defines a basic function:

  ```cpp
  int add(int a, int b) {
      return a + b;
  }
  ```

- `native_lib.h` exposes it as:

```cpp
extern "C" {
    int add(int a, int b);
}
```

### 2. Build the Shared Library

From the root of the project, run:

```bash
g++ -fPIC -shared -Icpp/include -o rust/libnative_lib.so cpp/src/native_lib.cpp
```

This generates `libnative_lib.so` inside the `rust/` folder.

### 3. Rust Side

- `main.rs` declares the C function:

```rust
extern "C" {
    fn add(a: i32, b: i32) -> i32;
}
```

- `build.rs` tells Cargo to link against the `.so` file:

```
fn main() {
    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-lib=dylib=native_lib");
}
```

Note: `cargo:rustc-link-search=.` means the .so must be in the rust/ directory.

## 🚀 Build & Run

```bash
# Step into the Rust directory
cd rust

# Run with the LD_LIBRARY_PATH pointing to current directory:
LD_LIBRARY_PATH=. cargo run
```

You should see:

```bash
2 + 3 = 5
```
