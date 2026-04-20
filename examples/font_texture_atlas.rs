
use rsml::drawable::drawable::Drawable;


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

        context.camera.borrow_mut().set_projection_mode(rsml::renderer::camera::ProjectionMode::ORTHOGRAPHIC);

        self.scene = MainScene::new();
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        let Some(scene) = &self.scene else { return; };

        scene.sprite.draw(render_target);
    }


    fn event(&mut self, event: winit::event::WindowEvent, _context: rsml::WindowContext) {

        if let winit::event::WindowEvent::KeyboardInput {
            event: winit::event::KeyEvent {
                physical_key: winit::keyboard::PhysicalKey::Code(code),
                state: key_state,
                text,
                ..
            },
            ..
        } = event {

            let Some(scene) = &mut self.scene else { return; };

            if code.eq(&winit::keyboard::KeyCode::Escape) & key_state.is_pressed() {

            } else if key_state.is_pressed() {

                let Some(text) = text else { return; };

                for c in text.chars() {

                    scene.add_char(c);
                }
            }
        }
    }
}


struct MainScene {

    pub sprite: rsml::Shape,

    font: rsml::Font,

    font_size: f32
}


impl MainScene {

    pub fn new() -> Option<Self> {

        let font_size = 50.0;

        let Ok(mut font) = rsml::Font::from_file("./comic.ttf") else {
            println!("Failed to load font from file!");
            return None;
        };

        let Ok(texture) = font.get_texture(font_size) else {
            return None;
        };

        let sprite = rsml::Shape::create_sprite(
            texture.get_width() as f32,
            texture.get_height() as f32,
            texture
        );

        Some(Self {
            sprite,
            font,
            font_size
        })
    }


    fn add_char(&mut self, c: char) {

        let Ok(_) = self.font.get_char(c, self.font_size) else { return; };

        let Ok(texture) = self.font.get_texture(self.font_size) else { return; };

        self.sprite = rsml::Shape::create_sprite(
            texture.get_width() as f32,
            texture.get_height() as f32,
            texture
        );
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start_single_window_app(MainWindow::new())?;

    Ok(())
}
