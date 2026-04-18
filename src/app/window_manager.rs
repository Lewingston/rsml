
use winit::window::WindowId;
use winit::event_loop::ActiveEventLoop;

use crate::renderer::Renderer;
use crate::window::Window;
use crate::window::WindowHandler;

use crate::error::Error;


pub struct WindowManager {

    window_map: std::collections::HashMap<WindowId, WindowHandler>,
}


impl WindowManager {

    pub fn new() -> Self {

        Self {
            window_map: std::collections::HashMap::new(),
        }
    }


    pub fn create_window<WindowType: Window + 'static>(
        &mut self,
        event_loop: &ActiveEventLoop,
        window: WindowType
    ) -> Result<(), Error> {

        let window_handler = WindowHandler::new(window, event_loop)?;
        self.window_map.insert(window_handler.get_window_id(), window_handler);

        Ok(())
    }


    pub fn resize_window(&mut self, window_id: WindowId, width: u32, height: u32) {

        let Some(window) = self.window_map.get_mut(&window_id) else { return; };

        window.resize(width, height, Renderer::get());
    }


    pub fn draw_window(&mut self, window_id: WindowId) {

        let Some(window) = self.window_map.get_mut(&window_id) else { return; };

        window.draw(Renderer::get());
    }


    pub fn window_event(&mut self, window_id: WindowId, event: winit::event::WindowEvent) {

        let Some(window) = self.window_map.get_mut(&window_id) else { return; };

        window.event(event);
    }


    pub fn close_window(&mut self, window_id: WindowId) {

        self.window_map.remove(&window_id);
    }


    pub fn get_window_count(&mut self) -> usize {

        self.window_map.len()
    }
}
