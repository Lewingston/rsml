
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

    header_text: rsml::Text,
    text_area:   rsml::Shape,
    text_input:  rsml::Text,
    info_text:   rsml::Text,

    text: String,

    font: Rc<RefCell<rsml::Font>>
}


impl Scene {

    fn new(
        text:   &str,
        width:  u32,
        height: u32,
        font:   Rc<RefCell<rsml::Font>>
    ) -> Option<Self> {

        use fontdue::layout::LayoutSettings as Layout;

        let width  = width as f32;
        let height = height as f32;

        let header_layout = Layout {
            x: -width / 2.0,
            y: height / 2.0 - 15.0,
            max_width: Some(width),
            horizontal_align: fontdue::layout::HorizontalAlign::Center,
            ..Layout::default()
        };

        let header_text = rsml::Text::new("Text Rendering Demo", font.clone(), 30.0, Some(header_layout));
        header_text.set_color(Color { r: 100, g: 100, b: 100, a: 255 });

        let padding_hor = 5.0;
        let text_input_layout = Layout {
            x: -width / 4.0 + padding_hor,
            y: height / 2.0 - 61.0,
            max_width: Some((width / 2.0) - (padding_hor * 2.0)),
            horizontal_align: fontdue::layout::HorizontalAlign::Left,
            vertical_align:   fontdue::layout::VerticalAlign::Top,
            ..Layout::default()
        };

        let text_input = rsml::Text::new(text, font.clone(), 20.0, Some(text_input_layout));
        text_input.set_color(Color { r: 100, g: 100, b: 100, a: 255 });

        let mut text_area = rsml::Shape::create_rectangle(width / 2.0, height - 60.0);
        text_area.set_color(Color { r: 20, g: 27, b: 36, a: 255 });
        text_area.get_transform().translate(cgmath::Vector3::<f32>{x: 0.0, y: -30.0, z: -1.0});

        let info_text_layout = Layout {
            x: -width / 4.0 + padding_hor,
            y: height / 2.0 - 61.0,
            max_width: Some((width / 2.0) - (padding_hor * 2.0)),
            max_height: Some(height - 61.0),
            horizontal_align: fontdue::layout::HorizontalAlign::Center,
            vertical_align:   fontdue::layout::VerticalAlign::Middle,
            ..Layout::default()
        };

        let info_text = rsml::Text::new("Type for text input", font.clone(), 30.0, Some(info_text_layout));
        info_text.set_color(Color { r: 25, g: 25, b: 25, a: 255 });

        Some(Self {
            header_text,
            text_area,
            text_input,
            info_text,
            text: text.to_string(),
            font
        })
    }


    fn draw(&self, render_target: &mut rsml::RenderTarget) {

        self.header_text.draw(render_target);
        self.text_area.draw(render_target);

        if self.text.is_empty() {
            self.info_text.draw(render_target);
        } else {
            self.text_input.draw(render_target);
        }
    }


    pub fn input_text(&mut self, c: char) {

        self.text.push(c);

        self.text_input.set_text(&self.text);
    }


    pub fn remove_char(&mut self) {

        self.text.pop();

        self.text_input.set_text(&self.text);
    }


    pub fn get_text(&self) -> &str {

        &self.text
    }


    pub fn get_font(&self) -> Rc<RefCell<rsml::Font>> {

        self.font.clone()
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

        let Ok(font) = rsml::Font::from_bytes(include_bytes!("./roboto.ttf")) else { return; };
        let font = Rc::new(RefCell::new(font));

        self.scene = Scene::new("", context.get_width(), context.get_height(), font);
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        let Some(scene) = &self.scene else { return; };

        scene.draw(render_target);
    }


    fn event(&mut self, event: winit::event::WindowEvent, _context: rsml::WindowContext) {

        let Some(scene) = &mut self.scene else { return; };

        match event {
            winit::event::WindowEvent::KeyboardInput {
                event,
                is_synthetic: false,
                ..
            } => {

                if event.state != winit::event::ElementState::Pressed {
                    return;
                }

                if let Some(text) = &event.text {

                    for c in text.chars() {
                        match c {
                            '\n' | '\r' => {
                                scene.input_text('\n');
                            }
                            c if !c.is_control() => {
                                scene.input_text(c);
                            }
                            _ => {

                            }
                        }
                    }
                }

                match &event.logical_key {
                    winit::keyboard::Key::Named(winit::keyboard::NamedKey::Backspace) => {
                        scene.remove_char();
                    }
                    _ => {}
                }
            }
            winit::event::WindowEvent::Resized(size) => {

                self.scene = Scene::new(scene.get_text(), size.width, size.height, scene.get_font());
            }
            _ => {}
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
