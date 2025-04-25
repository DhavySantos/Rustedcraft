use std::ffi::c_void;

/// A struct representing an OpenGL Buffer Object.
/// It holds the target, usage, and ID of the buffer object.
pub struct BufferObject {
    /// The target of the buffer object, such as an array buffer or a uniform buffer.
    target: BufferTarget,

    /// The usage pattern of the buffer object, which determines how it will be used.
    usage: BufferUsage,

    /// The unique identifier of the buffer object.
    id: u32,
}

/// An enum representing the various target types for OpenGL buffer objects.
/// Each variant corresponds to an OpenGL constant for a specific buffer target.
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum BufferTarget {
    /// Buffer used for storing vertex attribute data.
    ArrayBuffer = gl::ARRAY_BUFFER,

    /// Buffer used as the source for a copy operation (e.g., for reading).
    CopyReadBuffer = gl::COPY_READ_BUFFER,

    /// Buffer used as the destination for a copy operation (e.g., for writing).
    CopyWriteBuffer = gl::COPY_WRITE_BUFFER,

    /// Buffer used for storing element array data, such as index buffers.
    ElementArrayBuffer = gl::ELEMENT_ARRAY_BUFFER,

    /// Buffer used for packing pixel data, typically used for pixel transfers.
    PixelPackBuffer = gl::PIXEL_PACK_BUFFER,

    /// Buffer used for unpacking pixel data, typically used for pixel transfers.
    PixelUnpackBuffer = gl::PIXEL_UNPACK_BUFFER,

    /// Buffer used for storing texture data (used with texture operations).
    TextureBuffer = gl::TEXTURE_BUFFER,

    /// Buffer used for transform feedback operations.
    TransformFeedbackBuffer = gl::TRANSFORM_FEEDBACK_BUFFER,

    /// Buffer used for storing uniform data for shaders.
    UniformBuffer = gl::UNIFORM_BUFFER,
}

/// An enum representing the usage pattern for OpenGL buffer objects.
/// Each variant corresponds to an OpenGL constant specifying how a buffer will be used.
/// The usage hints help OpenGL optimize for performance.
#[repr(u32)]
#[derive(Debug, Clone, Copy)]
pub enum BufferUsage {
    /// Buffer intended for dynamic drawing. Data will change frequently.
    DynamicDraw = gl::DYNAMIC_DRAW,
    /// Buffer intended for static drawing. Data will not change frequently.
    StaticDraw = gl::STATIC_DRAW,
    /// Buffer intended for streaming data. Data will change frequently and in small bursts.
    StreamDraw = gl::STREAM_DRAW,
    /// Buffer intended for dynamic reading. Data will be read frequently and may change.
    DynamicRead = gl::DYNAMIC_READ,
    /// Buffer intended for static reading. Data will not change and is read frequently.
    StaticRead = gl::STATIC_READ,
    /// Buffer intended for streaming reading. Data will change frequently and in small bursts.
    StreamRead = gl::STREAM_READ,
}

impl BufferObject {
    /// Creates a new `BufferObject` and generates an OpenGL buffer with the given target and usage.
    ///
    /// # Parameters
    /// - `target`: Specifies the target of the buffer (e.g., `ArrayBuffer`).
    /// - `usage`: Specifies the intended usage of the buffer (e.g., `StaticDraw`).
    ///
    /// # Returns
    /// - A `BufferObject` that contains the generated buffer ID and its target/usage.
    pub fn new(target: BufferTarget, usage: BufferUsage) -> Self {
        let mut id = 0; // Variable to hold the buffer ID.

        // Generate a new buffer ID using OpenGL's GenBuffers function.
        unsafe { gl::GenBuffers(1, &mut id) };

        // Return a new `BufferObject` with the generated ID, target, and usage.
        BufferObject { id, target, usage }
    }

    /// Uploads data to the OpenGL buffer.
    ///
    /// # Parameters
    /// - `data`: The data to be uploaded to the buffer, typically vertex data, indices, etc.
    ///
    /// # This method
    /// - Binds the buffer to the OpenGL context, uploads the provided data to the buffer, and assigns the specified usage.
    pub fn data<T>(&self, data: &[T]) {
        let ptr = data.as_ptr() as *const c_void; // Convert the data slice into a raw pointer (c_void).
        let size = size_of_val(data) as isize; // Calculate the size of the data in bytes.

        // Upload the data to the GPU buffer using OpenGL's BufferData function.
        unsafe {
            gl::BindBuffer(self.target as u32, self.id); // Bind the buffer to the OpenGL context.
            gl::BufferData(self.target as u32, size, ptr, self.usage as u32); // Upload the data.
        }
    }

    /// Binds the buffer to its target in OpenGL.
    ///
    /// This method ensures that all future OpenGL operations on this buffer will refer to the correct buffer.
    pub fn bind(&self) {
        unsafe { gl::BindBuffer(self.target as u32, self.id) }; // Bind the buffer to its target.
    }
}

impl Drop for BufferObject {
    /// Deletes the OpenGL buffer when the `BufferObject` is dropped.
    ///
    /// This ensures that the allocated buffer memory is freed when the object goes out of scope,
    /// preventing memory leaks in the OpenGL context.
    fn drop(&mut self) {
        unsafe { gl::DeleteBuffers(1, &self.id) }; // Delete the buffer from OpenGL.
    }
}
