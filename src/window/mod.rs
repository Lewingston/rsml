
//use std::sync::Arc;

use thiserror::Error;

use winit::{

    application::ApplicationHandler,
    event::WindowEvent as WinitEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::Window as WinitWindow
};

#[derive(Debug, Error)]
pub enum WindowError {

    #[error("Unable to create window: {0}")]
    CreateWindowError(String),

    #[error("Unable to open window: {0}")]
    ShowWindowError(String)
}

pub struct Window {

    window: Option<WinitWindow>
}

impl Window {

    /// # Errors
    ///
    /// Will return error if start of the application event loop failed.
    pub fn new() -> Result<Self, WindowError> {

        let event_loop: EventLoop<Window> = match EventLoop::with_user_event().build() {
            Ok(event_loop) => event_loop,
            Err(err) => return Err(WindowError::CreateWindowError(format!("Failed to create event loop! {err}"))),
        };

        let mut window = Self {
            window: None
        };

        match event_loop.run_app(&mut window) {
            Ok(()) => {},
            Err(err) => return Err(WindowError::CreateWindowError(format!("Failed to start event loop! {err}"))),
        }

        Ok(window)
    }
}


impl ApplicationHandler<Window> for Window {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let window_attributes = WinitWindow::default_attributes();

        let winit_window = match event_loop.create_window(window_attributes) {
            Ok(window) => window,
            Err(err) => { eprintln!("Unable to create window: {err}"); return; },
        };

        self.window = Some(winit_window);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut _window: Window) {

    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WinitEvent
    ) {

        println!("Event: {event:?}");

        if event == WinitEvent::CloseRequested { event_loop.exit() }

        /*
        match event {

            WinitEvent::CloseRequested => event_loop.exit(),
            //WinitEvent::Resized(_size) => {},
            //WinitEvent::RedrawRequested => {},
            //WinitEvent::KeyboardInput => {},
            _ => {}
        }
        */
    }
}
