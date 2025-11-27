use crate::error_log;
use crate::utils::*;
use gl::types::{GLchar, GLfloat, GLint, GLuint};
use std::ffi::CString;
use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::path::Path;

pub struct Shader {
    id: u32,
    vertex_shader: String,
    fragment_shader: String,
}

impl Shader {
    pub fn new(vertex_shader_file: &str, fragment_shader_file: &str) -> io::Result<Shader> {
        if !check_if_file_exists(Path::new(vertex_shader_file))? {
            error_log!("Vertex shader file does not exist: {}", vertex_shader_file);
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Vertex shader not found: {}", vertex_shader_file),
            ));
        }
        if !check_if_file_exists(Path::new(fragment_shader_file))? {
            error_log!("Fragment shader not found: {}", fragment_shader_file);
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Vertex shader not found: {}", fragment_shader_file),
            ));
        }

        let mut contents_vertex: String = String::new();
        let mut buf_reader_vertex: BufReader<File> =
            BufReader::new(File::open(vertex_shader_file)?);
        buf_reader_vertex
            .read_to_string(&mut contents_vertex)
            .unwrap_or_else(|e| {
                error_log!("Failed to initialize vertex shader: {}", e);
                panic!("Failed!");
            });

        let mut contents_fragment: String = String::new();
        let mut buf_reader_fragment: BufReader<File> =
            BufReader::new(File::open(fragment_shader_file)?);
        buf_reader_fragment
            .read_to_string(&mut contents_fragment)
            .unwrap_or_else(|e| {
                error_log!("Failed to initialize vertex shader: {}", e);
                panic!("Failed!");
            });

        return Ok(Shader {
            id: 0,
            vertex_shader: contents_vertex,
            fragment_shader: contents_fragment,
        });
    }

    fn compile(&mut self) -> (GLuint, GLuint) {
        let mut success: GLint = 1;

        unsafe {
            //compile the vertex shader
            let vertex: GLuint = gl::CreateShader(gl::VERTEX_SHADER);
            let v_shader_code: CString = CString::new(self.vertex_shader.clone()).unwrap();
            gl::ShaderSource(vertex, 1, &v_shader_code.as_ptr(), std::ptr::null());
            gl::CompileShader(vertex);
            gl::GetShaderiv(vertex, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut message_len: GLint = 0;
                gl::GetShaderiv(vertex, gl::INFO_LOG_LENGTH, &mut message_len);
                let mut buffer = Vec::with_capacity(message_len as usize);
                buffer.set_len((message_len as usize) - 1);
                gl::GetShaderInfoLog(
                    vertex,
                    message_len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
                let log = String::from_utf8_lossy(&buffer);
                error_log!(
                    "Error {} when compiling shader {} ",
                    log,
                    self.vertex_shader
                );
            }

            let fragment: GLuint = gl::CreateShader(gl::FRAGMENT_SHADER);
            let f_shader_code: CString = CString::new(self.fragment_shader.clone()).unwrap();
            gl::ShaderSource(fragment, 1, &f_shader_code.as_ptr(), std::ptr::null());
            gl::CompileShader(fragment);

            gl::GetShaderiv(fragment, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut message_len: GLint = 0;
                gl::GetShaderiv(vertex, gl::INFO_LOG_LENGTH, &mut message_len);
                let mut buffer = Vec::with_capacity(message_len as usize);
                buffer.set_len((message_len as usize) - 1);
                gl::GetShaderInfoLog(
                    vertex,
                    message_len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
                let log = String::from_utf8_lossy(&buffer);
                error_log!(
                    "Error {} when compiling shader {} ",
                    log,
                    self.vertex_shader
                );
            }

            return (vertex, fragment);
        }
    }

    fn link(&mut self, vs: GLuint, fs: GLuint) -> GLuint {
        let mut success: GLint = 1;
        unsafe {
            self.id = gl::CreateProgram();
            gl::AttachShader(self.id, vs);
            gl::AttachShader(self.id, fs);
            gl::LinkProgram(self.id);

            gl::GetProgramiv(self.id, gl::LINK_STATUS, &mut success);

            if success == 0 {
                let mut message_len: GLint = 0;
                gl::GetProgramiv(self.id, gl::INFO_LOG_LENGTH, &mut message_len);
                let mut buffer = Vec::with_capacity(message_len as usize);
                buffer.set_len((message_len as usize) - 1);
                gl::GetProgramInfoLog(
                    self.id,
                    message_len,
                    std::ptr::null_mut(),
                    buffer.as_mut_ptr() as *mut GLchar,
                );
                let log = String::from_utf8_lossy(&buffer);
                error_log!("Error {} when linking shader {} ", log, self.vertex_shader);
            }
            return self.id;
        }
    }

    pub fn build_shader(&mut self) -> u32 {
        let (vs, fs): (GLuint, GLuint) = self.compile();
        return self.link(vs, fs);
    }
    pub fn apply_shader(&mut self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    pub fn set_bool(&mut self, name: String, value: bool) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                value as GLint,
            );
        }
    }

    pub fn set_int(&mut self, name: String, value: i32) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::Uniform1i(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                value as GLint,
            );
        }
    }

    pub fn set_float(&mut self, name: String, value: f32) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                value as GLfloat,
            );
        }
    }

    pub fn get_uniform_location(&self, name: String) -> i32 {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            return gl::GetUniformLocation(self.id, c_name.as_ptr());
        }
    }

    pub fn set_uniform_2v(&mut self, name: String, value: glam::Vec2) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::Uniform2fv(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                1,
                value.to_array().as_ptr(),
            );
        }
    }

    pub fn set_uniform_2(&mut self, name: String, x: f32, y: f32) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::Uniform2f(gl::GetUniformLocation(self.id, c_name.as_ptr()), x, y);
        }
    }

    pub fn set_uniform_3v(&mut self, name: String, value: glam::Vec3) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::Uniform3fv(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                1,
                value.to_array().as_ptr(),
            );
        }
    }

    pub fn set_uniform_3(&mut self, name: String, x: f32, y: f32, z: f32) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::Uniform3f(gl::GetUniformLocation(self.id, c_name.as_ptr()), x, y, z);
        }
    }

    pub fn set_uniform_4v(&mut self, name: String, value: glam::Vec4) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::Uniform4fv(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                1,
                value.to_array().as_ptr(),
            );
        }
    }

    pub fn set_uniform_4(&mut self, name: String, x: f32, y: f32, z: f32, w: f32) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::Uniform4f(gl::GetUniformLocation(self.id, c_name.as_ptr()), x, y, z, w);
        }
    }

    pub fn set_uniform_matrix_2(&mut self, name: String, value: glam::Mat2) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::UniformMatrix2fv(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                1,
                gl::FALSE,
                value.to_cols_array().as_ptr(),
            );
        }
    }

    pub fn set_uniform_matrix_3(&mut self, name: String, value: glam::Mat3) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::UniformMatrix3fv(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                1,
                gl::FALSE,
                value.to_cols_array().as_ptr(),
            );
        }
    }

    pub fn set_uniform_matrix_4(&mut self, name: String, value: glam::Mat4) {
        let c_name: CString = CString::new(name.clone()).unwrap();
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(self.id, c_name.as_ptr()),
                1,
                gl::FALSE,
                value.to_cols_array().as_ptr(),
            );
        }
    }
}
