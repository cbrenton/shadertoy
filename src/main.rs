use glium::Surface;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

#[macro_use]
extern crate glium;
fn main() {
    let event_loop = glium::winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let positions = [[-0.5, -0.5], [0.0, 0.5], [0.5, -0.25]];
    let mut tris = vec![];
    for position in positions {
        tris.push(Vertex { position });
    }

    // upload vertices to vertex buffer
    let vertex_buffer = glium::VertexBuffer::new(&display, &tris).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    // read vert shader from file

    // create a path to the desired file
    let path = Path::new("./resources/shaders/hello_vert.glsl");
    let path_display = path.display();

    // open path in RO mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path_display, why),
        Ok(file) => file,
    };

    // read file contents into a string, returns `io::Result<usize>`
    let mut vert_shader_src = String::new();
    match file.read_to_string(&mut vert_shader_src) {
        Err(why) => panic!("couldn't read {}: {}", path_display, why),
        Ok(_) => (),
    };

    let frag_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
"#;

    let program =
        glium::Program::from_source(&display, &vert_shader_src, frag_shader_src, None).unwrap();

    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    target
        .draw(
            &vertex_buffer,
            &indices,
            &program,
            &glium::uniforms::EmptyUniforms,
            &Default::default(),
        )
        .unwrap();
    target.finish().unwrap();

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                _ => (),
            },
            _ => (),
        };
    });
}
