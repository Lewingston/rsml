
use winit::window::WindowId;
use winit::event_loop::ActiveEventLoop;

use crate::app::renderer::Renderer;
use crate::window::Window;
use crate::window::WindowHandler;

use crate::error::Error;


pub struct WindowManager {

    window_map: std::collections::HashMap<WindowId, WindowHandler>,
    renderer:   Option<Renderer>,
}



impl WindowManager {

    pub fn new() -> Self {

        Self {
            window_map: std::collections::HashMap::new(),
            renderer:   None
        }
    }


    pub fn create_window<WindowType: Window + 'static>(
        &mut self,
        event_loop: &ActiveEventLoop,
        window: WindowType
    ) -> Result<(), Error>
    {
        match &self.renderer {
            Some(renderer) => {
                let window_handler = WindowHandler::new(window, event_loop, renderer)?;
                self.window_map.insert(window_handler.get_window_id(), window_handler);
                Ok(())
            }
            None => {
                let (window_handler, renderer) = WindowHandler::create_window_and_renderer(window, event_loop)?;
                self.window_map.insert(window_handler.get_window_id(), window_handler);
                self.renderer = Some(renderer);
                Ok(())
            }
        }
    }


    pub fn resize_window(&mut self, window_id: WindowId, width: u32, height: u32) {

        let Some(window) = self.window_map.get_mut(&window_id) else { return; };

        let Some(renderer) = self.renderer.as_ref() else { return; };

        window.resize(width, height, renderer.get_device());
    }


    pub fn draw_window(&self, window_id: WindowId) {

        let Some(window) = self.window_map.get(&window_id) else { return; };

        let Some(renderer) = self.renderer.as_ref() else { return; };

        window.draw(renderer);
    }


    pub fn get_window(&mut self, window_id: WindowId) -> Option<&mut WindowHandler> {

        self.window_map.get_mut(&window_id)
    }


    pub fn close_window(&mut self, window_id: WindowId) {

        self.window_map.remove(&window_id);
    }


    pub fn get_window_count(&mut self) -> usize {

        self.window_map.len()
    }
}
