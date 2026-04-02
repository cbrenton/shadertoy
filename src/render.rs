use glium::Surface;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate glium;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
}
implement_vertex!(Vertex, position);

pub struct RenderManager {
    pub display: glium::Display<glium::glutin::surface::WindowSurface>,
    pub programs: HashMap<String, ShaderProgram>,
    pub shader_path: String,
}

impl RenderManager {
    pub fn new(
        display: glium::Display<glium::glutin::surface::WindowSurface>,
        shader_path: &str,
    ) -> Self {
        Self {
            display,
            programs: HashMap::new(),
            // TODO: make sure there's a trailing slash
            shader_path: shader_path.to_string(),
        }
    }

    pub fn target(&mut self) -> glium::Frame {
        self.display.draw()
    }

    // TODO: default to hello world
    pub fn load_shader(&mut self, shader_name: &str, vert_name: &str, frag_name: &str) {
        let vert_src = self.read_shader_file(&format!("{}/{}.glsl", self.shader_path, vert_name));
        let frag_src = self.read_shader_file(&format!("{}/{}.glsl", self.shader_path, frag_name));

        let positions = [[-0.5, -0.5], [0.0, 0.5], [0.5, -0.25]];
        let mut tris = vec![];
        for position in positions {
            tris.push(Vertex { position });
        }
        let vertex_buffer = glium::VertexBuffer::new(&self.display, &tris).unwrap();

        let program = self.create_program(&vert_src, &frag_src);

        let shader_program = ShaderProgram::new(program, vertex_buffer);

        self.programs
            .insert(shader_name.to_string(), shader_program);
    }

    fn create_program(&mut self, vert_src: &str, frag_src: &str) -> glium::Program {
        glium::Program::from_source(&self.display, vert_src, frag_src, None).unwrap()
    }

    fn read_shader_file(&mut self, filepath: &str) -> String {
        // create a path to the desired file
        let path = Path::new(filepath);
        let path_display = path.display();

        // open path in RO mode, returns `io::Result<File>`
        let mut file = match File::open(path) {
            Err(why) => panic!("couldn't open {}: {}", path_display, why),
            Ok(file) => file,
        };

        // read file contents into a string, returns `io::Result<usize>`
        let mut src = String::new();
        match file.read_to_string(&mut src) {
            Err(why) => panic!("couldn't read {}: {}", path_display, why),
            Ok(_) => src,
        }
    }

    pub fn draw_frame(&mut self) {
        let mut target = self.target();
        target.clear_color(0.0, 0.0, 1.0, 1.0);

        for (name, program) in &self.programs {
            println!("drawing {name}");
            target
                .draw(
                    &program.vertex_buffer,
                    glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList),
                    &program.program,
                    &glium::uniforms::EmptyUniforms,
                    &Default::default(),
                )
                .unwrap();
        }
        target.finish().unwrap();
    }

    pub fn resize(&mut self, new_size: glium::winit::dpi::PhysicalSize<u32>) {
        self.display.resize(new_size.into())
    }
}

pub struct ShaderProgram {
    pub vertex_buffer: glium::VertexBuffer<Vertex>,
    pub program: glium::Program,
}

impl ShaderProgram {
    pub fn new(program: glium::Program, vertex_buffer: glium::VertexBuffer<Vertex>) -> Self {
        Self {
            program,
            vertex_buffer,
        }
    }
}
