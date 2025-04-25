/// A trait that defines a method to retrieve the corresponding OpenGL type for a given Rust type.
pub trait GLType {
    /// Returns the OpenGL constant representing the type of the implementing Rust type.
    ///
    /// # Returns
    /// * The OpenGL constant (as a `u32`) corresponding to the type.
    fn gl_type() -> u32;
}

/// Implementation of the `GLType` trait for `f32`.
/// Maps the Rust `f32` type to the OpenGL `FLOAT` constant.
impl GLType for f32 {
    fn gl_type() -> u32 {
        gl::FLOAT // OpenGL constant for a 32-bit floating point value
    }
}

/// Implementation of the `GLType` trait for `f64`.
/// Maps the Rust `f64` type to the OpenGL `DOUBLE` constant.
impl GLType for f64 {
    fn gl_type() -> u32 {
        gl::DOUBLE // OpenGL constant for a 64-bit floating point value
    }
}

/// Implementation of the `GLType` trait for `i32`.
/// Maps the Rust `i32` type to the OpenGL `INT` constant.
impl GLType for i32 {
    fn gl_type() -> u32 {
        gl::INT // OpenGL constant for a 32-bit signed integer value
    }
}

/// Implementation of the `GLType` trait for `u32`.
/// Maps the Rust `u32` type to the OpenGL `UNSIGNED_INT` constant.
impl GLType for u32 {
    fn gl_type() -> u32 {
        gl::UNSIGNED_INT // OpenGL constant for a 32-bit unsigned integer value
    }
}

/// Implementation of the `GLType` trait for `i16`.
/// Maps the Rust `i16` type to the OpenGL `SHORT` constant.
impl GLType for i16 {
    fn gl_type() -> u32 {
        gl::SHORT // OpenGL constant for a 16-bit signed integer value
    }
}

/// Implementation of the `GLType` trait for `u16`.
/// Maps the Rust `u16` type to the OpenGL `UNSIGNED_SHORT` constant.
impl GLType for u16 {
    fn gl_type() -> u32 {
        gl::UNSIGNED_SHORT // OpenGL constant for a 16-bit unsigned integer value
    }
}

/// Implementation of the `GLType` trait for `i8`.
/// Maps the Rust `i8` type to the OpenGL `BYTE` constant.
impl GLType for i8 {
    fn gl_type() -> u32 {
        gl::BYTE // OpenGL constant for an 8-bit signed integer value
    }
}

/// Implementation of the `GLType` trait for `u8`.
/// Maps the Rust `u8` type to the OpenGL `UNSIGNED_BYTE` constant.
impl GLType for u8 {
    fn gl_type() -> u32 {
        gl::UNSIGNED_BYTE // OpenGL constant for an 8-bit unsigned integer value
    }
}
