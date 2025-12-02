# Zig

Currently, the .dll needs to be inside the zig consumer folder. if not, the zig program will run without printing anything

```bash
# Copy the .dll
copy /y ..\generated_libs\complexNumber\bin\complexNumbers.dll .

# Run
zig run src/main.zig --library complexNumbers -I. -L.
# or
zig run src/main.zig --library ../generated_libs/complexNumber/bin/complexNumbers -I. -L.
```