
use rsml::drawable::drawable::Color;
use rsml::drawable::drawable::Drawable;

use std::rc::Rc;
use std::cell::RefCell;


struct MyApp {

}


impl rsml::App for MyApp {

    fn start(&mut self, context: &mut rsml::AppContext) {

        _ = context.create_window(MainWindow::new());
    }
}


struct Scene {

    cube:           rsml::Shape,
    camera_control: rsml::CameraController,
    frame_stats:    rsml::FrameStatDisplay
}


impl Scene {

    fn new(width: u32, height: u32, camera: &Rc<RefCell<rsml::Camera>>) -> Self {

        let mut frame_stats = rsml::FrameStatDisplay::new(width, height);
        frame_stats.set_front_color(Color{ r: 255, g: 255, b: 255, a: 255 });

        Self {
            cube:           rsml::Shape::create_cube(),
            camera_control: rsml::CameraController::new(camera.clone()),
            frame_stats
        }
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget, frame_monitor: &rsml::FrameMonitor) {

        self.cube.draw(render_target);
        self.frame_stats.draw(render_target, frame_monitor);
    }


    fn screen_resized(&mut self, width: u32, height: u32) {

        self.frame_stats.screen_resized(width, height);
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

        context.window_config.background_color = Color { r: 26, g: 33, b: 46, a: 255 };

        self.scene = Some(Scene::new(
            context.get_width(),
            context.get_height(),
            context.camera
        ));
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget, frame_monitor: &rsml::FrameMonitor) {

        let Some(scene) = &mut self.scene else { return; };

        scene.draw(render_target, frame_monitor);
    }


    fn event(&mut self, event: winit::event::WindowEvent, _context: rsml::WindowContext) {

        use rsml::winit::event::WindowEvent;
        use rsml::winit::keyboard::KeyCode;

        let Some(scene) = &mut self.scene else { return; };


        match event {
            WindowEvent::Resized(size) => {

                scene.screen_resized(size.width, size.height);
            }
            WindowEvent::KeyboardInput { event, .. } => {

                let winit::keyboard::PhysicalKey::Code(code) = event.physical_key else { return; };

                scene.camera_control.keyboard_input(code, event.state.is_pressed());

                if code == KeyCode::Tab && event.state.is_pressed() {
                    scene.frame_stats.toggle_display_mode();
                }
            }
            _ => {}
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
