use gl::types::{GLsizei, GLuint};
use std::os::raw::c_void;
use std::sync::OnceLock;

pub struct Shape {
    vertices: Vec<f32>,
    indices: Option<Vec<u32>>,
    gpu_buffers: OnceLock<(u32, u32, u32)>,
}

impl Shape {
    pub fn new(vertices: Vec<f32>, indices: Option<Vec<u32>>) -> Self {
        return Shape {
            vertices: vertices,
            indices: indices,
            gpu_buffers: OnceLock::new(),
        };
    }
    fn init_gpu_buffers(
        &self,
    ) -> (GLuint, GLuint, GLuint) {
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
            if let Some(ref indices) = self.indices {
                gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
                gl::BufferData(
                    gl::ELEMENT_ARRAY_BUFFER,
                    (indices.len() * std::mem::size_of::<u32>()) as isize,
                    indices.as_ptr() as *const c_void,
                    gl::STATIC_DRAW,
                );
            }
            // position attribute
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // texture attribute
            gl::VertexAttribPointer(
                1,
                2,
                gl::FLOAT,
                gl::FALSE,
                (5 * std::mem::size_of::<f32>()) as GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);
        }

        return (vao, vbo, ebo);
    }

    pub fn draw(&mut self) {
        let (vao, _vbo, _ebo): (GLuint, GLuint, GLuint) = *self
            .gpu_buffers
            .get_or_init(|| self.init_gpu_buffers());
        if let Some(ref indices) = self.indices{
            unsafe {
                gl::BindVertexArray(vao);
                gl::DrawElements(
                    gl::TRIANGLES,
                    indices.len() as i32,
                    gl::UNSIGNED_INT,
                    std::ptr::null(),
                );
            }
        } else {
            unsafe {
                gl::BindVertexArray(vao);

                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
    }
}
