use gl::types::*;

pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,
    vertices: usize,
    _vertex_size: usize,
}

impl Mesh {
    pub fn new(buffer: *const f32, vertices: usize, attrs: *const i32) -> Self {
        let mut _vertex_size = 0;
        let mut i = 0;
        while unsafe { *attrs.offset(i) } != 0 {
            _vertex_size += unsafe { *attrs.offset(i) as usize };
            i += 1;
        }

        let mut vao = 0;
        let mut vbo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * _vertex_size * vertices) as GLsizeiptr,
                buffer as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );

            let mut offset = 0;
            let mut i = 0;
            while *attrs.offset(i) != 0 {
                let size = *attrs.offset(i) as GLint;
                gl::VertexAttribPointer(
                    i as GLuint,
                    size,
                    gl::FLOAT,
                    gl::FALSE,
                    (_vertex_size * std::mem::size_of::<f32>()) as GLint,
                    (offset * std::mem::size_of::<f32>()) as *const std::ffi::c_void,
                );
                gl::EnableVertexAttribArray(i as GLuint);
                offset += size as usize;
                i += 1;
            }

            gl::BindVertexArray(0);
        }

        Mesh {
            vao,
            vbo,
            vertices,
            _vertex_size,
        }
    }

    pub fn reload(&mut self, buffer: *const f32, vertices: usize) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (std::mem::size_of::<f32>() * self._vertex_size * vertices) as GLsizeiptr,
                buffer as *const std::ffi::c_void,
                gl::STATIC_DRAW,
            );
        }
        self.vertices = vertices;
    }

    pub fn draw(&self, primitive: GLenum) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(primitive, 0, self.vertices as GLsizei);
            gl::BindVertexArray(0);
        }
    }
}

impl Drop for Mesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
