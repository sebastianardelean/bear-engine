use std::io;
use std::path::Path;
use crate::error_log;
use crate::utils::check_if_file_exists;


pub struct Texture {
    id: u32,
    texture_wrap_s: i32,
    texture_wrap_t: i32,
    texture_wrap_r: i32,
    texture_filter_min: i32,
    texture_filter_mag:i32
}

impl Texture  {
    pub fn new(texture_wrap_s: i32,
               texture_wrap_t: i32,
               texture_wrap_r: i32,
               texture_filter_min:i32,
               texture_fiter_mag:i32) -> io::Result<Texture> {
        // if !check_if_file_exists(Path::new(texture_file_path))? {
        //     error_log!("{} does not exist", texture_file_path);
        //     return Err(io::Error::new(io::ErrorKind::NotFound, "Texture file not found"));
        // }


        return Ok(Texture {
            id: 0,
            texture_wrap_s: texture_wrap_s,
            texture_wrap_t: texture_wrap_t,
            texture_wrap_r: texture_wrap_r,//for 3D 
            texture_filter_min: texture_filter_min,
            texture_filter_mag: texture_fiter_mag
        });
    }

    pub fn create_texture(&mut self) {
        unsafe {
            gl::GenTextures(
                1,
                &mut self.id
            );
            gl::BindTexture(gl::TEXTURE_2D, self.id);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.texture_wrap_s);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.texture_wrap_t);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.texture_filter_min);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.texture_filter_mag);




        }
    }

}

// glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_REPEAT);
// glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_REPEAT);
// glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_LINEAR_MIPMAP_LINEAR);
// glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);