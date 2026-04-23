
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

    text: rsml::Text
}


impl Scene {

    fn new() -> Option<Self> {

        let Ok(mut font) = rsml::Font::from_file("./comic.ttf") else { return None };

        _ = font.get_char('X', 40.0);
        _ = font.get_char('Y', 40.0);
        _ = font.get_char('Z', 40.0);
        _ = font.get_char('1', 40.0);
        _ = font.get_char('2', 40.0);
        _ = font.get_char('ß', 40.0);
        _ = font.get_char('a', 40.0);
        _ = font.get_char('c', 40.0);
        _ = font.get_char('A', 40.0);

        let font = Rc::new(RefCell::new(font));

        let text = rsml::Text::new("TEST".to_string(), font, 40.0);

        Some(Self {text})
    }


    fn draw(&self, render_target: &mut rsml::RenderTarget) {

        self.text.draw(render_target);
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

        self.scene = Scene::new();
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

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
