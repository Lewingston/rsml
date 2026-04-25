
use winit::window::WindowId;
#[cfg(target_arch = "wasm32")]
use winit::window::Window as WinitWindow;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoopProxy;

use crate::window::Window;
use crate::window::WindowHandler;

use crate::error::Error;

use crate::app::RsmlAppEvent;

#[cfg(target_arch = "wasm32")]
use crate::renderer::renderer::Renderer;

use std::sync::Arc;


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
        _proxy:     Arc<EventLoopProxy<RsmlAppEvent>>,
        window:     WindowType
    ) -> Result<(), Error> {

        #[cfg(not(target_arch = "wasm32"))]
        {
            let window_handler = WindowHandler::new(window, event_loop)?;
            self.window_map.insert(window_handler.get_window_id(), window_handler);
        }

        #[cfg(target_arch = "wasm32")]
        {
            Renderer::trigger_surface_creation(
                Box::new(window),
                _proxy,
                WindowHandler::create_winit_window(event_loop)?
            );
        }

        Ok(())
    }


    #[cfg(target_arch = "wasm32")]
    pub fn add_window(
        &mut self,
        window:       Box<dyn Window>,
        winit_window: Arc<WinitWindow>,
        surface:      wgpu::Surface<'static>
    ) {

        let window_handler = WindowHandler::new(window, winit_window, surface);
        self.window_map.insert(window_handler.get_window_id(), window_handler);
    }


    pub fn resize_window(&mut self, window_id: WindowId, width: u32, height: u32) {

        let Some(window) = self.window_map.get_mut(&window_id) else { return; };

        window.resize(width, height);
    }


    pub fn draw_window(&mut self, window_id: WindowId) {

        let Some(window) = self.window_map.get_mut(&window_id) else { return; };

        window.draw();
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
