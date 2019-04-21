extern crate gl;
extern crate sdl2;

use std::ffi::CString;

#[macro_use]
mod util;

mod types;

use types::{GLColor, GLRect, Shader};

const CLEAR_COLOR: GLColor = GLColor {
    r: 0.7,
    g: 0.0,
    b: 0.7,
    a: 1.0,
};

const VIEWPORT: GLRect = GLRect {
    x0: 0,
    y0: 0,
    x1: 800,
    y1: 600,
};

fn main() -> Result<(), String> {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let window = video_subsystem
        .window("OpenGL", VIEWPORT.width(), VIEWPORT.height())
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let _opengl_context = window.gl_create_context().unwrap();
    initialize_opengl(&video_subsystem);

    let source_string = CString::new("foobar").unwrap();

    let vert_shader = Shader::vert_from_source(&source_string)?;
    let frag_shader = Shader::frag_from_source(&source_string)?;

    let program_id = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program_id, vert_shader.id());
        gl::AttachShader(program_id, frag_shader.id());
        gl::LinkProgram(program_id);
    }

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'main;
            }
            // Handle user input
        }

        clear_screen();
        window.gl_swap_window();
    }

    Ok(())
}

fn initialize_opengl(video: &sdl2::VideoSubsystem) {
    let gl_attr = video.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    gl::load_with(|s| video.gl_get_proc_address(s) as *const std::os::raw::c_void);

    unsafe {
        // Set the viewport to match the window
        gl::Viewport(VIEWPORT.x0, VIEWPORT.y0, VIEWPORT.x1, VIEWPORT.y1);

        // Set GL "clear" color
        gl::ClearColor(CLEAR_COLOR.r, CLEAR_COLOR.g, CLEAR_COLOR.b, CLEAR_COLOR.a);
    }
}

fn clear_screen() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
