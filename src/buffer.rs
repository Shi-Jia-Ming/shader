use std::ptr;

// 创建帧缓冲区
pub unsafe  fn create_frame_buffer() -> (u32, u32) {
    let mut frame_buffer: u32 = 0;

    gl::GenFramebuffers(1, &mut frame_buffer);
    gl::BindFramebuffer(gl::FRAMEBUFFER, frame_buffer);

    let mut texture_color_buffer: u32 = 0;
    gl::GenTextures(1, &mut texture_color_buffer);
    gl::BindTexture(gl::TEXTURE_2D, texture_color_buffer);

    // 创建棋盘纹理数据
    let width = 800;
    let height = 600;
    let mut texture_data: Vec<u8> = vec![0; width * height * 3];
    for y in 0..height {
        for x in 0..width {
            let offset = (y * width + x) * 3;
            let color = if (x / 100 + y / 100) % 2 == 0 { 255 } else { 0 };
            texture_data[offset] = color;
            texture_data[offset + 1] = color;
            texture_data[offset + 2] = color;
        }
    }

    gl::TexImage2D(
        gl::TEXTURE_2D,
        0,
        gl::RGB as i32,
        width.try_into().unwrap(),
        height.try_into().unwrap(),
        0,
        gl::RGB,
        gl::UNSIGNED_BYTE,
        texture_data.as_ptr() as *const std::ffi::c_void,
        // std::ptr::null(),
    );

    // 设置纹理参数
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

    gl::FramebufferTexture2D(
        gl::FRAMEBUFFER,
        gl::COLOR_ATTACHMENT0,
        gl::TEXTURE_2D,
        texture_color_buffer,
        0,
    );

    assert!(gl::CheckFramebufferStatus(gl::FRAMEBUFFER) == gl::FRAMEBUFFER_COMPLETE);

    (frame_buffer, texture_color_buffer)
}

pub unsafe fn create_line() -> u32 {
    let vertices: [f32; 24] = [
        // positions       // colors
        0.5,  0.5, 0.0,   1.0, 0.0, 0.0,  // top right
        0.5, -0.5, 0.0,   0.0, 1.0, 0.0,  // bottom right
       -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,  // bottom left
       -0.5,  0.5, 0.0,   1.0, 1.0, 0.0   // top left 
    ];

    let indices: [u32; 6] = [
        0, 1, 3,  // first triangle
        1, 2, 3   // second triangle
    ];

    // 创建顶点数组对象
    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ebo: u32 = 0;

    // 创建顶点缓冲对象
    unsafe {
        gl::GenBuffers(1, &mut vbo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
    }

    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        gl::BindVertexArray(vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<f32>()) as isize,
            vertices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );

        gl::VertexAttribPointer(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * std::mem::size_of::<f32>() as i32,
            ptr::null(),
        );
        gl::EnableVertexAttribArray(0);

        gl::VertexAttribPointer(
            1, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            6 * std::mem::size_of::<f32>() as gl::types::GLsizei, 
            (3 * std::mem::size_of::<f32>()) as *const _
        );
        gl::EnableVertexAttribArray(1);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }

    vao
}