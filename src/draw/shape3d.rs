use gl::types::{GLsizei, GLuint};
use std::os::raw::c_void;
use std::sync::OnceLock;
use glam::{Mat4, Vec3};
use crate::draw::{get_identity, rotate, translate};
use crate::draw::lib::Shape;

pub struct Shape3D {
    vertices: Vec<f32>,
    positions: Vec<Vec3>,
    gpu_buffers: OnceLock<(u32, u32)>,
}

impl Shape3D {
    pub fn new(vertices: Vec<f32>, positions: Vec<Vec3>) -> Self {
        return Shape3D {
            vertices: vertices,
            positions: positions,
            gpu_buffers: OnceLock::new(),
        };
    }
    fn init_gpu_buffers(&self) -> (GLuint, GLuint) {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;


        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);


            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * std::mem::size_of::<f32>()) as isize,
                self.vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );


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

        return (vao, vbo);
    }
}

impl Shape for Shape3D {
    fn draw(&mut self) {
        let (vao, _vbo): (GLuint, GLuint) =
            *self.gpu_buffers.get_or_init(|| self.init_gpu_buffers());

        unsafe {
            gl::BindVertexArray(vao);

            gl::DrawArrays(gl::TRIANGLES, 0, 36);
            
            // gl::DrawElements(
            //     gl::TRIANGLES,
            //     self.indices.len() as i32,
            //     gl::UNSIGNED_INT,
            //     std::ptr::null(),
            // );
        }
    }
}