# Zig

Apparently, the .dll needs to be inside the zig consumer folder. if not, the zig program will run without printing anything

```bash
# Copy the .dll
copy /y ..\generated_libs\complexNumber\bin\complexNumbers.dll .

# Run
zig run src/main.zig --library complexNumbers -I. -L.
# or
zig run src/main.zig --library ../generated_libs/complexNumber/bin/complexNumbers -I. -L.
```

Output

```bash
C:\project-coding\cpp\202512\cpp-c-style-lib\complexNumberLib-consumer-zig>zig run src/main.zig --library ../generated_libs/complexNumber/bin/complexNumbers -I. -L.
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