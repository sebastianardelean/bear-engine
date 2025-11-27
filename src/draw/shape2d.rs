use gl::types::{GLsizei, GLuint};
use std::os::raw::c_void;
use std::sync::OnceLock;
use crate::draw::lib::Shape;

pub struct Shape2D {
    vertices: Vec<f32>,
    indices: Vec<u32>,
    gpu_buffers: OnceLock<(u32, u32, u32)>,
}

impl Shape2D {
    pub fn new(vertices: Vec<f32>, indices: Vec<u32>) -> Self {
        return Shape2D {
            vertices: vertices,
            indices: indices,
            gpu_buffers: OnceLock::new(),
        };
    }
    fn init_gpu_buffers(&self) -> (GLuint, GLuint, GLuint) {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<f32>()) as isize,
                self.vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (self.indices.len() * std::mem::size_of::<u32>()) as isize,
                self.indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // color attribute
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            // texture attribute
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as GLsizei,
                (6 * std::mem::size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(2);
        }

        return (vao, vbo, ebo);
    }
}

impl Shape for Shape2D {
    fn draw(&mut self) {
        let (vao, _vbo, _ebo): (GLuint, GLuint, GLuint) =
            *self.gpu_buffers.get_or_init(|| self.init_gpu_buffers());

        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
        }
    }


}
