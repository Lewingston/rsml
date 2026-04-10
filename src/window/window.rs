
use winit::event::WindowEvent;
use winit::window::Window as WinitWindow;
use winit::event_loop::ActiveEventLoop;

use crate::error::Error;

use crate::renderer::Renderer;
use crate::renderer::render_target::RenderTarget;
use crate::renderer::camera::Camera;

use std::sync::Arc;
use std::rc::Rc;


pub trait Window {

    fn start(&mut self, window_context: WindowContext);

    fn draw(&mut self, render_target: &mut RenderTarget);

    fn event(&mut self, event: WindowEvent);
}


pub struct WindowHandler {

    window:          Box<dyn Window>,
    winit_window:    Arc<WinitWindow>,
    surface:         wgpu::Surface<'static>,
    surface_config:  wgpu::SurfaceConfiguration,
    camera:          Rc<Camera>
}


pub struct WindowContext<'a, 'b, 'c> {

    pub renderer:       &'a Renderer,
    pub surface:        &'b wgpu::Surface<'static>,
    pub surface_config: &'c wgpu::SurfaceConfiguration
}


impl WindowHandler {


    pub fn new<T: Window + 'static>(
        window:     T,
        event_loop: &ActiveEventLoop,
        renderer:   &Renderer
    ) -> Result<Self, Error> {

        let winit_window = create_winit_window(event_loop)?;

        let surface = renderer.create_surface(winit_window.clone())?;

        let surface_config = create_surface_config(&winit_window, &surface, renderer);

        let camera = Rc::new(Camera::new(renderer.get_device(), surface_config.width as f32 / surface_config.height as f32));

        let mut window_handler = Self {
            window: Box::new(window),
            winit_window,
            surface,
            surface_config,
            camera
        };

        window_handler.start(renderer);

        Ok(window_handler)
    }


    pub fn create_window_and_renderer<T: Window + 'static>(
        window:     T,
        event_loop: &ActiveEventLoop,
    ) -> Result<(Self, Renderer), Error> {

        let winit_window = create_winit_window(event_loop)?;

        let (renderer, surface) = pollster::block_on(Renderer::init_and_create_surface(winit_window.clone()))?;

        let surface_config = create_surface_config(&winit_window, &surface, &renderer);

        let camera = Rc::new(Camera::new(renderer.get_device(), surface_config.width as f32 / surface_config.height as f32));

        let mut window_handler = Self {
            window: Box::new(window),
            winit_window,
            surface,
            surface_config,
            camera
        };

        window_handler.start(&renderer);

        Ok((window_handler, renderer))
    }


    fn start(&mut self, renderer: &Renderer) {

        let context = WindowContext {
            renderer,
            surface: &self.surface,
            surface_config: &self.surface_config
        };

        self.window.start(context);

        self.winit_window.set_visible(true);
    }


    pub fn resize(
        &mut self,
        width:  u32,
        height: u32,
        device: &wgpu::Device
    ) {
        self.surface_config.width  = width;
        self.surface_config.height = height;
        self.surface.configure(device, &self.surface_config);
    }


    pub fn draw(&mut self, renderer: &Renderer) {

        let output = match self.surface.get_current_texture() {

            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                self.surface.configure(renderer.get_device(), &self.surface_config);
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout |
            wgpu::CurrentSurfaceTexture::Occluded |
            wgpu::CurrentSurfaceTexture::Validation => {

                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(renderer.get_device(), &self.surface_config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                // TODO: Recreate all resources or exit application
                return;
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = renderer.get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view:           &view,
                    resolve_target: None,
                    depth_slice:    None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0
                        }),
                        store: wgpu::StoreOp::Store
                    }
                })],
                depth_stencil_attachment: None,
                occlusion_query_set:      None,
                timestamp_writes:         None,
                multiview_mask:           None
            });

            let mut render_target = RenderTarget::new(render_pass, self.camera.clone());

            self.window.draw(&mut render_target);
        }

        renderer.get_queue().submit(std::iter::once(encoder.finish()));
        output.present();
    }


    pub fn get_window(&mut self) -> &mut Box<dyn Window> {
        &mut self.window
    }


    pub fn get_window_id(&self) -> winit::window::WindowId {
        self.winit_window.id()
    }
}


fn create_winit_window(event_loop: &ActiveEventLoop) -> Result<Arc<WinitWindow>, Error> {

    let mut window_attributes = WinitWindow::default_attributes();
    window_attributes.visible = false;

    match event_loop.create_window(window_attributes) {
        Ok(window) => Ok(Arc::new(window)),
        Err(err)   => Err(Error::FailedToCreateWindow(err.to_string()))
    }
}


fn create_surface_config(
    winit_window: &WinitWindow,
    surface:      &wgpu::Surface,
    renderer:     &Renderer
) -> wgpu::SurfaceConfiguration {

    let mut surface_config = renderer.get_surface_config(surface);
    surface_config.width   = winit_window.inner_size().width;
    surface_config.height  = winit_window.inner_size().height;

    surface_config
}
