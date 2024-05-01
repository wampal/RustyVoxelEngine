use gl::types::*;

pub struct Texture {
    id: GLuint,
    _width: i32,
    _height: i32,
}

impl Texture {
    pub fn new(id: GLuint, _width: i32, _height: i32) -> Self {
        Self { id, _width, _height }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }
}
