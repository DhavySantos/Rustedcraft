#[repr(u32)]
#[derive(Clone, Copy)]
pub enum ParameterName {
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
