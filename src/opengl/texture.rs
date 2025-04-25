pub struct Texture {
    target: TextureTarget,
    id: u32,
}

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

#[repr(u32)]
#[derive(Clone, Copy)]
pub enum TexParName {
    BaseLevel = gl::TEXTURE_BASE_LEVEL,
    BorderColor = gl::TEXTURE_BORDER_COLOR,
    CompareFunc = gl::TEXTURE_COMPARE_FUNC,
    CompareMode = gl::TEXTURE_COMPARE_MODE,
    LodBias = gl::TEXTURE_LOD_BIAS,
    MinFilter = gl::TEXTURE_MIN_FILTER,
    MagFilter = gl::TEXTURE_MAG_FILTER,
    MinLod = gl::TEXTURE_MIN_LOD,
    MaxLod = gl::TEXTURE_MAX_LOD,
    MaxLevel = gl::TEXTURE_MAX_LEVEL,
    SwizzleR = gl::TEXTURE_SWIZZLE_R,
    SwizzleG = gl::TEXTURE_SWIZZLE_G,
    SwizzleB = gl::TEXTURE_SWIZZLE_B,
    SwizzleA = gl::TEXTURE_SWIZZLE_A,
    SwizzleRgba = gl::TEXTURE_SWIZZLE_RGBA,
    WrapS = gl::TEXTURE_WRAP_S,
    WrapT = gl::TEXTURE_WRAP_T,
    WrapR = gl::TEXTURE_WRAP_R,
}

#[repr(u32)]
pub enum TexParValue {
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
    Linear = gl::LINEAR,
    Repeat = gl::REPEAT,
}

#[repr(u32)]
pub enum TexInternalFormat {
    RGB = gl::RGB,
}

impl Texture {
    pub fn new(target: TextureTarget) -> Self {
        let mut id = 0;
        unsafe { gl::GenTextures(1, &mut id) };

        Self { id, target }
    }

    pub fn parameter(&self, pname: TexParName, pvalue: TexParValue) {
        self.bind();
        unsafe { gl::TextureParameteri(self.target as u32, pname as u32, pvalue as i32) };
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(self.target as u32, self.id) };
    }
}
