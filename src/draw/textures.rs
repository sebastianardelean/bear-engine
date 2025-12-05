use crate::utils::check_if_file_exists;
use crate::{error_log, trace_log};
use image::{DynamicImage, GenericImageView, ImageReader};
use std::io;
use std::path::{Path, PathBuf};
use gl::types::GLint;

#[derive(Clone)]
pub struct Texture {
    texture_id: u32,
    texture_file: String,
    texture_path:String,
    texture_type: String
}


impl Texture {
    pub fn new(
        texture_file: &str,
        texture_type:&str,
        texture_path:&str,
    ) -> io::Result<Texture> {
        let texture_file_path:PathBuf =[texture_path,texture_file].iter().collect();
        if !check_if_file_exists(texture_file_path.as_path())? {
            error_log!("{} does not exist", texture_file_path.as_path().display());
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Texture file not found",
            ));
        }

        return Ok(Texture {
            texture_id: 0,
            texture_file: texture_file.to_string(),
            texture_type: texture_type.to_string(),
            texture_path: texture_file_path.as_path().to_string_lossy().to_string(),
        });
    }

    pub fn get_texture_type(&self) -> &str {
        return &self.texture_type;
    }

    pub fn get_texture_id(&self) -> u32 {
        return self.texture_id;
    }

    pub fn get_texture_file(&self) -> &str {
        return &self.texture_file;
    }

    pub fn create_texture(&mut self) -> u32 {
        let mut id: u32 = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            // set texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as GLint);

            // set texture filtering parameters
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MIN_FILTER,
                gl::LINEAR_MIPMAP_LINEAR as GLint,
            );
            gl::TexParameteri(
                gl::TEXTURE_2D,
                gl::TEXTURE_MAG_FILTER,
                gl::LINEAR as GLint,
            );
        }

        // load image, create texture and generate mipmaps

        let img = image::open(self.texture_path.clone()).unwrap_or_else(|e|{
           error_log!("Error opening texture: {}", e);
            panic!("Error opening texture: {}", e);
        }).flipv();
        let (w, h) = img.dimensions();
        
        let (format, data_ptr) = match img {
            DynamicImage::ImageLuma8(img) => (gl::RED, img.into_raw()),
            DynamicImage::ImageRgb8(img)  => (gl::RGB, img.into_raw()),
            DynamicImage::ImageRgba8(img) => (gl::RGBA, img.into_raw()),
            // force conversion if needed
            _ => {
                let rgba = img.to_rgba8();
                (gl::RGBA, rgba.into_raw())
            }
        };


        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as GLint,
                w as i32,
                h as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                data_ptr.as_ptr() as *const std::ffi::c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);
        }
        self.texture_id = id;
        return id;
    }
}




