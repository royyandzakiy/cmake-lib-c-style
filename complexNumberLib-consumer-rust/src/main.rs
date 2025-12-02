use std::ffi::CStr;
use std::os::raw::c_char;
use complex_test::*;

const BUFFER_SIZE: usize = 100;

fn main() {
    unsafe {
        // Create complex numbers
        let a = complex_create(2.0, 3.0);
        let b = complex_create(4.0, 5.0);
        
        if a.is_null() || b.is_null() {
            eprintln!("Failed to create complex numbers");
            return;
        }
        
        // Perform operations
        let c = complex_add(a, b);
        let d = complex_subtract(a, b);
        let e = complex_multiply(a, b);
        let f = complex_divide(a, b);
        
        // Print results
        let mut buffer: [c_char; BUFFER_SIZE] = [0; BUFFER_SIZE];
        
        if !c.is_null() {
            complex_to_string(c, buffer.as_mut_ptr(), BUFFER_SIZE as i32);
            let str = CStr::from_ptr(buffer.as_ptr()).to_string_lossy();
            println!("a + b = {}", str);
        }
        
        if !d.is_null() {
            complex_to_string(d, buffer.as_mut_ptr(), BUFFER_SIZE as i32);
            let str = CStr::from_ptr(buffer.as_ptr()).to_string_lossy();
            println!("a - b = {}", str);
        }
        
        if !e.is_null() {
            complex_to_string(e, buffer.as_mut_ptr(), BUFFER_SIZE as i32);
            let str = CStr::from_ptr(buffer.as_ptr()).to_string_lossy();
            println!("a * b = {}", str);
        }
        
        if !f.is_null() {
            complex_to_string(f, buffer.as_mut_ptr(), BUFFER_SIZE as i32);
            let str = CStr::from_ptr(buffer.as_ptr()).to_string_lossy();
            println!("a / b = {}", str);
        }
        
        // Demonstrate other operations
        println!("\nAdditional operations:");
        
        // Get real and imaginary parts
        println!("Real part of a: {}", complex_get_real(a));
        println!("Imaginary part of a: {}", complex_get_imaginary(a));
        
        // Magnitude
        println!("Magnitude of a: {}", complex_magnitude(a));
        
        // Set new values
        complex_set_real(a, 10.0);
        complex_set_imaginary(a, 20.0);
        println!("After setting new values:");
        println!("Real part of a: {}", complex_get_real(a));
        println!("Imaginary part of a: {}", complex_get_imaginary(a));
        
        // Conjugate
        let a_conj = complex_create(2.0, 3.0);
        if !a_conj.is_null() {
            complex_conjugate(a_conj);
            complex_to_string(a_conj, buffer.as_mut_ptr(), BUFFER_SIZE as i32);
            let str = CStr::from_ptr(buffer.as_ptr()).to_string_lossy();
            println!("Conjugate of (2, 3): {}", str);
            complex_destroy(a_conj);
        }
        
        // Cleanup
        complex_destroy(a);
        complex_destroy(b);
        if !c.is_null() { complex_destroy(c); }
        if !d.is_null() { complex_destroy(d); }
        if !e.is_null() { complex_destroy(e); }
        if !f.is_null() { complex_destroy(f); }
    }
}