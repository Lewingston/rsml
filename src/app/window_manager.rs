
use winit::window::Window as WinitWindow;
use winit::window::WindowId;
use winit::event_loop::ActiveEventLoop;

use crate::app::renderer::Renderer;
use crate::window::Window;
use crate::window::WindowHandler;

use crate::error::Error;

use std::sync::Arc;


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

    pub fn create_window<T: Window + 'static>(
        &mut self,
        event_loop: &ActiveEventLoop,
        window: T) -> Result<(), Error>
    {

        let window_handler = WindowHandler::new(window, event_loop, self)?;

        self.window_map.insert(window_handler.get_window_id(), window_handler);

        Ok(())
    }


    pub fn create_window_surface (
        &mut self,
        winit_window: Arc<WinitWindow>) -> Result<wgpu::Surface<'static>, Error>
    {
        match &self.renderer {
            Some(renderer) => renderer.create_surface(winit_window),
            None           => self.init_renderer_and_create_surface(winit_window)
        }
    }


    pub fn get_surface_config(&self, surface: &wgpu::Surface) -> Result<wgpu::SurfaceConfiguration, Error> {

        match &self.renderer {
            Some(renderer) => Ok(renderer.get_surface_config(surface)),
            None           => Err(Error::FailedToCreateWindowSurface("Renderer not initalized!".to_string()))
        }
    }


    fn init_renderer_and_create_surface(
        &mut self,
        winit_window: Arc<WinitWindow>) -> Result<wgpu::Surface<'static>, Error>
    {

        let (renderer, surface) = pollster::block_on(Renderer::init_and_create_surface(winit_window))?;

        self.renderer = Some(renderer);

        Ok(surface)
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
