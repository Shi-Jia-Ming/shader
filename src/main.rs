use buffer::create_frame_buffer;

extern crate gl;
extern crate glutin;

mod buffer;
mod shader;
mod window;


fn main() {
    let (event_loop, windowed_context) = window::create_window();

    event_loop.run(move |event, _, control_flow| {
        
        *control_flow = glutin::event_loop::ControlFlow::Wait;

        // 读取并编译着色器
        let vertex_shader_source = shader::load_shader_from_file("src/vertex_shader.glsl");
        let fragment_shader_source = shader::load_shader_from_file("src/fragment_shader.glsl");

        let vertex_shader = shader::compile_shader(&vertex_shader_source, gl::VERTEX_SHADER);
        let fragment_shader = shader::compile_shader(&fragment_shader_source, gl::FRAGMENT_SHADER);

        // 链接着色器
        let shader_program = shader::link_program(vertex_shader, fragment_shader);

        // 初始化顶点数据
        let vao = unsafe { buffer::create_line() };

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                // 按 Esc 键退出
                glutin::event::WindowEvent::KeyboardInput { input, .. } => {
                    if let glutin::event::KeyboardInput {
                        state: glutin::event::ElementState::Released,
                        virtual_keycode: Some(glutin::event::VirtualKeyCode::Escape),
                        ..
                    } = input {
                        *control_flow = glutin::event_loop::ControlFlow::Exit;
                    }
                },
                _ => (),
            },
            glutin::event::Event::MainEventsCleared => {
                unsafe { 

                    gl::BindVertexArray(vao);
                    gl::ClearColor(0.7, 0.7, 0.1, 0.0);
                    gl::Clear(gl::COLOR_BUFFER_BIT);

                    gl::UseProgram(shader_program);
                
                    gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
                }

                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    })
}

unsafe fn draw_event(vao: u32, shader_program: u32) {
    gl::BindVertexArray(vao);
    gl::ClearColor(0.7, 0.7, 0.1, 0.0);
    gl::Clear(gl::COLOR_BUFFER_BIT);

    gl::UseProgram(shader_program);
    
    gl::DrawArrays(gl::TRIANGLE_FAN, 0, 4);
}