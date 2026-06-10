use std::f32::consts::PI;
use glow::HasContext;
use sdl2::mouse::MouseState;

fn main() {
    // Init sdl
    let sdl = sdl2::init().unwrap();
    // Get video from sdl
    let video = sdl.video().unwrap();

    // Set up GL attributes
    let gl_attr = video.gl_attr();
    gl_attr.set_double_buffer(true);
    gl_attr.set_depth_size(24);
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(3, 3);

    let window = video
        .window("Rust with GL", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();
    println!("created window");

    // Create gl context on above window
    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();

    // Gets sdl2's event queue. Anything that happened from IO
    let mut event_pump = sdl.event_pump().unwrap();


    // Creates actual "GL". All GL calls will pass through this
    // Glow takes a string and passes it to sdl2
    // sdl2 returns a pointer to the function matching the string
    // Unsafe --> Opt out of Rust's memory safety guidelines
    // As OpenGL/C is not part of rust, the compiler can't check the safety of the calls
    // We have to make sure they are correct and safe ourselves
    let gl = unsafe {
        glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _)
    };
    println!("Created gl");

    //GPU log
    unsafe {
        let renderer = gl.get_parameter_string(glow::RENDERER);
        let vendor = gl.get_parameter_string(glow::VENDOR);
        let version = gl.get_parameter_string(glow::VERSION);
        println!("GPU Vendor: {}", vendor);
        println!("GPU Renderer: {}", renderer);
        println!("OpenGL Version: {}", version);
    }

    // Set window
    unsafe {
        gl.viewport(0, 0, 800, 600);
        gl.clear_color(0.1, 0.2, 0.3, 1.0);
    }

    let (program, vao, vbo) = unsafe {
        let program_2d = gl.create_program().unwrap();
        setup_triangle(&gl, program_2d)
    };

    let mut v1: f32 = 0.0;
    let mut v2: f32 = 2.0 * PI / 3.0;
    let mut v3: f32 = 4.0 * PI / 3.0;
    let mut mouse_down_x: f32 = 0.0;
    let mut mouse_down_y: f32 = 0.0;
    let mut offset_x: f32 = 0.0;
    let mut offset_y: f32 = 0.0;
    // 'main is a label for an infinite loop
    'main: loop {
        // Gets all pending events that happened since last frame
        for event in event_pump.poll_iter() {
            match event {
                // Pattern matching. This is looking for a user to "Quit" (exit window) and
                // breaks the loop 'main when matched
                // The _ => {} says when anything else is caught, do nothing
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::MouseButtonDown { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                    //offset_x += (x as f32 - mouse_down_x) / 400.0;
                    //offset_y += (y as f32 - mouse_down_y) / 300.0;
                    //mouse_down_x = x as f32;
                    //mouse_down_y = y as f32;
                }
                sdl2::event::Event::MouseButtonUp { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, ..} => {
                    // Divisions necessary for transition from pixel to GL NDC space
                    //offset_x += (x as f32 - mouse_down_x) / 400.0;
                    //offset_y += (y as f32 - mouse_down_y) / 300.0;
                }
                _ => {}
            }
        }

        // Relative calculates diff automatically
        // Mouse state directly needs your own calculation
        // Event mismatch/floating point calcs on direct mnouse state lead to errors
        // aka unwanted dragging
        let mouse = event_pump.relative_mouse_state();
        println!("{:?}", (mouse.x(), mouse.y()));
        if mouse.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
            offset_x += mouse.x() as f32 / 400.0;
            offset_y += mouse.y() as f32 / 300.0;
        }

        // Animation logic here
        v1 = v1.rem_euclid(2.0 * PI);
        v2 = v2.rem_euclid(2.0 * PI);
        v3 = v3.rem_euclid(2.0 * PI);
        let (model)
            = create_triangle_vertices(v1, v2, v3, &offset_x, &offset_y);
        //Ensure we maintain the correct posits
        println!("{}",model[0]);
        v1 += 0.01;
        v2 += 0.01;
        v3 += 0.01;

        // Set up the window rendering
        unsafe {
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.bind_vertex_array(Some(vao));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
            gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(&model),
                glow::DYNAMIC_DRAW,
            );
            gl.use_program(Some(program));
            gl.draw_arrays(glow::TRIANGLES, 0, 3); // Will need to be expanded
        }

        // OpenGL will render into a back buffer
        // This swaps the back buffer with the one we just wiped, rending our image
        // => double buffering
        window.gl_swap_window();
    }
}

fn create_triangle_vertices(mut v1: f32, mut v2: f32, mut v3: f32, x: &f32, y: &f32)
    -> ([f32; 6]) {

    let v1_x;
    let v2_x;
    let v3_x;
    let v1_y;
    let v2_y;
    let v3_y;

    // Calculate vertex position on circle
    v1_x = v1.sin() * 0.5 + x;
    v1_y = v1.cos() * 0.5 - y;
    v2_x = v2.sin() * 0.5 + x;
    v2_y = v2.cos() * 0.5 - y;
    v3_x = v3.sin() * 0.5 + x;
    v3_y = v3.cos() * 0.5 - y;

    //Sanity check: Get length of an edge
    println!("{}", ((v1_x - v2_x) * (v1_x - v2_x) + (v1_y - v2_y) * (v1_y - v2_y)).abs().sqrt());

    // Output vertex model
    let model: [f32; 6] = [
        v1_x, v1_y, // v1
        v2_x, v2_y, // v2
        v3_x, v3_y, // v3
    ];

    model
}

//unsafe fn setup_circle(gl: &glow::Context) -> (glow::NativeProgram, glow::NativeVertexArray, glow::NativeBuffer) {}
//    let vertex = r#"#version 330 core
//        layout(lovation=0) in vec2 pos;
//        void main() { gl_Position = vec4(pos, 0.0, 1.0); }
//       "#;
//}

unsafe fn setup_triangle(gl: &glow::Context, program: glow::NativeProgram) -> (glow::NativeProgram, glow::NativeVertexArray, glow::NativeBuffer) {
    //Same as shaders in C++
    // Vert shader
    let vertex = r#"#version 330 core
        layout(location = 0) in vec2 pos;
        void main() { gl_Position = vec4(pos, 0.0, 1.0); }
    "#;

    // Frag shader
    let fragment = r#"#version 330 core
        out vec4 color;
        void main() { color = vec4(1.0,0.5,0.2,1.0); }
    "#;

    for (src, shader_type) in [
        (vertex, glow::VERTEX_SHADER),
        (fragment, glow::FRAGMENT_SHADER),
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

    gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 8, 0);
    gl.enable_vertex_attrib_array(0);

    (program, vao, vbo)
}
