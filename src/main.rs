mod render;

#[macro_use]
extern crate glium;
fn main() {
    let event_loop = glium::winit::event_loop::EventLoop::builder()
        .build()
        .expect("event loop building");
    let (_window, display) = glium::backend::glutin::SimpleWindowBuilder::new().build(&event_loop);

    let mut render_mgr = render::RenderManager::new(display, "./resources/shaders/");
    render_mgr.load_shader("helloworld", "hello_vert", "hello_frag");

    let _ = event_loop.run(move |event, window_target| {
        match event {
            glium::winit::event::Event::WindowEvent { event, .. } => match event {
                glium::winit::event::WindowEvent::CloseRequested => window_target.exit(),
                glium::winit::event::WindowEvent::Resized(new_size) => {
                    render_mgr.resize(new_size);
                }
                _ => (),
            },
            _ => {
                render_mgr.draw_frame();
            }
        };
    });
}
