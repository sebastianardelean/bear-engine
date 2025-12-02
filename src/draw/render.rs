use crate::draw::shaders::Shader;
use crate::draw::{Shape, bind_texture};

pub fn render_prepare(shader: &mut Shader, textures: &Vec<(u32, &str)>) {
    shader.apply_shader();
    for (i, texture) in textures.iter().enumerate() {
        shader.set_int(String::from(texture.1), i as i32);
    }
}

pub fn render_bind_texture(textures: &Vec<(u32, &str)>) {
    for (i, texture) in textures.iter().enumerate() {
        bind_texture(texture.0, i as u32);
    }
}

pub fn render(shader: &mut Shader, shape: &mut Shape) {
    shader.apply_shader();
    shape.draw();
}
