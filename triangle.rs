#![no_uv]

extern crate glfw;
extern crate native;
extern crate hgl;
extern crate gl;
extern crate libc;
extern crate cgmath;

use std::mem::size_of;

use glfw::Context;
use hgl::{Shader, Program, Triangles, Vbo, Vao};

static VERTEX_SHADER: &'static str = "
#version 140

uniform float time;
uniform float rate;
uniform vec3 axis;
in vec3 position;
in vec3 color;
out vec3 Color;

mat4 rotationMatrix(vec3 axis, float angle)
{
    axis = normalize(axis);
    float s = sin(angle);
    float c = cos(angle);
    float oc = 1.0 - c;
    return mat4(oc * axis.x * axis.x + c,          oc * axis.x * axis.y - axis.z * s, oc * axis.z * axis.x + axis.y * s, 0.0,
                oc * axis.x * axis.y + axis.z * s, oc * axis.y * axis.y + c,          oc * axis.y * axis.z - axis.x * s, 0.0,
                oc * axis.z * axis.x - axis.y * s, oc * axis.y * axis.z + axis.x * s, oc * axis.z * axis.z + c,          0.0,
                0.0,                               0.0,                               0.0,                               1.0);
}

void main() {
    gl_Position = rotationMatrix(axis, time*rate) * vec4(position, 1.0);
    Color = color;
}";

static FRAGMENT_SHADER: &'static str = "
#version 140
out vec4 out_color;
in vec3 Color;

void main() {
    out_color = vec4(Color, 1.0);
}";

#[start]
fn main(argc: int, argv: **u8) -> int {
    native::start(argc, argv, proc() {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

        glfw.window_hint(glfw::ContextVersion(3, 1));

        let (window, events) = glfw.create_window(800, 600, "HGL", glfw::Windowed).unwrap();
        window.set_all_polling(true);

        window.make_current();
        gl::load_with(|p| glfw.get_proc_address(p));

        gl::Viewport(0, 0, 800, 600);

        let vao = Vao::new();
        vao.bind();
        let program = Program::link([Shader::compile(VERTEX_SHADER, hgl::VertexShader),
                                     Shader::compile(FRAGMENT_SHADER, hgl::FragmentShader)]).unwrap();
        program.bind_frag(0, "out_color");
        program.bind();

        let vbo = Vbo::from_data([ 0.0f32,  0.5, 0.0, 1.0, 0.0, 0.0,
                                   0.5,    -0.5, 0.0, 0.0, 1.0, 0.0,
                                  -0.5,    -0.5, 0.0, 0.0, 0.0, 1.0 ],
                                 hgl::StaticDraw);

        vao.enable_attrib(&program, "position", gl::FLOAT, 3, 6*size_of::<f32>() as i32, 0);
        vao.enable_attrib(&program, "color", gl::FLOAT, 3, 6*size_of::<f32>() as i32, 3*size_of::<f32>());
        vbo.bind();

        let mut frame: uint = 0;

        gl::Uniform1f(program.uniform("time"), frame as f32);
        gl::Uniform1f(program.uniform("rate"), 0.05f32);
        gl::Uniform3f(program.uniform("axis"), -1.0, 0.0, 1.0);

        while !window.should_close() {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            glfw.poll_events();
            for (time, event) in glfw::flush_messages(&events) {
              println!("{}: {}", time, event);
              match event {
                glfw::KeyEvent(glfw::KeyEscape, _, glfw::Press, _) => {
                  window.set_should_close(true)
                },
                glfw::KeyEvent(glfw::KeyKp1, _, glfw::Press, _) => gl::Uniform3f(program.uniform("axis"),  0.0,  0.0,  1.0),
                glfw::KeyEvent(glfw::KeyKp4, _, glfw::Press, _) => gl::Uniform3f(program.uniform("axis"),  0.0,  0.0, -1.0),
                glfw::KeyEvent(glfw::KeyKp2, _, glfw::Press, _) => gl::Uniform3f(program.uniform("axis"),  0.0,  1.0,  0.0),
                glfw::KeyEvent(glfw::KeyKp5, _, glfw::Press, _) => gl::Uniform3f(program.uniform("axis"),  0.0, -1.0,  0.0),
                glfw::KeyEvent(glfw::KeyKp3, _, glfw::Press, _) => gl::Uniform3f(program.uniform("axis"),  1.0,  0.0,  0.0),
                glfw::KeyEvent(glfw::KeyKp6, _, glfw::Press, _) => gl::Uniform3f(program.uniform("axis"), -1.0,  0.0,  0.0),
                glfw::SizeEvent(w, h) => {
                 window.set_title(format!("Time: {}, Window size: ({}, {})", time, w, h).as_slice());
                 gl::Viewport(0, 0, w, h);
                }
                glfw::KeyEvent(glfw::KeySpace, _, glfw::Press, _) => {
                  match window.get_cursor_mode() {
                    glfw::CursorDisabled => window.set_cursor_mode(glfw::CursorNormal),
                    glfw::CursorNormal => window.set_cursor_mode(glfw::CursorDisabled),
                    _ => {}
                  }
                },
                _ => {},
              }
            }

            gl::Uniform1f(program.uniform("time"), frame as f32);
            vao.draw_array(Triangles, 0, 3);
            window.swap_buffers();
            frame += 1;
        }
    });
    0
}
