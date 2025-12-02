#include <iostream>
#include "complexNumberLib/complexNumbers.h"

int main() {
    // Create complex numbers
    ComplexNumberHandle a = complex_create(2, 3);
    ComplexNumberHandle b = complex_create(4, 5);
    
    if (!a || !b) {
        std::cerr << "Failed to create complex numbers" << std::endl;
        return 1;
    }
    
    // Perform operations
    ComplexNumberHandle c = complex_add(a, b);
    ComplexNumberHandle d = complex_subtract(a, b);
    ComplexNumberHandle e = complex_multiply(a, b);
    // ComplexNumberHandle f = complex_div(a, b);
    
    // Print results
    const int buffer_size = 100;
    char buffer[buffer_size];
    
    if (c) {
        complex_to_string(c, buffer, buffer_size);
        std::cout << "a + b = " << buffer << std::endl;
    }
    
    if (d) {
        complex_to_string(d, buffer, buffer_size);
        std::cout << "a - b = " << buffer << std::endl;
    }
    
    if (e) {
        complex_to_string(e, buffer, buffer_size);
        std::cout << "a * b = " << buffer << std::endl;
    }
    
    // if (f) {
    //     complex_to_string(f, buffer, buffer_size);
    //     std::cout << "a / b = " << buffer << std::endl;
    // }
    
    // Demonstrate other operations
    std::cout << "\nAdditional operations:" << std::endl;
    
    // Get real and imaginary parts
    std::cout << "Real part of a: " << complex_get_real(a) << std::endl;
    std::cout << "Imaginary part of a: " << complex_get_imaginary(a) << std::endl;
    
    // Magnitude
    std::cout << "Magnitude of a: " << complex_magnitude(a) << std::endl;
    
    // Conjugate
    ComplexNumberHandle a_conj = complex_create(2, 3);  // Create a copy
    complex_conjugate(a_conj);
    complex_to_string(a_conj, buffer, buffer_size);
    std::cout << "Conjugate of a: " << buffer << std::endl;
    
    // Cleanup
    complex_destroy(a);
    complex_destroy(b);
    complex_destroy(c);
    complex_destroy(d);
    complex_destroy(e);
    // complex_destroy(f);
    complex_destroy(a_conj);
    
    return 0;
}