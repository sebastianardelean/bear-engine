use gl::types::{GLsizei, GLuint};
use std::os::raw::c_void;
use std::sync::OnceLock;

#[derive(Clone, Copy)]
pub enum DrawMode {
    RenderTexture,
    RenderLight,
    RenderBoth,
}

pub struct Shape {
    vertices: Vec<f32>,
    indices: Option<Vec<u32>>,
    draw_mode: DrawMode,
    gpu_buffers: OnceLock<(u32, u32, u32)>,
    number_of_triangles: i32,
}
const COORDINATES_INDEX: u32 = 0;
const COORDINATES_SIZE: i32 = 3;
const TEXTURE_SIZE: i32 = 2;
const NORMAL_SIZE: i32 = 3;
impl Shape {
    pub fn new(vertices: Vec<f32>, indices: Option<Vec<u32>>, draw_mode: DrawMode) -> Self {
        let len: i32 = vertices.len() as i32;
        let number_of_triangles: i32 = match draw_mode {
            DrawMode::RenderTexture => len / (COORDINATES_SIZE + TEXTURE_SIZE),
            DrawMode::RenderLight => len / (COORDINATES_SIZE + NORMAL_SIZE),
            DrawMode::RenderBoth => len / (COORDINATES_SIZE + TEXTURE_SIZE + NORMAL_SIZE),
        };

        return Shape {
            vertices: vertices,
            indices: indices,
            draw_mode: draw_mode,
            gpu_buffers: OnceLock::new(),
            number_of_triangles: number_of_triangles,
        };
    }

    fn init_gpu_buffers(&self) -> (GLuint, GLuint, GLuint) {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;
        let mut ebo: GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            if self.indices.is_some() {
                gl::GenBuffers(1, &mut ebo);
            }

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
            match self.draw_mode {
                DrawMode::RenderTexture => {
                    let stride: usize = (COORDINATES_SIZE + TEXTURE_SIZE) as usize;
                    let texture_offset: usize = COORDINATES_SIZE as usize;
                    // position attribute
                    gl::VertexAttribPointer(
                        0,
                        COORDINATES_SIZE,
                        gl::FLOAT,
                        gl::FALSE,
                        (stride * std::mem::size_of::<f32>()) as GLsizei,
                        std::ptr::null(),
                    );
                    gl::EnableVertexAttribArray(COORDINATES_INDEX);

                    // texture attribute
                    gl::VertexAttribPointer(
                        1,
                        TEXTURE_SIZE,
                        gl::FLOAT,
                        gl::FALSE,
                        (stride * std::mem::size_of::<f32>()) as GLsizei,
                        (texture_offset * std::mem::size_of::<f32>()) as *const c_void,
                    );
                    gl::EnableVertexAttribArray(1);
                }
                DrawMode::RenderLight => {
                    let stride: usize = (COORDINATES_SIZE + NORMAL_SIZE) as usize;
                    let normal_offset: usize = COORDINATES_SIZE as usize;
                    // position attribute
                    gl::VertexAttribPointer(
                        0,
                        COORDINATES_SIZE,
                        gl::FLOAT,
                        gl::FALSE,
                        (stride * std::mem::size_of::<f32>()) as GLsizei,
                        std::ptr::null(),
                    );
                    gl::EnableVertexAttribArray(0);
                    // light and material attribute
                    gl::VertexAttribPointer(
                        1,
                        NORMAL_SIZE,
                        gl::FLOAT,
                        gl::FALSE,
                        (stride * std::mem::size_of::<f32>()) as GLsizei,
                        (normal_offset * std::mem::size_of::<f32>()) as *const c_void,
                    );
                    gl::EnableVertexAttribArray(1);
                }
                DrawMode::RenderBoth => {
                    let stride: usize = (COORDINATES_SIZE + TEXTURE_SIZE + NORMAL_SIZE) as usize;
                    let texture_offset: usize = (COORDINATES_SIZE + TEXTURE_SIZE) as usize;
                    let normal_offset: usize = (COORDINATES_SIZE) as usize;

                    // position attribute
                    gl::VertexAttribPointer(
                        0,
                        COORDINATES_SIZE,
                        gl::FLOAT,
                        gl::FALSE,
                        (stride * std::mem::size_of::<f32>()) as GLsizei,
                        std::ptr::null(),
                    );
                    gl::EnableVertexAttribArray(0);
                    // light and material attribute
                    gl::VertexAttribPointer(
                        1,
                        NORMAL_SIZE,
                        gl::FLOAT,
                        gl::FALSE,
                        (stride * std::mem::size_of::<f32>()) as GLsizei,
                        (normal_offset * std::mem::size_of::<f32>()) as *const c_void,
                    );
                    gl::EnableVertexAttribArray(1);
                    // texture attribute
                    gl::VertexAttribPointer(
                        2,
                        TEXTURE_SIZE,
                        gl::FLOAT,
                        gl::FALSE,
                        (stride * std::mem::size_of::<f32>()) as GLsizei,
                        (texture_offset * std::mem::size_of::<f32>()) as *const c_void,
                    );
                    gl::EnableVertexAttribArray(2);
                }
            }
        }

        return (vao, vbo, ebo);
    }

    pub fn draw(&mut self) {
        let (vao, _vbo, _ebo): (GLuint, GLuint, GLuint) =
            *self.gpu_buffers.get_or_init(|| self.init_gpu_buffers());
        if let Some(ref indices) = self.indices {
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

                gl::DrawArrays(gl::TRIANGLES, 0, self.number_of_triangles);
            }
        }
    }
}
