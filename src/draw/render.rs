use std::os::raw::c_void;
use std::sync::OnceLock;
use gl::types::{GLsizei, GLuint};
use crate::draw::{bind_texture, Shape, Texture};

use crate::draw::shaders::Shader;


pub struct RenderManager {
    shapes: Vec<Shape>,
}

impl RenderManager {
    pub fn new() -> RenderManager {

        return RenderManager {
            shapes: Vec::new(),
        }
    }

    pub fn queue_shapes(&mut self, shape:Shape) {
        self.shapes.push(shape);
    }

    pub fn prepare(&mut self,
                         shader: &mut Shader,
                         textures:&Vec<(u32,&str)>) {
        shader.apply_shader();
        for (i,texture) in textures.iter().enumerate() {
            shader.set_int(String::from(texture.1), i as i32);
        }


    }

    pub fn apply_texture(&mut self,    shader: &mut Shader,
                         textures:&Vec<(u32,&str)>) {
        for (i,texture) in textures.iter().enumerate() {
            bind_texture(texture.0,i as u32);
        }

    }

    pub fn draw(&mut self,
                shader: &mut Shader){
        shader.apply_shader();

        for shape in &mut self.shapes {
            shape.draw();
        }

    }


}




