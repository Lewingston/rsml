
use winit::application::ApplicationHandler;
use winit::event_loop::EventLoop;
use winit::event_loop::ActiveEventLoop;
use winit::event_loop::EventLoopProxy;
use winit::event::WindowEvent;
use winit::window::Window as WinitWindow;

#[cfg(target_arch = "wasm32")]
use winit::platform::web::EventLoopExtWebSys;

use crate::app::window_manager::WindowManager;
use crate::window::Window;
use crate::error::Error;

use std::sync::Arc;

pub type RsmlAppEvent =  (Box<dyn Window>, Arc<WinitWindow>, wgpu::Surface<'static>);


struct AppHandler<AppType: App + 'static> {

    app:            AppType,
    window_manager: WindowManager,
    proxy:          Arc<EventLoopProxy<RsmlAppEvent>>
}


pub trait App {

    fn start(&mut self, _: &mut AppContext);
}


/// # Errors
///
/// Propagates winit error if creating or start of event loop fails.
pub fn start<T: App + 'static>(app: T) -> Result<(), Error> {

    #[cfg(not(target_arch = "wasm32"))]
    {
        env_logger::init();
    }
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init_with_level(log::Level::Info)
            .map_err(|e| Error::FailedToInitConsoleLogger(format!("{e}")))?;
    }

    let event_loop: EventLoop<RsmlAppEvent> = EventLoop::with_user_event().build()
        .map_err(|e| Error::FailedToCreateEventLoop(format!("{e}")))?;

    let proxy = Arc::new(event_loop.create_proxy());

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut app_handler = AppHandler::<T> {
            app,
            window_manager: WindowManager::new(),
            proxy
        };
        event_loop.run_app(&mut app_handler)
            .map_err(|e| Error::FailedToStartApp(format!("{e}")))?;
    }
    #[cfg(target_arch = "wasm32")]
    {
        let app_handler = AppHandler::<T> {
            app,
            window_manager: WindowManager::new(),
            proxy
        };
        event_loop.spawn_app(app_handler);
    }

    Ok(())
}


/// # Errors
///
/// Propagates winit error if creating or start of event loop fails.
pub fn start_single_window_app<T: Window + 'static>(window: T) -> Result<(), Error> {

    start(SingleWindowApp{window: Some(window)})?;

    Ok(())
}


struct SingleWindowApp<T: Window> {

    window: Option<T>
}


impl<T: Window + 'static> App for SingleWindowApp<T> {

    fn start(&mut self, context: &mut AppContext) {

        let window = self.window.take();

        let Some(window) = window else {
            panic!("Where is the window?");
        };

        _ = context.create_window(window);
    }
}


impl<T: App + 'static> ApplicationHandler<RsmlAppEvent> for AppHandler<T> {


    fn resumed(&mut self, event_loop: &ActiveEventLoop) {

        let mut context = AppContext {
            event_loop,
            proxy:          self.proxy.clone(),
            window_manager: &mut self.window_manager,
        };

        self.app.start(&mut context);
    }


    fn user_event(&mut self, _event_loop: &ActiveEventLoop, mut _event: RsmlAppEvent) {

        #[cfg(target_arch = "wasm32")]
        {
            let window       = _event.0;
            let winit_window = _event.1;
            let surface      = _event.2;

            self.window_manager.add_window(window, winit_window, surface);
        }
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
            WindowEvent::RedrawRequested => {

                self.window_manager.draw_window(window_id);
            }
            WindowEvent::Resized(size) => {

                self.window_manager.resize_window(window_id, size.width, size.height);
                self.window_manager.window_event(window_id, event);
            }
            _ => {

                self.window_manager.window_event(window_id, event);
            }
        }
    }
}


pub struct AppContext<'a, 'b> {

    event_loop:     &'a ActiveEventLoop,
    proxy:          Arc<EventLoopProxy<RsmlAppEvent>>,
    window_manager: &'b mut WindowManager,
}


impl AppContext<'_, '_,> {

    /// # Errors
    ///
    /// Propagates winit error if creation of window fails.
    pub fn create_window<T: Window + 'static>(&mut self, window: T) -> Result<(), Error> {

        self.window_manager.create_window(self.event_loop, self.proxy.clone(), window)?;

        Ok(())
    }
}
