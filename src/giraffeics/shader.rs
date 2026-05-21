use std::{ffi::CString, str::FromStr};

use gl::types::{GLenum, GLfloat, GLuint};
use nalgebra::Vector3;

enum ShaderType {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize,
}

struct Shader(GLuint);
impl Shader {
    fn from_source(source: &str, ty: ShaderType) -> Result<Self, String> {
        let shader = unsafe { gl::CreateShader(ty as GLenum) };
        if shader == 0 {
            return Err("couldn't allocate shader".into());
        }
        let shader = Self(shader);

        unsafe {
            gl::ShaderSource(
                shader.0,
                1,
                &(source.as_bytes().as_ptr().cast()),
                &(source.len().try_into().unwrap()),
            );
            gl::CompileShader(shader.0);
            let mut compiled = 0;
            gl::GetShaderiv(shader.0, gl::COMPILE_STATUS, &mut compiled);
            if compiled != i32::from(gl::TRUE) {
                let out =
                    format!("shader compile error: {}", shader.get_info_log());
                shader.delete();
                return Err(out);
            }

            Ok(shader)
        }
    }

    fn get_info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe {
            gl::GetShaderiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len)
        };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            gl::GetShaderInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    fn delete(self) {
        unsafe { gl::DeleteShader(self.0) };
    }
}

pub struct ShaderProgram(GLuint);
impl ShaderProgram {
    pub fn from_vert_frag(vert: &str, frag: &str) -> Result<Self, String> {
        let program = unsafe { gl::CreateProgram() };
        if program == 0 {
            return Err("couldn't allocate program".into());
        };

        let program = ShaderProgram(program);

        let vert_shader = Shader::from_source(vert, ShaderType::Vertex)?;
        let frag_shader = Shader::from_source(frag, ShaderType::Fragment)?;

        unsafe {
            gl::AttachShader(program.0, vert_shader.0);
            gl::AttachShader(program.0, frag_shader.0);

            gl::LinkProgram(program.0);

            vert_shader.delete();
            frag_shader.delete();

            let mut success = 0;
            gl::GetProgramiv(program.0, gl::LINK_STATUS, &mut success);
            if success != i32::from(gl::TRUE) {
                let out =
                    format!("program link error: {}", program.get_info_log());
                program.delete();
                return Err(out);
            }

            Ok(program)
        }
    }

    fn get_info_log(&self) -> String {
        let mut needed_len = 0;
        unsafe {
            gl::GetProgramiv(self.0, gl::INFO_LOG_LENGTH, &mut needed_len)
        };
        let mut v: Vec<u8> = Vec::with_capacity(needed_len.try_into().unwrap());
        let mut len_written = 0_i32;
        unsafe {
            gl::GetProgramInfoLog(
                self.0,
                v.capacity().try_into().unwrap(),
                &mut len_written,
                v.as_mut_ptr().cast(),
            );
            v.set_len(len_written.try_into().unwrap());
        }
        String::from_utf8_lossy(&v).into_owned()
    }

    fn delete(self) {
        unsafe { gl::DeleteProgram(self.0) };
    }

    pub fn use_program(&self) {
        unsafe { gl::UseProgram(self.0) };
    }

    pub fn set_mat4(&self, name: &str, mat: *const GLfloat) {
        unsafe {
            gl::UniformMatrix4fv(
                gl::GetUniformLocation(
                    self.0,
                    CString::from_str(name).unwrap().as_ptr(),
                ),
                1,
                gl::FALSE,
                mat,
            );
        }
    }

    pub fn set_vec3(&self, name: &str, vec: Vector3<f32>) {
        unsafe {
            gl::Uniform3f(
                gl::GetUniformLocation(
                    self.0,
                    CString::from_str(name).unwrap().as_ptr(),
                ),
                vec.x,
                vec.y,
                vec.z,
            );
        }
    }

    pub fn set_float(&self, name: &str, f: f32) {
        unsafe {
            gl::Uniform1f(
                gl::GetUniformLocation(
                    self.0,
                    CString::from_str(name).unwrap().as_ptr(),
                ),
                f,
            )
        }
    }
}
