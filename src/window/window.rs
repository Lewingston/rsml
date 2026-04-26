
use winit::event::WindowEvent;
use winit::window::Window as WinitWindow;
use winit::event_loop::ActiveEventLoop;

use crate::error::Error;

use crate::renderer::Renderer;
use crate::renderer::render_target::RenderTarget;
use crate::renderer::camera::Camera;

use crate::drawable::texture::Texture;
use crate::drawable::drawable::Color;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowAttributesExtWebSys;

use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;


pub trait Window {

    fn start(&mut self, window_context: WindowContext);

    fn draw(&mut self, render_target: &mut RenderTarget);

    fn event(&mut self, event: WindowEvent, window_context: WindowContext);
}


pub struct WindowHandler {

    window:          Box<dyn Window>,
    winit_window:    Arc<WinitWindow>,
    surface:         wgpu::Surface<'static>,
    surface_config:  wgpu::SurfaceConfiguration,
    camera:          Rc<RefCell<Camera>>,
    depth_texture:   Texture,
    config:          WindowConfig
}


pub struct WindowContext<'window_handler> {

    pub surface:        &'window_handler wgpu::Surface<'static>,
    pub surface_config: &'window_handler wgpu::SurfaceConfiguration,
    pub camera:         &'window_handler Rc<RefCell<Camera>>,
    pub window_config:  &'window_handler mut WindowConfig
}


impl WindowContext<'_> {

    #[must_use]
    pub fn get_width(&self) -> u32 { self.surface_config.width }

    #[must_use]
    pub fn get_height(&self) -> u32 { self.surface_config.height }
}


pub struct WindowConfig
{
    pub adjust_camera_on_resize: bool,
    pub background_color:        Color
}


impl WindowHandler {


