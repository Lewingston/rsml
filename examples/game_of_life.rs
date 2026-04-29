
use rsml::drawable::drawable::Color;
use rsml::drawable::drawable::Drawable;


const CELL_SIZE: usize = 24;


struct MyApp {

}


impl rsml::App for MyApp {

    fn start(&mut self, context: &mut rsml::AppContext) {

        _ = context.create_window(MainWindow::new());
    }
}


struct Scene {

    game: GameOfLife
}


impl Scene {

    fn new(width: u32, height: u32) -> Option<Self> {

        let width:  usize = width  as usize / CELL_SIZE;
        let height: usize = height as usize / CELL_SIZE;

        Some(Self {
            game: GameOfLife::new(width, height)
        })
    }


    fn draw(&self, render_target: &mut rsml::RenderTarget) {

        self.game.draw(render_target);
    }
}


struct GameOfLife {

    cells:   Vec<Vec<bool>>,
    sprites: Vec<Vec<rsml::Shape>>,
    width:   usize,
    height:  usize,
}


impl GameOfLife {

    fn new(width: usize, height: usize) -> Self {

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
}


impl rsml::Window for MainWindow {

    fn start(&mut self, context: rsml::WindowContext) {

        let width  = context.get_width() as f32;
        let height = context.get_height() as f32;

        let mut camera = context.camera.borrow_mut();

        camera.set_projection_mode(rsml::renderer::camera::ProjectionMode::ORTHOGRAPHIC);

        let mut cam_params = camera.get_parameters().clone();
        cam_params.pos    = cgmath::Point3::<f32> { x: width / 2.0, y: height / 2.0, z: 10.0 };
        cam_params.target = cgmath::Point3::<f32> { x: width / 2.0, y: height / 2.0, z: 0.0 };
        camera.set_parameters(cam_params);

        context.window_config.background_color = Color { r: 24, g: 26, b: 27, a: 255 };
        context.window_config.adjust_camera_on_resize = true;

        self.scene = Scene::new(context.get_width(), context.get_height());
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
