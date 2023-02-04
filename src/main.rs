use sdl2::video::Window;
use std::ffi::{CString, CStr};
use gl::types::*;
use rendering::*;

pub mod rendering;

extern crate sdl2;
extern crate gl;

fn main() {
    // initialization: window and opengl
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Compatibility);
    gl_attr.set_context_version(3, 3);
    let mut event_pump = sdl.event_pump().unwrap();
    let window: Window =  video_subsystem
                            .window("Game", 900, 700)
                            .opengl()
                            .resizable()
                            .build()
                            .unwrap();
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    unsafe {
        gl::Viewport(0, 0, 900, 700);
        gl::ClearColor(0.0, 0.0, 0.9, 1.0);
    }

    // init shader
    let vert_shader = Shader::from_vert_source(
        &CString::new(include_str!("triangle.vert")).unwrap()
    ).unwrap();
    
    let frag_shader = Shader::from_frag_source(
        &CString::new(include_str!("triangle.frag")).unwrap()
    ).unwrap();
    let shader_program = Program::from_shaders(
        &[vert_shader, frag_shader]
    ).unwrap();

    shader_program.set_used();

    // Send triangle data to graphics driver
    let vertices: Vec<f32> = vec![
        -0.1, 1.0, 0.0,
        0.1, 1.0, 0.0,
        -0.1, 0.0, 0.0,

        0.1, 0.0, 0.0,
        0.1, 1.0, 0.0,
        -0.1, 0.0, 0.0,
    ];
    let mut vbo: GLuint = 0;
    //let mut vao: GLuint = 123123123;
    unsafe {
        println!("1 vbo: {:?}", vbo);
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        println!("2 vbo: {:?}", vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, // target 
            (vertices.len() * std::mem::size_of::<f32>()) as GLsizeiptr, // size of data in bytes
            vertices.as_ptr() as *const GLvoid, // pointer to data
            gl::STATIC_DRAW, // usage
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0); // unbind the buffer


        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::EnableVertexAttribArray(28); // this is "layout (location = 0)" in vertex shader
        gl::VertexAttribPointer(
            28, // index of the generic vertex attribute ("layout (location = 0)")
            3, // the number of components per generic vertex attribute
            gl::FLOAT, // data type
            gl::FALSE, // normalized (int-to-float conversion)
            // (4 * std::mem::size_of::<f32>()) as GLint, // stride (byte offset between consecutive attributes)
            0,
            std::ptr::null() // offset of the first component
        );
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    'main: loop {



        // handle events
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {},
            }
        }

        // drawing
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            // gl::BindVertexArray(vao);
            gl::DrawArrays(
                gl::TRIANGLES, // mode
                0, // starting index in the enabled arrays
                6 // number of indices to be rendered
            );
        }

                
        
        window.gl_swap_window();
    }

}