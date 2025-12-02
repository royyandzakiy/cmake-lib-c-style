use libc::{c_double, c_int, c_void};

#[repr(C)]
pub struct ComplexNumberHandle(c_void);

extern "C" {
    // Creation and destruction
    pub fn complex_create(real: c_double, imaginary: c_double) -> *mut ComplexNumberHandle;
    pub fn complex_destroy(handle: *mut ComplexNumberHandle);
    
    // Getters
    pub fn complex_get_real(handle: *const ComplexNumberHandle) -> c_double;
    pub fn complex_get_imaginary(handle: *const ComplexNumberHandle) -> c_double;
    
    // Setters
    pub fn complex_set_real(handle: *mut ComplexNumberHandle, real: c_double);
    pub fn complex_set_imaginary(handle: *mut ComplexNumberHandle, imaginary: c_double);
    
    // Operations
    pub fn complex_conjugate(handle: *mut ComplexNumberHandle);
    pub fn complex_magnitude(handle: *const ComplexNumberHandle) -> c_double;
    
    // Arithmetic operations
    pub fn complex_add(a: *const ComplexNumberHandle, b: *const ComplexNumberHandle) -> *mut ComplexNumberHandle;
    pub fn complex_subtract(a: *const ComplexNumberHandle, b: *const ComplexNumberHandle) -> *mut ComplexNumberHandle;
    pub fn complex_multiply(a: *const ComplexNumberHandle, b: *const ComplexNumberHandle) -> *mut ComplexNumberHandle;
    pub fn complex_divide(a: *const ComplexNumberHandle, b: *const ComplexNumberHandle) -> *mut ComplexNumberHandle;
    
    // String representation
    pub fn complex_to_string(handle: *const ComplexNumberHandle, buffer: *mut libc::c_char, buffer_size: c_int);
}