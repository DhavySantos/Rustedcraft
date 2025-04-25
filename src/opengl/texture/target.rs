#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TextureTarget {
    Texture1D = gl::TEXTURE_1D,
    Texture2D = gl::TEXTURE_2D,
    Texture3D = gl::TEXTURE_3D,
    Texture1DArray = gl::TEXTURE_1D_ARRAY,
    Texture2DArray = gl::TEXTURE_2D_ARRAY,
    TextureRectangle = gl::TEXTURE_RECTANGLE,
    TextureCubeMap = gl::TEXTURE_CUBE_MAP,
}
