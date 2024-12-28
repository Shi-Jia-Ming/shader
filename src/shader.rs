use std::{ffi::CString, ptr};

use gl::types::{GLchar, GLint, GLuint};

// 通过读取着色器文件来加载 GLSL 代码
pub fn load_shader_from_file(filename: &str) -> String {
    use std::fs;
    fs::read_to_string(filename).expect("unable to read shader file")
}

// 编译着色器
pub fn compile_shader(shader_source: &str, shader_type: GLuint) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(shader_type);
        let cstr = CString::new(shader_source.as_bytes()).unwrap();
        // unsafe transmute
        gl::ShaderSource(shader, 1, &cstr.as_ptr(), ptr::null_mut());
        gl::CompileShader(shader);

        let mut success: GLint = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

        if success == 0 {
            let mut len: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = vec![0; len as usize];
            gl::GetShaderInfoLog(shader, len, &mut len, buffer.as_mut_ptr() as *mut GLchar);
            println!(
                "ERROR::SHADER::COMPILATION_FAILED\n{}",
                String::from_utf8(buffer).unwrap()
            );
        }

        shader
    }
}

pub fn link_program(vertex_shader: GLuint, fragment_shader: GLuint) -> GLuint {
    unsafe {
        let program = gl::CreateProgram();
        gl::AttachShader(program, vertex_shader);
        gl::AttachShader(program, fragment_shader);
        gl::LinkProgram(program);

        let mut success: GLint = 0;
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

        if success == 0 {
            let mut len: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);

            let mut buffer: Vec<u8> = vec![0; len as usize];
            gl::GetProgramInfoLog(program, len, &mut len, buffer.as_mut_ptr() as *mut GLchar);
            println!(
                "ERROR::SHADER::PROGRAM::LINK_FAILED\n{}",
                String::from_utf8(buffer).unwrap()
            );
        }

        program
    }
}
