use std::f32::consts::PI;
use glow::HasContext;

//give it getters?
pub struct Triangle {
    id: i8,
    pub v1: f32,
    pub v2: f32,
    pub v3: f32,
    pub is_selected: bool,
}

impl Triangle {
    pub fn new(v1: f32, v2: f32, v3: f32) -> Triangle {
        let id = 0;
        let is_selected = false;

        let mut vert: Vec<f32> = Vec::new();
        vert.push(v1);
        vert.push(v2);
        vert.push(v3);
        check_vertices(&vert);

        Triangle {
            id,
            v1,
            v2,
            v3,
            is_selected,
        }
    }


    //Ensures that vertices don't drift
    pub fn clamp_euclid(&mut self) {
        self.v1 = self.v1.rem_euclid(2.0 * PI);
        self.v2 = self.v2.rem_euclid(2.0 * PI);
        self.v3 = self.v3.rem_euclid(2.0 * PI);
    }

    
}

fn check_vertices(vertices: &Vec<f32>) {
    for &v in vertices {
         if v < 0.0 {
             panic!("Vertex is negative");
         }
    }
}

pub fn create_triangle_vertices(triangle: &Triangle, x: &f32, y: &f32) -> Vec<f32> {
    let v1_x;
    let v2_x;
    let v3_x;
    let v1_y;
    let v2_y;
    let v3_y;

    // Calculate vertex position on circle
    v1_x = triangle.v1.sin() * 0.5 + x;
    v1_y = triangle.v1.cos() * 0.5 - y;
    v2_x = triangle.v2.sin() * 0.5 + x;
    v2_y = triangle.v2.cos() * 0.5 - y;
    v3_x = triangle.v3.sin() * 0.5 + x;
    v3_y = triangle.v3.cos() * 0.5 - y;

    //Sanity check: Get length of an edge
    println!(
        "{}",
        ((v1_x - v2_x) * (v1_x - v2_x) + (v1_y - v2_y) * (v1_y - v2_y))
            .abs()
            .sqrt()
    );

    // Output vertex model

    let mut model: Vec<f32> = Vec::new();
    model.extend_from_slice(&[v1_x, v1_y, v2_x, v2_y, v3_x, v3_y]);
    // model.extend_from_slice(&[v4_x, v4_y, v5_x, v5_y, v6_x, v6_y]);

    model
}

//unsafe fn setup_circle(gl: &glow::Context) -> (glow::NativeProgram, glow::NativeVertexArray, glow::NativeBuffer) {}
//    let vertex = r#"#version 330 core
//        layout(lovation=0) in vec2 pos;
//        void main() { gl_Position = vec4(pos, 0.0, 1.0); }
//       "#;
//}

    pub unsafe fn setup_triangle(
        gl: &glow::Context,
        program: glow::NativeProgram,
    ) -> (
        glow::NativeProgram,
        glow::NativeVertexArray,
        glow::NativeBuffer,
    ) {
        //Same as shaders in C++
        // Vert shader
        let vertex = r#"#version 330 core
            layout(location = 0) in vec2 pos;
            void main() { gl_Position = vec4(pos, 0.0, 1.0); }
        "#;

        // Frag shader
        let fragment = r#"#version 330 core
            out vec4 FragColor;
            uniform vec3 uColor;
            void main() { FragColor = vec4(uColor, 1.0); }
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
