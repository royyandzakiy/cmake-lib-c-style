# C++

```bash
cmake -B build -DCMAKE_PREFIX_PATH="../generated_libs/complexNumber" -DCMAKE_BUILD_TYPE=Release
 && cmake --build build --config Release
```

Output

```bash
C:\project-coding\cpp\202512\cpp-c-style-lib\complexNumberLib-consumer-cpp>C:\project-coding\cpp\202512\cpp-c-style-lib\complexNumberLib-consumer-cpp\build\Release\complexNumberTest.exe
a + b = (6, 8)
a - b = (-2, -2)
a * b = (-7, 22)

Additional operations:
Real part of a: 2
Imaginary part of a: 3
Magnitude of a: 3.60555
Conjugate of a: (2, -3)
```