
struct MyApp {

}

impl rsml::App for MyApp {

    fn start(&mut self, context: &mut rsml::AppContext) {

        println!("APP STARTED!");

        _ = context.create_window(MainWindow::new());
        _ = context.create_window(SecondaryWindow::new());
    }
}


struct MainScene {

    pub render_pipeline: rsml::DefaultRenderPipeline,
    pub vertex_buffer:   rsml::VertexBuffer
}


struct MainWindow {

    scene: Option<MainScene>
}

impl MainWindow {

    fn new() -> Self {

        Self{
            scene: None
        }
    }
}

impl rsml::Window for MainWindow {

    fn start(&mut self, context: rsml::WindowContext) {

        println!("MainWindow start");

        self.scene = Some(MainScene {
            render_pipeline: rsml::DefaultRenderPipeline::new(context.device, context.surface_config),
            vertex_buffer: rsml::VertexBuffer::create_triangle(context.device)
        });
    }

    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        let Some(scene) = &self.scene else { return; };

        render_target.draw(&scene.vertex_buffer, scene.render_pipeline.get_pipeline());
    }

    fn event(&mut self, event: winit::event::WindowEvent) {

        println!("MainWindow event: {event:?}");
    }
}

struct SecondaryWindow {

}

impl SecondaryWindow {

    fn new() -> Self {

        Self{}
    }
}

impl rsml::Window for SecondaryWindow {

    fn start(&mut self, _context: rsml::WindowContext) {

        println!("SecondaryWindow start");
    }

    fn draw(&mut self, _render_target: &mut rsml::RenderTarget) {

    }

    fn event(&mut self, event: winit::event::WindowEvent) {

        println!("SecondaryWindow event: {event:?}");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
