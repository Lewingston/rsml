

struct MyApp {

}


impl rsml::App for MyApp {

    fn start(&mut self, context: &mut rsml::AppContext) {

        _ = context.create_window(MainWindow::new());
        _ = context.create_window(MainWindow::new());
        _ = context.create_window(MainWindow::new());
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

    fn start(&mut self, _context: rsml::WindowContext) {

    }


    fn draw(&mut self, _render_target: &mut rsml::RenderTarget) {

    }


    fn event(&mut self, _event: winit::event::WindowEvent, _context: rsml::WindowContext) {

    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
