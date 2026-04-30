
use rsml::drawable::drawable::Color;


struct MyApp {

}


impl rsml::App for MyApp {

    fn start(&mut self, context: &mut rsml::AppContext) {

        _ = context.create_window(MainWindow::new());
    }
}


struct Scene {

}


impl Scene {

    fn new(_width: u32, _height: u32) -> Option<Self> {

        Some(Self {})
    }


    fn draw(&self, _render_target: &mut rsml::RenderTarget) {

    }
}


struct MainWindow {

    scene: Option<Scene>
}


impl MainWindow {

    fn new() -> Self {

        Self {
            scene: None
        }
    }
}


impl rsml::Window for MainWindow {

    fn start(&mut self, context: rsml::WindowContext) {

        context.camera.borrow_mut().set_projection_mode(rsml::renderer::camera::ProjectionMode::ORTHOGRAPHIC);

        context.window_config.background_color = Color { r: 26, g: 33, b: 46, a: 255 };
        context.window_config.adjust_camera_on_resize = true;

        self.scene = Scene::new(context.get_width(), context.get_height());
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget, _frame_monitor: &rsml::FrameMonitor) {

        let Some(scene) = &self.scene else { return; };

        scene.draw(render_target);
    }


    fn event(&mut self, _event: winit::event::WindowEvent, _context: rsml::WindowContext) {

    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
