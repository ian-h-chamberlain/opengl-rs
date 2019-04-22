extern crate gl;

use std::ffi::CStr;

pub use gl::types::{GLboolean, GLchar, GLenum, GLfloat, GLint, GLsizei, GLuint};

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
        let create_shader = || unsafe {
            let id = gl::CreateShader(shader_type);
            gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
            gl::CompileShader(id);
            id
        };

        let id = create_gl_object!(
            create_shader,
            gl::GetShaderiv,
            gl::GetShaderInfoLog,
            gl::COMPILE_STATUS
        )?;

        println!("Created GL shader");

        Ok(Self { id })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteShader(self.id) };
    }
}

pub struct Program {
    id: GLuint,
}

impl Program {
    pub fn set_used(&self) {
        unsafe { gl::UseProgram(self.id) };
    }

    pub fn id(&self) -> GLuint {
        self.id
    }

    pub fn from_shaders(shaders: &[Shader]) -> Result<Self, String> {
        let id = create_gl_object!(
            gl::CreateProgram,
            gl::GetProgramiv,
            gl::GetProgramInfoLog,
            gl::LINK_STATUS
        )?;

        println!("Created GL Program");

        for shader in shaders {
            unsafe { gl::AttachShader(id, shader.id()) };
        }

        unsafe { gl::LinkProgram(id) };

        Ok(Self { id })
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.id) };
    }
}
