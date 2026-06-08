use glow::HasContext;

fn main() {
    // Init sdl
    let sdl = sdl2::init().unwrap();
    // Get video from sdl
    let video = sdl.video().unwrap();

    // Set up GL attributes
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video
        .window("glow demo", 800, 600)
        .opengl()
        .build()
        .unwrap();

    // Create gl context on above window
    let _gl_context = window.gl_create_context().unwrap();

    // Creates actual "GL". All GL calls will pass through this
    // Glow takes a string and passes it to sdl2
    // sdl2 returns a pointer to the function matching the string
    // Unsafe --> Opt out of Rust's memory safety guidelines
    // As OpenGL/C is not part of rust, the compiler can't check the safety of the calls
    // We have to make sure they are correct and safe ourselves
    let gl = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };

    let (program, vao) = unsafe { setup_triangle(&gl) };

    unsafe {
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
    }

    // Gets sdl2's event queue. Anything that happened from IO
    let mut event_pump = sdl.event_pump().unwrap();
    // 'main is a label for an infinite loop
    //
    'main: loop {
        // Gets all pending events that happened since last frame
        for event in event_pump.poll_iter() {
            match event {
                // Pattern matching. This is looking for a user to "Quit" (exit window) and
                // breaks the loop 'main when matched
                // The _ => {} says when anything else is caught, do nothing
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
        // Set up the new windonw
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(vao));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
        // OpenGL will render into a back buffer
        // This swaps the back buffer with the one we just wiped, rending our image
        // => double buffering
        window.gl_swap_window();
    }
}

unsafe fn setup_triangle(gl: &glow::Context) -> (glow::NativeProgram, glow::NativeVertexArray) {
    //Same as shaders in C++
    // Vert shader
    let vs_src = r#"#version 330 core
        layout(location = 0) in vec2 pos;
        void main() { gl_Position = vec4(pos, 0.0, 1.0); }
    "#;

    // Frag shader
    let fs_src = r#"#version 330 core
        out vec4 color;
        void main() { color = vec4(1.0,0.5,0.2,1.0); }
    "#;

    let program = gl.create_program().unwrap();
    for (src, shader_type) in [
        (vs_src, glow::VERTEX_SHADER),
        (fs_src, glow::FRAGMENT_SHADER),
    ] {
        let shader = gl.create_shader(shader_type).unwrap();
        gl.shader_source(shader, src);
        gl.compile_shader(shader);
        assert!(
            gl.get_shader_compile_status(shader),
            "{}",
            gl.get_shader_info_log(shader)
        );
        gl.attach_shader(program, shader);
        gl.delete_shader(shader);
    }
    gl.link_program(program);
    assert!(
        gl.get_program_link_status(program),
        "{}",
        gl.get_program_info_log(program)
    );

    let vao = gl.create_vertex_array().unwrap();
    gl.bind_vertex_array(Some(vao));

    let vbo = gl.create_buffer().unwrap();
    gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));

    let vertices: &[f32] = &[-0.5, -0.5, 0.5, -0.5, 0.0, 0.5];
    unsafe {
        gl.buffer_data_u8_slice(
            glow::ARRAY_BUFFER,
            bytemuck::cast_slice(vertices),
            glow::STATIC_DRAW,
        );
    }

    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);
    gl.enable_vertex_attrib_array(0);

    (program, vao)
}
