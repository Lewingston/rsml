
use winit::event::WindowEvent;
use winit::window::Window as WinitWindow;
use winit::event_loop::ActiveEventLoop;

use crate::app::window_manager::WindowManager;
use crate::error::Error;

use std::sync::Arc;

pub trait Window {

    fn start(&mut self);

    fn draw(&mut self);

    fn event(&mut self, event: WindowEvent);
}


pub struct WindowHandler {

    window:         Box<dyn Window>,
    winit_window:   Arc<WinitWindow>,
    surface:        wgpu::Surface<'static>,
    surface_config: wgpu::SurfaceConfiguration,
}


impl WindowHandler {

    pub fn new<T: Window + 'static>(
        window:         T,
        event_loop:     &ActiveEventLoop,
        window_manager: &mut WindowManager) -> Result<Self, Error>
    {

        let mut window_attributes = WinitWindow::default_attributes();
        window_attributes.visible = false;

        let winit_window = Arc::new(match event_loop.create_window(window_attributes) {
            Ok(window) => window,
            Err(err)   => return Err(Error::FailedToCreateWindow(err.to_string()))
        });

        let surface = window_manager.create_window_surface(winit_window.clone())?;

        let mut surface_config = window_manager.get_surface_config(&surface)?;
        surface_config.width  = winit_window.inner_size().width;
        surface_config.height = winit_window.inner_size().height;

        winit_window.set_visible(true);

        Ok(Self {
            window:         Box::new(window),
            winit_window:   winit_window,
            surface:        surface,
            surface_config: surface_config,
        })
    }


    pub fn get_window(&mut self) -> &mut Box<dyn Window> {
        &mut self.window
    }


    /*
    pub fn get_winit_window(&mut self) -> &mut WinitWindow {
        &mut self.winit_window
    }
    */


    pub fn get_window_id(&self) -> winit::window::WindowId {
        self.winit_window.id()
    }
}