    #[cfg(not(target_arch = "wasm32"))]
    pub fn new<T: Window + 'static>(
        window:     T,
        event_loop: &ActiveEventLoop
    ) -> Result<Self, Error> {

        let winit_window = WindowHandler::create_winit_window(event_loop)?;

        let surface = Renderer::create_window_surface(winit_window.clone())?;

        let surface_config = create_surface_config(&winit_window, &surface);

        let camera = Rc::new(
            RefCell::new(
                Camera::new(
                    surface_config.width,
                    surface_config.height
                )
            )
        );

        let depth_texture = Texture::create_depth_texture(&surface_config);

        let config = WindowConfig {
            adjust_camera_on_resize: true,
            background_color:        Color{r: 25, g: 50, b: 75, a: 255}
        };

        let mut window_handler = Self {
            window: Box::new(window),
            winit_window,
            surface,
            surface_config,
            camera,
            depth_texture,
            config
        };

        window_handler.start();

        Ok(window_handler)
    }


    #[cfg(target_arch = "wasm32")]
    pub fn new(
        window: Box<dyn Window>,
        winit_window: Arc<WinitWindow>,
        surface: wgpu::Surface<'static>
    ) -> Self {

        let surface_config = create_surface_config(&winit_window, &surface);

        let camera = Rc::new(
            RefCell::new(
                Camera::new(
                    surface_config.width,
                    surface_config.height
                )
            )
        );

        let depth_texture = Texture::create_depth_texture(&surface_config);

        let mut window_handler = Self {
            window,
            winit_window,
            surface,
            surface_config,
            camera,
            depth_texture
        };

        window_handler.start();

        window_handler
    }


    fn start(&mut self) {

        let context = WindowContext {
            surface:        &self.surface,
            surface_config: &self.surface_config,
            camera:         &self.camera,
            window_config:  &mut self.config
        };

        self.window.start(context);

        self.winit_window.set_visible(true);
    }


    pub fn event(&mut self, event: winit::event::WindowEvent) {

        let context = WindowContext {
            surface:        &self.surface,
            surface_config: &self.surface_config,
            camera:         &self.camera,
            window_config:  &mut self.config
        };

        self.window.event(event, context);
    }


    pub fn resize(
        &mut self,
        width:  u32,
        height: u32,
    ) {

        let limits = Renderer::get().get_limits();
        let max_tex_size = limits.max_texture_dimension_2d;

        self.surface_config.width  = std::cmp::min(width, max_tex_size);
        self.surface_config.height = std::cmp::min(height, max_tex_size);
        self.surface.configure(Renderer::get().get_device(), &self.surface_config);
        self.depth_texture = Texture::create_depth_texture(&self.surface_config);

        if self.config.adjust_camera_on_resize {
            let mut cam_params = self.camera.borrow().get_parameters().clone();
            cam_params.width  = width;
            cam_params.height = height;
            self.camera.borrow_mut().set_parameters(cam_params);
        }
    }


    pub fn draw(&mut self) {

        self.winit_window.request_redraw();

        let output = match self.surface.get_current_texture() {

            wgpu::CurrentSurfaceTexture::Success(surface_texture) => surface_texture,
            wgpu::CurrentSurfaceTexture::Suboptimal(surface_texture) => {
                self.surface.configure(Renderer::get().get_device(), &self.surface_config);
                surface_texture
            }
            wgpu::CurrentSurfaceTexture::Timeout |
            wgpu::CurrentSurfaceTexture::Occluded |
            wgpu::CurrentSurfaceTexture::Validation => {

                return;
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(Renderer::get().get_device(), &self.surface_config);
                return;
            }
            wgpu::CurrentSurfaceTexture::Lost => {
                // TODO: Recreate all resources or exit application
                return;
            }
        };

        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = Renderer::get().get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder")
        });

        {
            let color = self.config.background_color;

            let render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view:           &view,
                    resolve_target: None,
                    depth_slice:    None,
                    ops: wgpu::Operations {
                        //load:  wgpu::LoadOp::Clear(color.to_wgpu_color()),
                        load: wgpu::LoadOp::Clear(color.to_srgb()),
                        store: wgpu::StoreOp::Store
                    }
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view:      self.depth_texture.get_view(),
                    depth_ops: Some(wgpu::Operations {
                        load:  wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store
                    }),
                    stencil_ops: None
                }),
                occlusion_query_set:      None,
                timestamp_writes:         None,
                multiview_mask:           None
            });

            let mut render_target = RenderTarget::new(render_pass, self.camera.clone());

            self.window.draw(&mut render_target);
        }

        Renderer::get().get_queue().submit(std::iter::once(encoder.finish()));
        output.present();
    }


    pub fn get_window_id(&self) -> winit::window::WindowId {
        self.winit_window.id()
    }


    pub fn create_winit_window(event_loop: &ActiveEventLoop) -> Result<Arc<WinitWindow>, Error> {

        let mut window_attributes = WinitWindow::default_attributes();
        window_attributes.visible = false;

        #[cfg(target_arch = "wasm32")]
        {
            const CANVAS_ID: &str = "canvas";

            let window = wgpu::web_sys::window()
                .ok_or(Error::FailedToAcquireHtmlElement("window".to_string()))?;

            let document = window.document()
                .ok_or(Error::FailedToAcquireHtmlElement("document".to_string()))?;

            let canvas = document.get_element_by_id(CANVAS_ID)
                .ok_or(Error::FailedToAcquireHtmlElement("canvas".to_string()))?;

            let html_canvas_element = canvas.unchecked_into();
            window_attributes = window_attributes.with_canvas(Some(html_canvas_element));
        }

        match event_loop.create_window(window_attributes) {
            Ok(window) => Ok(Arc::new(window)),
            Err(err)   => Err(Error::FailedToCreateWindow(err.to_string()))
        }
    }
}


fn create_surface_config(
    winit_window: &WinitWindow,
    surface:      &wgpu::Surface
) -> wgpu::SurfaceConfiguration {

    let mut surface_config = Renderer::get().get_surface_config(surface);
    surface_config.width   = winit_window.inner_size().width;
    surface_config.height  = winit_window.inner_size().height;

    surface_config
}
