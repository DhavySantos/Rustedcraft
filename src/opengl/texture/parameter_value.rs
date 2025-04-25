#[repr(u32)]
pub enum ParameterValue {
    LinearMipmapLinear = gl::LINEAR_MIPMAP_LINEAR,
    Linear = gl::LINEAR,
    Repeat = gl::REPEAT,
}
