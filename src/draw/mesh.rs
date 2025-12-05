use crate::draw::vertex::Vertex;
use crate::draw::{Shader, Texture};
use gl::types::{GLsizei, GLuint};
use std::mem::offset_of;
use std::os::raw::c_void;
use std::sync::OnceLock;

pub struct MeshObject {
    vertices: Vec<Vertex>,
    indices: Vec<u32>,
    texture: Vec<Texture>,
    gpu_buffers: OnceLock<(u32, u32, u32)>,
}

impl MeshObject {
    pub fn new(vertices: Vec<Vertex>, indices: Vec<u32>, texture: Vec<Texture>) -> MeshObject {
        return MeshObject {
            vertices: vertices,
            indices: indices,
            texture: texture,
            gpu_buffers: OnceLock::new(),
        };
    }



    pub fn draw(&mut self, shader:&mut Shader) {
        let mut diffuse_nr:u32 = 1;
        let mut specular_nr:u32 = 1;
        let mut normal_nr:u32 = 1;
        let mut height_nr:u32 = 1;
        let (vao, _vbo, _ebo): (GLuint, GLuint, GLuint) =
            *self.gpu_buffers.get_or_init(|| self.init_gpu_buffers());
        for (i, texture) in self.texture.iter_mut().enumerate() {
            unsafe {
                gl::ActiveTexture(gl::TEXTURE0 + i as u32);

            }
            let texture_type = texture.get_texture_type();
            let number:u32 = match texture_type {
                "texture_diffuse" => {
                    let value = diffuse_nr;
                    diffuse_nr +=1;
                    value
                }
                "texture_specular" => {
                    let value = specular_nr;
                    specular_nr +=1;
                    value
                }
                "texture_normal" => {
                    let value = normal_nr;
                    normal_nr +=1;
                    value
                }
                "texture_height" => {
                    let value = height_nr;
                    height_nr +=1;
                    value
                }
                &_ => {
                    0
                }
            };
            let uniform_name = format!("{}{}", texture_type, number);


            shader.set_int(uniform_name, i as i32);
            unsafe {
                gl::BindTexture(gl::TEXTURE_2D, texture.get_texture_id());
            }

        }
        unsafe {
            gl::BindVertexArray(vao);
            gl::DrawElements(
                gl::TRIANGLES,
                self.indices.len() as i32,
                gl::UNSIGNED_INT,
                std::ptr::null(),
            );
            gl::BindVertexArray(0);
            gl::ActiveTexture(gl::TEXTURE0);
         }
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
                (self.vertices.len() * std::mem::size_of::<Vertex>()) as isize,
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

            // vertex position
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                std::ptr::null(),
            );
            //vertex normals
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(
                1,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                offset_of!(Vertex, normal) as *const c_void,
            );

            //vertex texture coords
            gl::EnableVertexAttribArray(2);
            gl::VertexAttribPointer(
                2,
                2,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                offset_of!(Vertex, texture_coordinates) as *const c_void,
            );

            //vertex tangent
            gl::EnableVertexAttribArray(3);
            gl::VertexAttribPointer(
                3,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                offset_of!(Vertex, tangent) as *const c_void,
            );

            //vertex bitangent
            gl::EnableVertexAttribArray(4);
            gl::VertexAttribPointer(
                4,
                3,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                offset_of!(Vertex, bitangent) as *const c_void,
            );
            //ids
            gl::EnableVertexAttribArray(5);
            gl::VertexAttribIPointer(
                5,
                4,
                gl::INT,
                std::mem::size_of::<Vertex>() as GLsizei,
                offset_of!(Vertex, bone_ids) as *const c_void,
            );
            //weights
            gl::EnableVertexAttribArray(6);
            gl::VertexAttribPointer(
                6,
                4,
                gl::FLOAT,
                gl::FALSE,
                std::mem::size_of::<Vertex>() as GLsizei,
                offset_of!(Vertex, weights) as *const c_void,
            );
            gl::BindVertexArray(0);
        }

        return (vao, vbo, ebo);
    }
}
