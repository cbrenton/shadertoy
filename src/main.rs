use glium::Surface;

mod render;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

#[macro_use]
extern crate glium;
fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let mut shader_mgr = render::ShaderManager::new(display, "./resources/shaders/");
    shader_mgr.load_shader("hello_vert", "hello_frag");

    let positions = [[-0.5, -0.5], [0.0, 0.5], [0.5, -0.25]];
    let mut tris = vec![];
    for position in positions {
        tris.push(Vertex { position });
    }

    // upload vertices to vertex buffer
    let vertex_buffer = glium::VertexBuffer::new(&shader_mgr.display, &tris).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut target = shader_mgr.target();
    target.clear_color(0.0, 0.0, 1.0, 1.0);

    let program = &shader_mgr.programs[0];
    target
        .draw(
            &vertex_buffer,
            indices,
            program,
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
