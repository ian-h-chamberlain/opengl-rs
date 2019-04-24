extern crate gl;
extern crate sdl2;

use std::ffi::{CStr, CString};

use sdl2::{event::Event, keyboard::Keycode};

#[macro_use]
mod util;
mod types;

use types::{GLColor, GLRect, Program, Shader};

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

    let (window, _opengl_context) = initialize_opengl(&video_subsystem);

    let vert_shader =
        Shader::vert_from_source(&CString::new(include_str!("data/triangle.vert")).unwrap())?;
    let frag_shader =
        Shader::frag_from_source(&CString::new(include_str!("data/triangle.frag")).unwrap())?;

    let gl_program = Program::from_shaders(&[vert_shader, frag_shader])?;
    gl_program.set_used();

    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main,
                _ => {} // TODO Handle user input
            }
        }

        clear_screen();
        window.gl_swap_window();
    }

    Ok(())
}

fn initialize_opengl(
    video: &sdl2::VideoSubsystem,
) -> (sdl2::video::Window, sdl2::video::GLContext) {
    let gl_attr = video.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    println!("GL context version: {:?}", gl_attr.context_version());

    let window = video
        .window("OpenGL", VIEWPORT.width(), VIEWPORT.height())
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let opengl_context = window.gl_create_context().unwrap();
    gl::load_with(|name| video.gl_get_proc_address(name) as *const _);

    unsafe {
        // Set the viewport to match the window
        gl::Viewport(VIEWPORT.x0, VIEWPORT.y0, VIEWPORT.x1, VIEWPORT.y1);
        // Set GL "clear" color
        gl::ClearColor(CLEAR_COLOR.r, CLEAR_COLOR.g, CLEAR_COLOR.b, CLEAR_COLOR.a);
    }

    let version = unsafe { CStr::from_ptr(gl::GetString(gl::VERSION) as *const _) };
    println!("OpenGL version string: {}", version.to_str().unwrap());

    (window, opengl_context)
}

fn clear_screen() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}
