use gl::types::*;

/// Represents an OpenGL Vertex Array Object (VAO).
///
/// VAOs store the state of vertex attribute configuration, allowing you
/// to bind and reuse them without reconfiguring every time.
pub struct VertexArray {
    /// The OpenGL-generated ID for the VAO.
    id: u32,
}

impl VertexArray {
    /// Creates a new `VertexArray` by generating a VAO with OpenGL.
    ///
    /// # Returns
    ///
    /// A new `VertexArray` instance with a valid OpenGL VAO ID.
    pub fn new() -> Self {
        let mut id: GLuint = 0;
        unsafe { gl::GenVertexArrays(1, &mut id) };

        VertexArray { id }
    }

    /// Binds this VAO, making it the current active vertex array.
    ///
    /// Call this before configuring vertex attributes or rendering.
    pub fn bind(&self) {
        unsafe { gl::BindVertexArray(self.id) };
    }

    /// Unbinds any currently bound VAO.
    ///
    /// This binds VAO 0, effectively resetting the VAO state.
    pub fn unbind() {
        unsafe { gl::BindVertexArray(0) };
    }

    /// Returns the OpenGL ID of the VAO.
    ///
    /// Useful for low-level OpenGL operations or debugging.
    pub fn id(&self) -> GLuint {
        self.id
    }
}

impl Drop for VertexArray {
    /// Automatically called when the `VertexArray` goes out of scope.
    ///
    /// Deletes the OpenGL VAO to free GPU resources.
    fn drop(&mut self) {
        unsafe { gl::DeleteVertexArrays(1, &self.id) };
    }
}
