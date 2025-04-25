use super::Shader;

use std::ptr::{null_mut, null};
use std::ffi::{CString, CStr};
use gl::types::*;

/// A structure representing an OpenGL shader program.
///
/// This struct encapsulates the OpenGL shader program's ID, which is used to reference the program
/// within OpenGL operations, such as shader linking, usage, and resource management.
pub struct Program {
    id: u32,
}

impl Program {
    /// Creates a new shader program using `glCreateProgram`.
    ///
    /// # Returns
    /// A new `Program` instance with an OpenGL-generated ID.
    pub fn new() -> Self {
        let id = unsafe { gl::CreateProgram() };
        Program { id }
    }

    /// Attaches a compiled shader to this program.
    ///
    /// # Arguments
    /// * `shader` - A reference to a compiled `Shader`.
    pub fn attach_shader(&self, shader: &Shader) {
        unsafe { gl::AttachShader(self.id, shader.id()) };
    }

    /// Links the shader program and checks for errors.
    ///
    /// # Returns
    /// * `Ok(())` if linking succeeds.
    /// * `Err(String)` with an error log if linking fails.
    pub fn link(&self) -> Result<(), String> {
        unsafe {
            gl::LinkProgram(self.id);

            // Check if linking was successful
            let mut success = gl::FALSE as GLint;
            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                // Get length of the info log
                let mut len = 0;
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut len);

                // Allocate buffer for the log
                let mut buffer = Vec::with_capacity(len as usize);
                buffer.set_len((len as usize) - 1); // Reserve space, excluding null terminator

                // Retrieve the log message
                gl::GetProgramInfoLog(self.id, len, null_mut(), buffer.as_mut_ptr() as *mut GLchar);

                // Convert and return the log as a Rust String
                return Err(String::from_utf8_lossy(&buffer).to_string());
            }
        }

        Ok(())
    }

    /// Makes this shader program the current one used by OpenGL.
    ///
    /// Equivalent to calling `glUseProgram(self.id)`.
    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    /// Returns the OpenGL program ID.
    ///
    /// This can be used for setting uniforms or other OpenGL calls.
    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Program {
    /// Automatically deletes the OpenGL program when the `Program` is dropped.
    ///
    /// Prevents resource leaks by calling `glDeleteProgram`.
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }
}
