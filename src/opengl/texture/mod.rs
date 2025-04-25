mod parameter_value;
mod internal_format;
mod parameter_name;
mod target;

pub use parameter_value::ParameterValue;
pub use internal_format::InternalFormat;
pub use parameter_name::ParameterName;
pub use target::TextureTarget;

pub struct Texture {
    target: TextureTarget,
    id: u32,
}

impl Texture {
    pub fn new(target: TextureTarget) -> Self {
        let mut id = 0;
        unsafe { gl::GenTextures(1, &mut id) };

        Self { id, target }
    }

    pub fn parameter(&self, pname: ParameterName, pvalue: ParameterValue) {
        self.bind();
        unsafe { gl::TextureParameteri(self.target as u32, pname as u32, pvalue as i32) };
    }

    pub fn bind(&self) {
        unsafe { gl::BindTexture(self.target as u32, self.id) };
    }
}
