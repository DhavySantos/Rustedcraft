use std::ffi::c_void;
use super::GLType;

/// Structure that encapsulates the configuration of a vertex attribute pointer in OpenGL.
pub struct VertexAttribPointer {
    /// The index of the vertex attribute.
    index: u32,
}

impl VertexAttribPointer {
    /// Creates a new `VertexAttribPointer` and configures OpenGL to use this vertex attribute.
    ///
    /// # Parameters
    ///
    /// * `index` - The index of the attribute in the shader (e.g., `layout(location = 0)`).
    /// * `size` - The number of components in the attribute (e.g., 3 for a vec3).
    /// * `normalized` - Whether integer values should be normalized (e.g., mapped to [-1, 1] or [0, 1]).
    /// * `stride` - The spacing (in elements of type `T`) between consecutive attributes.
    /// * `pointer` - The offset (in elements of type `T`) from the beginning of the buffer to the attribute data.
    ///
    /// # Behavior
    ///
    /// This function computes the byte stride and offset based on the size of type `T`, retrieves the
    /// corresponding OpenGL type using the `GLType` trait, and sets up the attribute pointer.
    ///
    /// It automatically enables the vertex attribute array for the given index.
    ///
    /// # Type Parameter
    ///
    /// * `T` - A Rust type that implements the `GLType` trait, used to determine the OpenGL data type.
    ///
    /// # Example
    ///
    /// ```
    /// let position_attrib = VertexAttribPointer::new::<f32>(0, 3, false, 3, 0);
    /// ```
    pub fn new<T>(index: u32, size: usize, normalized: bool, stride: usize, pointer: usize) -> Self
    where
        T: GLType,
    {
        let normalized = if normalized { 1 } else { 0 };
        let pointer = (std::mem::size_of::<T>() * pointer) as *const c_void;
        let stride = (std::mem::size_of::<T>() * stride) as i32;
        let r#type = T::gl_type();

        unsafe {
            gl::VertexAttribPointer(index, size as i32, r#type, normalized, stride, pointer);
            gl::EnableVertexAttribArray(index);
        }

        Self { index }
    }

    /// Enables the vertex attribute array at this index.
    pub fn enable(&self) {
        unsafe { gl::EnableVertexAttribArray(self.index) };
    }

    /// Disables the vertex attribute array at this index.
    pub fn disable(&self) {
        unsafe { gl::DisableVertexAttribArray(self.index) };
    }

    /// Returns the index of this vertex attribute.
    pub fn index(&self) -> u32 {
        self.index
    }
}
