
use winit::application::ApplicationHandler;
use winit::event_loop::EventLoop;
use winit::event_loop::ActiveEventLoop;
use winit::event::WindowEvent;
use winit::error::EventLoopError;

use crate::app::window_manager::WindowManager;
use crate::window::WindowHandler;
use crate::window::Window;
use crate::error::Error;


struct AppHandler<A: App> {

    app: A,
    window_manager: WindowManager,
}


pub trait App {

    fn start(&mut self, _: &mut AppContext);
}


/// # Errors
///
/// Propagates winit error if creating or start of event loop fails.
pub fn start<T: App + 'static>(app: T) -> Result<(), EventLoopError>
{
    let event_loop: EventLoop<AppHandler<T>> = EventLoop::with_user_event().build()?;

    let mut app_handler = AppHandler::<T> {
        app,
        window_manager: WindowManager::new(),
    };

    event_loop.run_app(&mut app_handler)?;

    Ok(())
}


impl<T: App + 'static> ApplicationHandler<AppHandler<T>> for AppHandler<T> {

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let mut context = AppContext {
            event_loop,
            window_manager: &mut self.window_manager
        };

        self.app.start(&mut context);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut _app: AppHandler<T>) {

    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent)
    {

        match event {
            WindowEvent::CloseRequested => {

                self.window_manager.close_window(window_id);
                if self.window_manager.get_window_count() == 0 {
                    event_loop.exit();
                }
            }
            WindowEvent::Resized(size) => {

                let Some(window) = self.get_window(window_id) else { return; };

                window.get_window().event(event);
            }
            _ => {

                let Some(window) = self.get_window(window_id) else { return; };

                window.get_window().event(event);
            }
        }
    }
}


impl<T: App> AppHandler<T> {

    fn get_window(&mut self, window_id: winit::window::WindowId) -> Option<&mut WindowHandler> {

        self.window_manager.get_window(window_id)
    }
}


pub struct AppContext<'a, 'b> {

    event_loop: &'a ActiveEventLoop,
    window_manager: &'b mut WindowManager,
}


impl AppContext<'_, '_> {

    /// # Errors
    ///
    /// Propagates winit error if creation of window fails.
    pub fn create_window<T: Window + 'static>(&mut self, window: T) -> Result<(), Error> {

        self.window_manager.create_window(self.event_loop, window)?;

        Ok(())
    }
}
