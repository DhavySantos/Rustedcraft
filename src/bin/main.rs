#![cfg_attr(debug_assertions, allow(dead_code, unused))]
use glfw::{Context, OpenGlProfileHint};
use glfw::{Key, Action, Modifiers};
use glfw::{WindowHint, Window};

use rustedcraft::opengl::{BufferTarget, BufferUsage, BufferObject};
use rustedcraft::opengl::{ShaderType, Shader};
use rustedcraft::opengl::VertexAttribPointer;
use rustedcraft::opengl::VertexArray;
use rustedcraft::opengl::Program;

use std::ffi::CString;
use std::ptr::null;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();

    glfw.window_hint(WindowHint::OpenGlProfile(OpenGlProfileHint::Core));
    glfw.window_hint(WindowHint::ContextVersion(3, 3));
    glfw.window_hint(WindowHint::Resizable(false));

    let (mut window, _) = glfw
        .create_window(640, 480, "RustedCraft", glfw::WindowMode::Windowed)
        .expect("Failed to create glfw window");

    gl::load_with(|s| window.get_proc_address(s));

    window.set_key_callback(handle_input);
    window.make_current();

    let vert_source = std::fs::read_to_string("assets/shaders/triangle.vert").unwrap();
    let triangle_vert = Shader::from_source(&vert_source, ShaderType::Vertex).unwrap();

    let frag_source = std::fs::read_to_string("assets/shaders/triangle.frag").unwrap();
    let triangle_frag = Shader::from_source(&frag_source, ShaderType::Fragment).unwrap();

    let shader_program = Program::new();

    shader_program.attach_shader(&triangle_vert);
    shader_program.attach_shader(&triangle_frag);
    shader_program.link();

    let uniform_name = CString::new("ourColor").unwrap();
    let uniform = unsafe { gl::GetUniformLocation(shader_program.id(), uniform_name.as_ptr()) };

    #[rustfmt::skip]
    let triangle: [f32; 18] = [
         0.5, -0.5, 0.0, 1.0, 0.0, 0.0,
        -0.5, -0.5, 0.0, 0.0, 1.0, 0.0,
         0.0,  0.5, 0.0, 0.0, 0.0, 1.0,
    ];

    #[rustfmt::skip]
    let indices: [u32; 3] = [
        0, 1, 2
    ];

    let vao = VertexArray::new();
    vao.bind();

    let vbo = BufferObject::new(BufferTarget::ArrayBuffer, BufferUsage::StaticDraw);
    let ebo = BufferObject::new(BufferTarget::ElementArrayBuffer, BufferUsage::StaticDraw);

    vbo.data(&triangle);
    ebo.data(&indices);

    let vap = VertexAttribPointer::new::<f32>(0, 3, false, 6, 0);
    let cap = VertexAttribPointer::new::<f32>(1, 3, false, 6, 3);

    vap.enable();
    cap.enable();

    while !window.should_close() {
        shader_program.use_program();

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::ClearColor(0.2, 0.2, 0.4, 1.0);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, null());
        };

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn handle_input(window: &mut Window, key: Key, scancode: i32, action: Action, mods: Modifiers) {
    match (key, action) {
        (Key::Escape, Action::Press) => window.set_should_close(true),
        _ => {}
    };
}
