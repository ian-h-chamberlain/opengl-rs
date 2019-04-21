extern crate gl;

use std::ffi::CStr;

use super::util;

pub use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLuint};

pub struct GLColor {
    pub r: GLfloat,
    pub g: GLfloat,
    pub b: GLfloat,
    pub a: GLfloat,
}

pub struct GLRect {
    pub x0: GLint,
    pub y0: GLint,
    pub x1: GLint,
    pub y1: GLint,
}

impl GLRect {
    pub fn width(&self) -> u32 {
        (self.x1 - self.x0) as u32
    }

    pub fn height(&self) -> u32 {
        (self.y1 - self.y0) as u32
    }
}

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn frag_from_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::FRAGMENT_SHADER)
    }

    pub fn vert_from_source(source: &CStr) -> Result<Self, String> {
        Self::from_source(source, gl::VERTEX_SHADER)
    }

    pub fn from_source(source: &CStr, shader_type: GLenum) -> Result<Self, String> {
        let id = unsafe { gl::CreateShader(shader_type) };

        unsafe {
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
        }

        #[allow(clippy::cast_lossless)]
        let compile_success =
            call_output_fn!(gl::GetShaderiv, gl::FALSE as GLint, id, gl::COMPILE_STATUS)
                as GLboolean;

        if compile_success as GLboolean != gl::TRUE {
            let len = call_output_fn!(gl::GetShaderiv, 0, id, gl::INFO_LOG_LENGTH);
            let error = util::space_cstring_from_size(len as usize);

            unsafe {
                gl::GetShaderInfoLog(id, len, std::ptr::null_mut(), error.as_ptr() as *mut GLchar);
            }

            Err(error.to_string_lossy().to_string())
        } else {
            Ok(Shader { id })
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}
