
use rsml::drawable::drawable::Drawable;
use rsml::drawable::drawable::Color;

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

        let font_size = 60.0;

        let Ok(font) = rsml::Font::from_file("./comic.ttf") else { return None };

        let font = Rc::new(RefCell::new(font));

        let text = "This is a test text.\nThis is the second line.\nSome thing lol!\n1234567890\n?$#%&X§";

        let mut text = rsml::Text::new(text, font.clone(), font_size);
        text.set_color(Color { r: 0, g: 0, b: 255, a: 255 });
        text.get_transform().rotate_z(cgmath::Rad(45.0));

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


#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn run_web() -> Result<(), wasm_binding::JsValue> {

    web_sys::console::log_1(&"Hello from Rust!".into());

    console_error_panic_hook::set_once();

    rsml::start(MyApp{}).unwrap_throw();

    Ok(())
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
