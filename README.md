# cmake-install-custom-lib: ComplexNumberLib

A CMake-based C++ library project demonstrating how to create, install, and consume custom libraries with both static and shared library configurations.

## Project Structure

```
complexNumberLib/
├── complexNumberLib-static-shared/  # Library source and build system
│   ├── cmake/
│   │   ├── complexNumbersConfig.cmake.in
│   │   └── copy-dlls.cmake          # Reusable DLL copy script
├── complexNumberLib-consumer/       # Example consumer application
├── generated_libs/                  # Installation directory for built libraries
└── README.md
```

## Quick Start

### Build and Install the Library

```bash
# Build and install shared library (Release)
cd complexNumberLib-static-shared
cmake -B build -DCMAKE_BUILD_TYPE=Release -DENABLE_SHARED=ON
cmake --build build --config Release
cmake --install build --prefix "../generated_libs/complexNumber" --config Release
```

### Build the Consumer Application

```bash
cd complexNumberLib-consumer
cmake -B build -DCMAKE_PREFIX_PATH="../generated_libs/complexNumber" -DCMAKE_BUILD_TYPE=Release
cmake --build build --config Release
```

## Features

- **Dual Library Support**: Build as either static or shared library using `-DENABLE_SHARED=ON/OFF`
- **Multi-configuration**: Supports both Debug and Release builds
- **Proper Installation**: Libraries, headers, and CMake config files installed to structured directories
- **Reusable DLL Management**: Automatic DLL copying for Windows shared library consumers using a reusable script
- **CMake Package Configuration**: Generated config files enable easy consumption via `find_package()`

## Building All Configurations

```bash
# One-liner to build all configurations
cd complexNumberLib-static-shared
cmake -B build -DCMAKE_BUILD_TYPE=Release -DENABLE_SHARED=OFF && cmake --build build --config Release && cmake --install build --prefix "../generated_libs/complexNumber" --config Release
# Repeat for other configurations as needed...
```

## Key Learning Points

This project demonstrates:
1. Creating export/import macros for cross-platform DLL/SO symbol visibility
2. Installing libraries with proper CMake package configuration
3. Consuming installed libraries via `find_package()`
4. **Reusable automatic DLL management** for Windows shared libraries
5. Multi-configuration (Debug/Release) support

## Reusable DLL Copy Script

The project includes a reusable CMake script for automatically copying DLL files on Windows:

### `copy-dlls.cmake` - Usage in Consumer Projects

Save this script in your project's `cmake/` directory and use it like this:

```cmake
# 1. Include the script in your CMakeLists.txt
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")
include(copy-dlls)

# 2. After linking with a shared library, call the function
copy_dlls_for_target(
    TARGET ${PROJECT_NAME}          # Your executable target
    LIBRARY_TARGET library::target  # Imported library target (from find_package)
    DLL_NAME "mylibrary"            # Optional: custom DLL base name
    DEBUG_POSTFIX "d"               # Optional: debug postfix (default: "d")
)
```

### Full Example for Different Projects

```cmake
# In your consumer project's CMakeLists.txt
cmake_minimum_required(VERSION 3.23)

project(MyApp)
list(APPEND CMAKE_MODULE_PATH "${CMAKE_CURRENT_SOURCE_DIR}/cmake")
include(copy-dlls)

# Find and use any installed library
find_package(SomeLibrary REQUIRED)
find_package(AnotherLibrary REQUIRED)

add_executable(${PROJECT_NAME} main.cpp)

# Link with libraries
target_link_libraries(${PROJECT_NAME} 
    PRIVATE 
        SomeLibrary::SomeLibrary
        AnotherLibrary::AnotherLibrary
)

# Automatically copy DLLs for both libraries
copy_dlls_for_target(
    TARGET ${PROJECT_NAME}
    LIBRARY_TARGET SomeLibrary::SomeLibrary
)

copy_dlls_for_target(
    TARGET ${PROJECT_NAME}
    LIBRARY_TARGET AnotherLibrary::AnotherLibrary
    DLL_NAME "another"        # If DLL has different base name
    DEBUG_POSTFIX "_debug"    # If using custom debug postfix
)
```

### Script Features

- **Automatic detection**: Identifies shared libraries automatically
- **Multi-configuration support**: Handles Debug/Release builds correctly
- **Customizable**: Optional parameters for different DLL naming conventions
- **Cross-project compatible**: Works with any imported CMake target
- **Informative**: Provides clear status messages

## Usage in Other Projects

### 1. Consuming the ComplexNumbers Library
After installation, consume the library in your CMake projects:

```cmake
find_package(complexNumbers REQUIRED)
target_link_libraries(your_target PRIVATE complexNumbers::complexNumbers)
```

### 2. Using the Reusable DLL Script
Copy the `copy-dlls.cmake` script to your project and include it:

```bash
# Copy the reusable script to your project
cp path/to/complexNumberLib/static-shared/cmake/copy-dlls.cmake your-project/cmake/
```

Then use it for any shared library dependency in your project.

## Troubleshooting

If DLL copying doesn't work:
1. Check that the library is actually built as shared (`-DENABLE_SHARED=ON`)
2. Verify the library target name is correct
3. Check debug messages for DLL path information
4. Ensure `find_package()` successfully found the library

## Detailed Build & Install cmake calls

```bash
cd complexNumberLib-c-style

 && cmake -B build -DCMAKE_BUILD_TYPE=Release -DENABLE_SHARED=ON
 && cmake --build build --config Release
 && cmake --install build --prefix "..\generated_libs\complexNumber" --config Release
```

Debug type
```bash
cmake -B build -DCMAKE_BUILD_TYPE=Debug -DENABLE_SHARED=ON
 && cmake --build build --config Debug
 && cmake --install build --prefix "..\generated_libs\complexNumber" --config Debug
```

### Consumer Build

```bash
cd complexNumberLib-c-style-consumer

cmake -B build -DCMAKE_PREFIX_PATH="../generated_libs/complexNumber" -DCMAKE_BUILD_TYPE=Release
 && cmake --build build --config Release
```

### All Build

One Liner
```bash
cd complexNumberLib-c-style

 && cmake -B build -DCMAKE_BUILD_TYPE=Release -DENABLE_SHARED=OFF
 && cmake --build build --config Release
 && cmake --install build --prefix "..\generated_libs\complexNumber" --config Release

 && cmake -B build -DCMAKE_BUILD_TYPE=Debug -DENABLE_SHARED=OFF
 && cmake --build build --config Debug
 && cmake --install build --prefix "..\generated_libs\complexNumber" --config Debug

 && cmake -B build -DCMAKE_BUILD_TYPE=Release -DENABLE_SHARED=ON
 && cmake --build build --config Release
 && cmake --install build --prefix "..\generated_libs\complexNumber" --config Release

 && cmake -B build -DCMAKE_BUILD_TYPE=Debug -DENABLE_SHARED=ON
 && cmake --build build --config Debug
 && cmake --install build --prefix "..\generated_libs\complexNumber" --config Debug

 && cd .. && cd complexNumberLib-c-style-consumer

 && cmake -B build -DCMAKE_PREFIX_PATH="../generated_libs/complexNumber" -DCMAKE_BUILD_TYPE=Release
 && cmake --build build --config Release

 && cmake -B build -DCMAKE_PREFIX_PATH="../generated_libs/complexNumber" -DCMAKE_BUILD_TYPE=Debug
 && cmake --build build --config Debug
```

reference: https://cfd.university/learn/automating-cfd-solver-and-library-compilation-using-cmake/how-to-compile-install-and-use-custom-libraries-with-cmake/