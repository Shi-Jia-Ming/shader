use glutin::event_loop::EventLoop;

// 创建一个全屏窗口
pub fn create_window() -> (EventLoop<()>, glutin::ContextWrapper<glutin::PossiblyCurrent, glutin::window::Window>) {
    let event_loop = EventLoop::new();

    let window_builder = glutin::window::WindowBuilder::new()
    .with_title("OpenGL in Rust");
    // .with_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)));

    let windowed_context = glutin::ContextBuilder::new()
    .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 6)))
    .with_gl_profile(glutin::GlProfile::Core)
    .with_vsync(true)
    .build_windowed(window_builder, &event_loop).unwrap();

    let windowed_context = unsafe {
        let windowed_context = windowed_context.make_current().unwrap();
        gl::load_with(|ptr| windowed_context.get_proc_address(ptr) as *const _);

        // 设置全屏模式
        windowed_context.window().set_fullscreen(Some(glutin::window::Fullscreen::Borderless(None)));

        windowed_context
    };

    (event_loop, windowed_context)
}