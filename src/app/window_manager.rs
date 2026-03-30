
use winit::window::Window as WinitWindow;
use winit::window::WindowId;
use winit::event_loop::ActiveEventLoop;
use winit::error::OsError;

use crate::window::Window;
use crate::window::WindowHandler;


pub struct WindowManager {

    window_map: std::collections::HashMap<WindowId, WindowHandler>
}


impl WindowManager {

    pub fn new() -> Self {

        Self {
            window_map: std::collections::HashMap::new()
        }
    }

    pub fn create_window<T: Window + 'static>(&mut self, event_loop: &ActiveEventLoop, window: T) -> Result<(), OsError> {

        let window_attributes = WinitWindow::default_attributes();

        let winit_window = event_loop.create_window(window_attributes)?;

        let window_id = winit_window.id();
        let window_handler = WindowHandler::new(window, winit_window);

        self.window_map.insert(window_id, window_handler);

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
