# Rust

Build & Run

```bash
# Debug
cargo build
 && copy /y ..\generated_libs\complexNumber\bin\complexNumbers.dll target\debug
 && cargo run

# Release
cargo build -r
 && copy /y ..\generated_libs\complexNumber\bin\complexNumbers.dll target\release
 && cargo run -r
```

Output

```bash
C:\project-coding\cpp\202512\cpp-c-style-lib\complexNumberLib-consumer-rust>cargo build -r && copy /y ..\generated_libs\complexNumber\bin\complexNumbers.dll target\release && cargo run -r
    Finished `release` profile [optimized] target(s) in 0.01s
        1 file(s) copied.
    Finished `release` profile [optimized] target(s) in 0.01s
     Running `target\release\complex-test.exe`
a + b = (6, 8)
a - b = (-2, -2)
a * b = (-7, 22)
a / b = (0.560976, 0.0487805)

Additional operations:
Real part of a: 2
Imaginary part of a: 3
Magnitude of a: 3.605551275463989
After setting new values:
Real part of a: 10
Imaginary part of a: 20
Conjugate of (2, 3): (2, -3)
```