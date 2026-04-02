use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

extern crate glium;

pub struct ShaderManager {
    pub display: glium::Display<glium::glutin::surface::WindowSurface>,
    pub programs: Vec<glium::Program>,
    pub shader_path: String,
}

impl ShaderManager {
    pub fn new(
        display: glium::Display<glium::glutin::surface::WindowSurface>,
        shader_path: &str,
    ) -> Self {
        Self {
            display,
            programs: Default::default(),
            // TODO: make sure there's a trailing slash
            shader_path: shader_path.to_string(),
        }
    }

    pub fn target(&mut self) -> glium::Frame {
        self.display.draw()
    }

    // TODO: default to hello world
    pub fn load_shader(&mut self, vert_name: &str, frag_name: &str) {
        let vert_src = self.read_shader_file(&format!("{}/{}.glsl", self.shader_path, vert_name));
        let frag_src = self.read_shader_file(&format!("{}/{}.glsl", self.shader_path, frag_name));

        let program = self.create_program(&vert_src, &frag_src);
        self.programs.push(program);
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
}
