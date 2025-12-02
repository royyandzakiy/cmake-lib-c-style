const ffi = require('ffi-napi');
const ref = require('ref-napi');
const Struct = require('ref-struct-napi');

// Define types
const voidPtr = ref.refType(ref.types.void);
const double = ref.types.double;
const int = ref.types.int;
const charPtr = ref.refType(ref.types.char);

// Platform-specific library name
let libName;
switch (process.platform) {
    case 'win32':
        libName = 'complexNumbers.dll';
        break;
    case 'darwin':
        libName = 'libcomplexNumbers.dylib';
        break;
    default:
        libName = 'libcomplexNumbers.so';
}

// Load the library
const libpath = require('path').join(__dirname, libName);
const complexLib = ffi.Library(libpath, {
    // Creation and destruction
    'complex_create': [voidPtr, [double, double]],
    'complex_destroy': ['void', [voidPtr]],
    
    // Getters
    'complex_get_real': [double, [voidPtr]],
    'complex_get_imaginary': [double, [voidPtr]],
    
    // Setters
    'complex_set_real': ['void', [voidPtr, double]],
    'complex_set_imaginary': ['void', [voidPtr, double]],
    
    // Operations
    'complex_conjugate': ['void', [voidPtr]],
    'complex_magnitude': [double, [voidPtr]],
    
    // Arithmetic operations
    'complex_add': [voidPtr, [voidPtr, voidPtr]],
    'complex_subtract': [voidPtr, [voidPtr, voidPtr]],
    'complex_multiply': [voidPtr, [voidPtr, voidPtr]],
    'complex_divide': [voidPtr, [voidPtr, voidPtr]],
    
    // String representation
    'complex_to_string': ['void', [voidPtr, charPtr, int]]
});

// Helper function for string conversion
function complexToString(handle) {
    const bufferSize = 100;
    const buffer = Buffer.alloc(bufferSize);
    complexLib.complex_to_string.async(handle, buffer, bufferSize, (err) => {
        if (err) throw err;
    });
    return buffer.toString('utf8').replace(/\0/g, '');
}

// Main function
async function main() {
    const BUFFER_SIZE = 100;
    
    try {
        // Create complex numbers
        const a = complexLib.complex_create(2, 3);
        const b = complexLib.complex_create(4, 5);
        
        if (a.isNull() || b.isNull()) {
            console.error("Failed to create complex numbers");
            return;
        }
        
        // Perform operations
        const c = complexLib.complex_add(a, b);
        const d = complexLib.complex_subtract(a, b);
        const e = complexLib.complex_multiply(a, b);
        const f = complexLib.complex_divide(a, b);
        
        // Print results
        console.log("a + b =", complexToString(c));
        console.log("a - b =", complexToString(d));
        console.log("a * b =", complexToString(e));
        console.log("a / b =", complexToString(f));
        
        // Demonstrate other operations
        console.log("\nAdditional operations:");
        
        // Get real and imaginary parts
        console.log("Real part of a:", complexLib.complex_get_real(a));
        console.log("Imaginary part of a:", complexLib.complex_get_imaginary(a));
        
        // Magnitude
        console.log("Magnitude of a:", complexLib.complex_magnitude(a));
        
        // Set new values
        complexLib.complex_set_real(a, 10);
        complexLib.complex_set_imaginary(a, 20);
        console.log("After setting new values:");
        console.log("Real part of a:", complexLib.complex_get_real(a));
        console.log("Imaginary part of a:", complexLib.complex_get_imaginary(a));
        
        // Conjugate
        const aConj = complexLib.complex_create(2, 3);
        complexLib.complex_conjugate(aConj);
        console.log("Conjugate of (2, 3):", complexToString(aConj));
        
        // Cleanup
        complexLib.complex_destroy(a);
        complexLib.complex_destroy(b);
        if (!c.isNull()) complexLib.complex_destroy(c);
        if (!d.isNull()) complexLib.complex_destroy(d);
        if (!e.isNull()) complexLib.complex_destroy(e);
        if (!f.isNull()) complexLib.complex_destroy(f);
        if (!aConj.isNull()) complexLib.complex_destroy(aConj);
        
    } catch (error) {
        console.error("Error:", error);
    }
}

// Run the program
main();