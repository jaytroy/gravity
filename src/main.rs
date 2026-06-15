mod planet;
mod triangle;
use triangle::*;

use std::f32::consts::PI;
use glow::{HasContext, NativeUniformLocation};

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

    let v1: f32 = 0.0;
    let v2: f32 = 2.0 * PI / 3.0;
    let v3: f32 = 4.0 * PI / 3.0;
    let mut triangle1: Triangle = Triangle::new(v1, v2, v3);
    let mut triangle2: Triangle = Triangle::new(v3, v1, v2);

    let mut model: Vec<f32> = Vec::new();
    let inc: f32 = 0.01;

    let mut offset_x: f32 = 0.0;
    let mut offset_y: f32 = 0.0;

    //Gets color location of frag shadeer
    let color_loc = unsafe { gl.get_uniform_location(program, "uColor") };

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
                sdl2::event::Event::MouseButtonUp { mouse_btn: sdl2::mouse::MouseButton::Left, x, y, .. } => {
                    // Divisions necessary for transition from pixel to GL NDC space
                    //offset_x += (x as f32 - mouse_down_x) / 400.0;
                    //offset_y += (y as f32 - mouse_down_y) / 300.0;
                }
                _ => {}
            }
        }

        // Relative mouse calculates diff automatically
        // Mouse state (directly) needs your own calculation
        // Event mismatch/floating point calcs on direct mo use state lead to errors
        // aka unwanted dragging
        let mouse = event_pump.relative_mouse_state();
        println!("{:?}", (mouse.x(), mouse.y()));
        if mouse.is_mouse_button_pressed(sdl2::mouse::MouseButton::Left) {
            offset_x += mouse.x() as f32 / 400.0;
            offset_y += mouse.y() as f32 / 300.0;
        }

        // Animation logic here
        triangle1.clamp_euclid();
        triangle2.clamp_euclid();

        model.extend(create_triangle_vertices(&triangle1, &offset_x, &offset_y)); //is this thread safe? probably, cause it's not async
        model.extend(create_triangle_vertices(&triangle2, &0.0, &0.0));

        triangle1.rotate(inc);
        triangle2.rotate(-inc);

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

            //offset starts at 0 => draw 1st triangle
            gl.uniform_3_f32(color_loc.as_ref(), 0.0, 0.0, 1.0);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
            //offset starts at 3 => draw 2nd triangle
            gl.uniform_3_f32(color_loc.as_ref(), 1.0, 0.0, 0.0);
            gl.draw_arrays(glow::TRIANGLES, 3, 3);
        }

        // OpenGL will render into a back buffer
        // This swaps the back buffer with the one we just wiped, rending our image
        // => double buffering
        window.gl_swap_window();

        //Cleanup
        model.clear();
    }
}