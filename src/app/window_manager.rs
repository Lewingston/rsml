
use winit::window::Window as WinitWindow;
use winit::window::WindowId;
use winit::event_loop::ActiveEventLoop;

use crate::app::renderer::Renderer;
use crate::window::Window;
use crate::window::WindowHandler;

use crate::error::Error;


pub struct WindowManager {

    window_map: std::collections::HashMap<WindowId, WindowHandler>,
    renderer: Option<Renderer>,
}


impl WindowManager {

    pub fn new() -> Self {

        Self {
            window_map: std::collections::HashMap::new(),
            renderer: None
        }
    }

    pub fn create_window<T: Window + 'static>(
        &mut self,
        event_loop: &ActiveEventLoop,
        window: T) -> Result<(), Error>
    {

        let mut window_attributes = WinitWindow::default_attributes();
        window_attributes.visible = false;

        let winit_window = match event_loop.create_window(window_attributes) {
            Ok(window) => window,
            Err(err) => return Err(Error::FailedToCreateWindow(err.to_string()))
        };

        if self.window_map.is_empty() {

            self.init_renderer(&winit_window)?;
        }

        winit_window.set_visible(true);

        let window_id = winit_window.id();
        let window_handler = WindowHandler::new(window, winit_window);

        self.window_map.insert(window_id, window_handler);

        Ok(())
    }


    fn init_renderer(&mut self, winit_window: &WinitWindow) -> Result<(), Error>
    {
        self.renderer = Some(pollster::block_on(
            Renderer::new(winit_window)
        )?);

        Ok(())
    }


    pub fn get_window(&mut self, window_id: WindowId) -> Option<&mut WindowHandler>
    {
        self.window_map.get_mut(&window_id)
    }


    pub fn close_window(&mut self, window_id: WindowId) {

        self.window_map.remove(&window_id);
    }


    pub fn get_window_count(&mut self) -> usize {

        self.window_map.len()
    }
}
