
use winit::event::WindowEvent;
use winit::window::Window as WinitWindow;


pub trait Window {

    fn start(&mut self);

    fn draw(&mut self);

    fn event(&mut self, event: WindowEvent);
}


pub struct WindowHandler {

    window: Box<dyn Window>,
    winit_window: WinitWindow,
}


impl WindowHandler {

    pub fn new<T: Window + 'static>(window: T, winit_window: WinitWindow) -> Self {

        Self {
            window: Box::<T>::new(window),
            winit_window
        }
    }

    pub fn get_window(&mut self) -> &mut Box<dyn Window> {
        &mut self.window
    }

    pub fn get_winit_window(&mut self) -> &mut WinitWindow {
        &mut self.winit_window
    }
}
