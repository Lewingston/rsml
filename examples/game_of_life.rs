
use rsml::drawable::drawable::Color;
use rsml::drawable::drawable::Drawable;

use std::rc::Rc;
use std::cell::RefCell;


const CELL_SIZE: usize = 16;


struct MyApp {

}


impl rsml::App for MyApp {

    fn start(&mut self, context: &mut rsml::AppContext) {

        _ = context.create_window(MainWindow::new());
    }
}


struct Scene {

    game:        GameOfLife,
    frame_count: rsml::Text
}


impl Scene {

    fn new(width: u32, height: u32) -> Option<Self> {

        let window_height = height;

        let Ok(font) = rsml::Font::from_bytes(include_bytes!("./roboto.ttf")) else { return None; };
        let font = Rc::new(RefCell::new(font));

        let layout = fontdue::layout::LayoutSettings::default();

        let mut frame_count = rsml::Text::new("LOL12345678", font, 16.0, Some(layout));
        frame_count.set_color(Color { r: 240, g: 240, b: 240, a: 255 });
        frame_count.get_transform().translate(cgmath::Vector3::<f32>{x: 0.0, y: window_height as f32, z: 1.0});

        Some(Self {
            game: GameOfLife::new(width, height),
            frame_count
        })
    }


    fn set_frame_rate(&mut self, frame_time: &std::time::Duration) {

        let frame_time: f32 = frame_time.as_micros() as f32;

        if frame_time > 0.0 {

            let frame_rate = 1_000_000.0 / frame_time;
            self.frame_count.set_text(&format!("{:.2}", frame_rate));

        } else {

            self.frame_count.set_text(&"0.0");
        }
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        self.game.draw(render_target);
        self.frame_count.draw(render_target);
    }


    pub fn resize(&mut self, width: u32, height: u32) {

        self.frame_count.get_transform().set_pos(cgmath::Point3::<f32>{x: 0.0, y: height as f32, z: 1.0});

        self.game = GameOfLife::new(width, height);
    }
}


struct GameOfLife {

    cells:       Vec<Vec<bool>>,
    sprites:     Vec<Vec<rsml::Shape>>,
    width:       usize,
    height:      usize,
}


impl GameOfLife {

    fn new(width: u32, height: u32) -> Self {

        let width:  usize = width  as usize / CELL_SIZE;
        let height: usize = height as usize / CELL_SIZE;

        let mut sprites: Vec<Vec<rsml::Shape>> = Vec::new();

        for x in 0..width {
            let mut row: Vec<rsml::Shape> = Vec::new();
            for y in 0..height {

                let mut sprite = rsml::Shape::create_rectangle((CELL_SIZE - 1) as f32, (CELL_SIZE - 1) as f32);
                sprite.set_color(Color { r: 4, g: 4, b: 4, a: 255 });

                let pos_x = CELL_SIZE as f32 * (x as f32 + 0.5);
                let pos_y = CELL_SIZE as f32 * (y as f32 + 0.5);
                sprite.get_transform().translate(cgmath::Vector3::<f32>{ x: pos_x, y: pos_y, z: 0.0 });

                row.push(sprite);
            }
            sprites.push(row);
        }

        println!("Cell count: {}", width * height);

        Self {
            cells: vec![vec![false; width]; height],
            sprites,
            width,
            height
        }
    }


    fn draw(&self, render_target: &mut rsml::RenderTarget) {

        for row in &self.sprites {
            for sprite in row {
                sprite.draw(render_target);
            }
        }
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


    fn position_camera(
        camera: &mut rsml::renderer::camera::Camera,
        width: u32,
        height: u32
    ) {

        let width:  f32 = width  as f32;
        let height: f32 = height as f32;

        let mut cam_params = camera.get_parameters().clone();
        cam_params.projection = rsml::renderer::camera::ProjectionMode::ORTHOGRAPHIC;
        cam_params.pos    = cgmath::Point3::<f32> { x: width / 2.0, y: height / 2.0, z: 10.0 };
        cam_params.target = cgmath::Point3::<f32> { x: width / 2.0, y: height / 2.0, z: 0.0 };
        camera.set_parameters(cam_params);
    }
}


impl rsml::Window for MainWindow {

    fn start(&mut self, context: rsml::WindowContext) {

        let mut camera = context.camera.borrow_mut();

        MainWindow::position_camera(&mut camera, context.get_width(), context.get_height());

        context.window_config.background_color = Color { r: 24, g: 26, b: 27, a: 255 };
        context.window_config.adjust_camera_on_resize = true;

        self.scene = Scene::new(context.get_width(), context.get_height());
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget, frame_monitor: &rsml::FrameMonitor) {

        let Some(scene) = &mut self.scene else { return; };

        scene.set_frame_rate(&frame_monitor.get_frame_time());

        scene.draw(render_target);
    }


    fn event(&mut self, event: winit::event::WindowEvent, context: rsml::WindowContext) {

        let Some(scene) = &mut self.scene else { return; };

        match event {
            winit::event::WindowEvent::Resized(size) => {

                MainWindow::position_camera(&mut context.camera.borrow_mut(), size.width, size.height);
                scene.resize(size.width, size.height);
            }
            _ => {}
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
