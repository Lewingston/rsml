
use rsml::window::Window;

fn main() {

    let _window = match Window::new() {
        Ok(window) => window,
        Err(err) =>  { println!("{err}",); return },
    };

    println!("Window created!");
}
