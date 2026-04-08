
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

    pub triangle: rsml::Shape
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
            triangle: rsml::Shape::create_triangle(context.renderer)
        });
    }

    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        let Some(scene) = &self.scene else { return; };

        render_target.draw(&scene.triangle);
    }

    fn event(&mut self, event: winit::event::WindowEvent) {

        println!("MainWindow event: {event:?}");
    }
}

struct SecondaryScene {

    pub square: rsml::Shape
}

struct SecondaryWindow {

    scene: Option<SecondaryScene>
}

impl SecondaryWindow {

    fn new() -> Self {

        Self{
            scene: None
        }
    }
}

impl rsml::Window for SecondaryWindow {

    fn start(&mut self, context: rsml::WindowContext) {

        println!("SecondaryWindow start");

        self.scene = Some(SecondaryScene {
            square: rsml::Shape::create_rectangle(context.renderer, 0.5, 0.5)
        });
    }

    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        let Some(scene) = &self.scene else { return; };

        render_target.draw(&scene.square);
    }

    fn event(&mut self, event: winit::event::WindowEvent) {

        println!("SecondaryWindow event: {event:?}");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
