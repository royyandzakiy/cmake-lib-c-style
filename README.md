# ComplexNumberLib: C++ Library with C-Style Wrapper

A demonstration project showcasing how to create a C++ library with a C-style wrapper interface for maximum language interoperability. This project illustrates the benefits of separating implementation from interface while maintaining a stable ABI across different programming languages.

## Project Overview

This project implements a complex number library in C++ with a clean C-style wrapper interface. The core C++ implementation uses modern features internally, while the public API is exposed through a stable C interface. This approach enables seamless interoperability with multiple programming languages while maintaining the performance benefits of C++.

## Key Architecture

**Separation of Concerns:**
- `complexNumbers.h`: Clean C++ class implementation (no export macros)
- `complexNumbersImpl.hpp`: Implementation details
- `complexWrapper.hpp`: C-style interface with stable ABI and export macros
- `complexWrapper.cpp`: C-to-C++ bridge implementation

**Key Features:**
- Opaque handle type (`ComplexNumberHandle`) for memory safety
- `extern "C"` linkage for maximum language compatibility
- Platform-specific export macros in wrapper only
- Explicit memory management interface
- Buffer-safe string operations

## Project Structure

```
cpp-c-style-lib/
├── complexNumberLib/                    # Library source
│   ├── complexNumberLib/               # Core implementation
│   │   ├── complexNumbers.h           # Clean C++ interface
│   │   ├── complexNumbersImpl.hpp     # Implementation details
│   │   └── complexNumbers.cpp         # C++ implementation
│   └── CMakeLists.txt                 # Build configuration
├── generated_libs/                     # Installed library files
│   └── complexNumber/
│       ├── include/                   # Public headers
│       ├── lib/                       # Library files
│       └── bin/                       # DLL files (Windows)
├── complexNumberLib-consumer-*/       # Language-specific consumers
│   ├── C                             # C consumer
│   ├── C++                           # C++ consumer
│   ├── Python                        # Python consumer (ctypes)
│   ├── Rust                          # Rust consumer (FFI)
│   └── Zig                           # Zig consumer (C interop)
└── README.md
```

## Why C-Style Wrappers?

### Stable ABI (Application Binary Interface)
C-style interfaces provide a stable binary interface that remains compatible across compiler versions and platforms. Unlike C++, which has name mangling and can break ABI between compiler versions, C maintains a simple, stable calling convention.

### Language Interoperability
Most programming languages have well-established mechanisms for calling C functions, but C++ interoperability is often more complex or limited. By providing a C interface, the library becomes accessible from:

- **C** (native support)
- **C++** (native support with `extern "C"`)
- **Python** (via ctypes or CFFI)
- **Rust** (via FFI bindings)
- **Zig** (via C interop)
- **Go** (via cgo)
- **Java** (via JNI)
- **C#** (via P/Invoke)
- **JavaScript** (via Node.js native addons)

### Implementation Freedom
The C wrapper allows the underlying C++ implementation to use advanced features (templates, inheritance, exceptions, etc.) without exposing these complexities to consumers. The internal implementation can evolve independently as long as the C interface remains stable.

## Building the Library

```bash
# Configure with shared library support
cd complexNumberLib
cmake -B build -DCMAKE_BUILD_TYPE=Release -DENABLE_SHARED=ON
cmake --build build --config Release
cmake --install build --prefix "../generated_libs/complexNumber"
```

## Technical Benefits

1. **ABI Stability**: The C interface remains compatible across compiler updates
2. **Binary Compatibility**: Library versions can be upgraded without recompiling consumers
3. **Exception Safety**: C wrapper catches C++ exceptions and converts to error codes
4. **Memory Safety**: Opaque handles prevent direct memory manipulation
5. **Thread Safety**: Can implement thread-safe operations in wrapper
6. **Versioning**: Easy to maintain backward compatibility