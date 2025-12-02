use glam::Vec3;
use crate::draw::{Camera, Shader};

pub struct Light {
    light_positions: Vec<Vec3>
}

impl Light {
    pub fn new(light_positions:Vec<Vec3>) -> Light {
        return Light{

            light_positions:light_positions
        };
    }

    pub fn apply_directional_light(&mut self, shader : &mut Shader) {
        // directional light
        shader.set_uniform_3v(String::from("dirLight.direction"), Vec3::new(-0.2, -1.0, -0.3));
        shader.set_uniform_3v(String::from("dirLight.ambient"), Vec3::new(0.05, 0.05, 0.05));
        shader.set_uniform_3v(String::from("dirLight.diffuse"), Vec3::new(0.4, 0.4, 0.4));
        shader.set_uniform_3v(String::from("dirLight.specular"), Vec3::new(0.5, 0.5, 0.5));
    }

    pub fn apply_spotlight(&mut self, shader : &mut Shader, camera: &mut Camera) {
        // spotLight
        shader.set_uniform_3v(String::from("spotLight.position"), *camera.get_position());
        shader.set_uniform_3v(String::from("spotLight.direction"), *camera.get_front());
        shader.set_uniform_3v(String::from("spotLight.ambient"), Vec3::new(0.0, 0.0, 0.0));
        shader.set_uniform_3v(String::from("spotLight.diffuse"), Vec3::new(1.0, 1.0, 1.0));
        shader.set_uniform_3v(String::from("spotLight.specular"), Vec3::new(1.0, 1.0, 1.0));
        shader.set_float(String::from("spotLight.constant"), 1.0);
        shader.set_float(String::from("spotLight.linear"), 0.09);
        shader.set_float(String::from("spotLight.quadratic"), 0.032);
        shader.set_float(String::from("spotLight.cutOff"), 12.5_f32.to_radians().cos());
        shader.set_float(String::from("spotLight.outerCutOff"), 15.0_f32.to_radians().cos());
    }

    pub fn apply_default_light_properties(&mut self, shader : &mut Shader, camera: &mut Camera) {



        for (i, position) in self.light_positions.iter().enumerate() {
            shader.set_uniform_3v(format!("pointLights[{}].position", i), self.light_positions[i]);
            shader.set_uniform_3v(format!("pointLights[{}].ambient", i), Vec3::new(0.05, 0.05, 0.05));
            shader.set_uniform_3v(format!("pointLights[{}].diffuse", i), Vec3::new(0.8, 0.8, 0.8));
            shader.set_uniform_3v(format!("pointLights[{}].specular", i), Vec3::new(1.0, 1.0, 1.0));
            shader.set_float(format!("pointLights[{}].constant", i), 1.0);
            shader.set_float(format!("pointLights[{}].linear", i), 0.09);
            shader.set_float(format!("pointLights[{}].quadratic", i), 0.032);
        }




    }
}