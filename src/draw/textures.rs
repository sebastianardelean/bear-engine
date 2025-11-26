use std::io;
use std::path::Path;
use image::ImageReader;
use crate::{error_log, trace_log};
use crate::utils::check_if_file_exists;


pub struct Texture {
    texture_file: String,
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
               texture_filter_mag:i32,
               texture_file_path:&str) -> io::Result<Texture> {

        if !check_if_file_exists(Path::new(texture_file_path))? {
            error_log!("{} does not exist", texture_file_path);
            return Err(io::Error::new(io::ErrorKind::NotFound, "Texture file not found"));
        }

        return Ok(Texture {
            texture_file: texture_file_path.to_string(),
            texture_wrap_s: texture_wrap_s,
            texture_wrap_t: texture_wrap_t,
            texture_wrap_r: texture_wrap_r,
            texture_filter_min: texture_filter_min,
            texture_filter_mag: texture_filter_mag
        });
    }

    pub fn create_texture(&mut self) ->u32{
        let mut id:u32 = 0;
        unsafe {
            gl::GenTextures(
                1,
                &mut id
            );
            gl::BindTexture(gl::TEXTURE_2D, id);
            // set texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, self.texture_wrap_s);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, self.texture_wrap_t);

            // set texture filtering parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, self.texture_filter_min);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, self.texture_filter_mag);
        }

        // load image, create texture and generate mipmaps

        let img = ImageReader::open(self.texture_file.clone()).unwrap().decode().unwrap().flipv().to_rgba8();
        

        let (w, h) = img.dimensions();
        let data_ptr = img.as_ptr();

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                w as i32,
                h as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                data_ptr as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        return id;
    }

}

pub fn bind_texture(id:u32, texture_offset:u32) {
    trace_log!("Binding texture {} with offset {}", id,texture_offset);
    unsafe {
        gl::ActiveTexture(gl::TEXTURE0+texture_offset);
        gl::BindTexture(gl::TEXTURE_2D, id);
    }
}

