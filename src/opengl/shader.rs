use std::ptr::{null, null_mut};
use std::ffi::CString;
use gl::types::*;

/// Represents an OpenGL shader object.
///
/// Use `Shader::from_source` to compile a shader from GLSL source code.
pub struct Shader {
    id: u32,
}

/// Enum representing the different kinds of shaders supported by OpenGL.
///
/// Used when creating a new shader to specify the type.
#[repr(u32)]
pub enum ShaderType {
    TessEvaluation = gl::TESS_EVALUATION_SHADER,
    TessControl = gl::TESS_CONTROL_SHADER,
    Fragment = gl::FRAGMENT_SHADER,
    Geometry = gl::GEOMETRY_SHADER,
    Compute = gl::COMPUTE_SHADER,
    Vertex = gl::VERTEX_SHADER,
}

impl Shader {
    /// Compiles a new shader from source code.
    ///
    /// # Arguments
    /// * `source` - GLSL source code for the shader.
    /// * `shader_type` - The type of shader to compile (vertex, fragment, etc).
    ///
    /// # Returns
    /// * `Ok(Shader)` if compilation is successful.
    /// * `Err(String)` containing the compilation error log if it fails.
    pub fn from_source(source: &str, shader_type: ShaderType) -> Result<Self, String> {
        let shader;

        unsafe {
            // Create a new shader object
            shader = gl::CreateShader(shader_type as u32);

            // Convert Rust string to C-compatible string
            let c_str = CString::new(source.as_bytes()).unwrap();

            // Provide shader source to OpenGL and compile it
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), null());
            gl::CompileShader(shader);

            // Check if compilation was successful
            let mut success = gl::FALSE as GLint;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success != gl::TRUE as GLint {
                // Retrieve the length of the compilation error log
                let mut len = 0;
                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

                // Allocate buffer and read the log
                let mut buffer = Vec::with_capacity(len as usize);
                buffer.set_len((len as usize) - 1); // Exclude null terminator
                gl::GetShaderInfoLog(shader, len, null_mut(), buffer.as_mut_ptr() as *mut GLchar);

                // Return the log as a string
                return Err(String::from_utf8_lossy(&buffer).to_string());
            }
        }

        Ok(Self { id: shader })
    }

    /// Returns the OpenGL shader ID.
    ///
    /// Useful when attaching the shader to a program.
    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for Shader {
    /// Deletes the shader object when the `Shader` is dropped.
    ///
    /// Prevents memory/resource leaks in OpenGL.
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}
