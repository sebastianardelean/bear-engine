use std::os::raw::c_void;
use std::sync::OnceLock;
use gl::types::{GLint, GLuint};
use crate::utils::coordinates::ortho;
use crate::draw::shaders::Shader;


pub struct DrawManager {
    vao_vbo:OnceLock<(u32,u32)>
}

impl DrawManager {
    pub fn new() -> DrawManager {
        return DrawManager{
            vao_vbo:OnceLock::new(),
        }
    }

    fn init_vao_vbo(&self, vertices: &[f32]) -> (GLuint, GLuint) {
        let mut vao:GLuint = 0;
        let mut vbo:GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // position attribute
            gl::VertexAttribPointer(
                0, 3, gl::FLOAT, gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as GLint,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // color attribute
            gl::VertexAttribPointer(
                1, 3, gl::FLOAT, gl::FALSE,
                (6 * std::mem::size_of::<f32>()) as GLint,
                (3 * std::mem::size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            gl::BindVertexArray(0);
        }

        return (vao, vbo);
    }

    pub fn draw(&mut self, vertices: &[f32], shader: &mut Shader, program: GLuint, w: i32, h: i32) {
        let (vao, _vbo):(GLuint,GLuint) = *self.vao_vbo.get_or_init(|| {
            self.init_vao_vbo(vertices)
        });

        shader.apply_shader();

        shader.set_float(String::from("someUniform"), 1.0 as GLint);


        unsafe {

            let proj = ortho(0.0, w as f32, h as f32, 0.0, -1.0, 1.0);
            let loc = gl::GetUniformLocation(program, b"uProjection\0".as_ptr() as _);
            gl::UniformMatrix4fv(loc, 1, gl::FALSE, proj.as_ptr());

            let col = gl::GetUniformLocation(program, b"uColor\0".as_ptr() as _);
            gl::Uniform3f(col, 1.0, 0.3, 0.2);
            gl::BindVertexArray(vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);

        }

    }
}




