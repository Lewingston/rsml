
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

    text:   rsml::Text,
    sprite: rsml::Shape
}


impl Scene {

    fn new() -> Option<Self> {

        let font_size = 80.0;

        let Ok(font) = rsml::Font::from_file("./comic.ttf") else { return None };

        let font = Rc::new(RefCell::new(font));

        let text = rsml::Text::new("TEST Text\nWith multi line\nbut smoething is off\n1234567\n#?ß!", font.clone(), font_size);

        let Ok(texture) = font.borrow_mut().get_texture(font_size) else { return None; };

        let mut sprite = rsml::Shape::create_sprite(
            texture.get_width() as f32,
            texture.get_height() as f32,
            texture
        );

        sprite.get_transform().translate(cgmath::Vector3::<f32>{ x: -200.0, y: 150.0, z: 0.0 });

        Some(Self {
            text,
            sprite
        })
    }


    fn draw(&self, render_target: &mut rsml::RenderTarget) {

        self.text.draw(render_target);
        self.sprite.draw(render_target);
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
