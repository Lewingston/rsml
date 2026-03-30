
struct MyApp {

}

impl rsml::App for MyApp {

    fn start(&mut self, context: &mut rsml::AppContext) {

        println!("APP STARTED!");

        _ = context.create_window(MainWindow::new());
        _ = context.create_window(SecondaryWindow::new());
    }
}

struct MainWindow {

}

impl MainWindow {

    fn new() -> Self {

        Self{}
    }
}

impl rsml::Window for MainWindow {

    fn start(&mut self) {

        println!("MainWindow start");
    }

    fn draw(&mut self) {

    }

    fn event(&mut self, event: winit::event::WindowEvent) {

        println!("MainWindow event: {event:?}");
    }
}

struct SecondaryWindow {

}

impl SecondaryWindow {

    fn new() -> Self {

        Self{}
    }
}

impl rsml::Window for SecondaryWindow {

    fn start(&mut self) {

        println!("SecondaryWindow start");
    }

    fn draw(&mut self) {

    }

    fn event(&mut self, event: winit::event::WindowEvent) {

        println!("SecondaryWindow event: {event:?}");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
