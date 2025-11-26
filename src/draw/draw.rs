use std::os::raw::c_void;
use std::sync::OnceLock;
use gl::types::{GLint, GLsizei, GLuint};
use crate::draw::bind_texture;
use crate::utils::coordinates::ortho;
use crate::draw::shaders::Shader;
use crate::draw::textures::Texture;

pub struct DrawManager {
    gpu_buffers:OnceLock<(u32, u32,u32)>
}

impl DrawManager {
    pub fn new() -> DrawManager {
        return DrawManager{
            gpu_buffers:OnceLock::new(),
        }
    }

    fn init_gpu_buffers(&self, vertices: &[f32], indices:&[u32], use_textures:bool) -> (GLuint, GLuint, GLuint) {
        let mut vao:GLuint = 0;
        let mut vbo:GLuint = 0;
        let mut ebo:GLuint = 0;

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ebo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (vertices.len() * std::mem::size_of::<f32>()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER,
                (indices.len() *std::mem::size_of::<u32>()) as isize,
                indices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // position attribute
            gl::VertexAttribPointer(
                0, 3, gl::FLOAT, gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as GLsizei,
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);

            // color attribute
            gl::VertexAttribPointer(
                1, 3, gl::FLOAT, gl::FALSE,
                (8 * std::mem::size_of::<f32>()) as GLsizei,
                (3 * std::mem::size_of::<f32>()) as *const c_void,
            );
            gl::EnableVertexAttribArray(1);

            if use_textures{
                // texture attribute
                gl::VertexAttribPointer(
                    2, 2, gl::FLOAT, gl::FALSE,
                    (8 * std::mem::size_of::<f32>()) as GLsizei,
                    (6 * std::mem::size_of::<f32>()) as *const c_void,
                );
                gl::EnableVertexAttribArray(2);
            }
        }

        return (vao, vbo,ebo);
    }

    pub fn draw(&mut self, vertices: &[f32],
                indices: &[u32],
                shader: &mut Shader,
                textures:&Vec<(u32,&str)>,
                use_texture: bool

    ) {
        let (vao, _vbo,_ebo):(GLuint,GLuint,GLuint) = *self.gpu_buffers.get_or_init(|| {
            self.init_gpu_buffers(vertices, indices,use_texture)

        });

        if use_texture {
            shader.apply_shader();
            for (i,texture) in textures.iter().enumerate() {
                shader.set_int(String::from(texture.1), i as i32);

            }
            for (i,texture) in textures.iter().enumerate() {
                bind_texture(texture.0,i as u32);
            }
        }


        shader.apply_shader();




        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_INT, std::ptr::null());

        }

    }


}




