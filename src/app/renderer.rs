
use winit::window::Window as WinitWindow;

use crate::error::Error;

use std::sync::Arc;


pub struct Renderer {

    wgpu_instance: wgpu::Instance,
    wgpu_adapter:  wgpu::Adapter,
    device:        wgpu::Device,
    queue:         wgpu::Queue,
}


impl Renderer {


    pub fn get_device(&self) -> &wgpu::Device { &self.device }


    pub async fn init_and_create_surface(window: Arc<WinitWindow>) -> Result<(Self, wgpu::Surface<'static>), Error> {

        let wgpu_instance = Renderer::create_instance();

        let (wgpu_adapter, surface) = Renderer::create_adapter_and_surface(&wgpu_instance, window).await?;

        let (device, queue) = Renderer::create_device_and_queue(&wgpu_adapter).await?;

        Ok((Self {
            wgpu_instance,
            wgpu_adapter,
            device,
            queue
        }, surface))
    }


    pub fn create_surface(&self, window: Arc<WinitWindow>) -> Result<wgpu::Surface<'static>, Error> {

        match self.wgpu_instance.create_surface(window) {
            Ok(surface) => Ok(surface),
            Err(err)    => Err(Error::FailedToCreateWindowSurface(err.to_string()))
        }
    }


    pub fn get_surface_config(&self, surface: &wgpu::Surface) -> wgpu::SurfaceConfiguration {

        let surface_caps = surface.get_capabilities(&self.wgpu_adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        wgpu::SurfaceConfiguration {
            usage:                         wgpu::TextureUsages::RENDER_ATTACHMENT,
            format:                        surface_format,
            width:                         0,
            height:                        0,
            present_mode:                  surface_caps.present_modes[0],
            alpha_mode:                    surface_caps.alpha_modes[0],
            view_formats:                  vec![],
            desired_maximum_frame_latency: 2,
        }
    }


    fn create_instance() -> wgpu::Instance {

        #[cfg(not(target_arch = "wasm32"))]
        let backend = wgpu::Backends::PRIMARY;

        #[cfg(target_arch = "wasm32")]
        let backend = wgpu::Backends::GL;

        wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends:                 backend,
            flags:                    wgpu::InstanceFlags::default(),
            memory_budget_thresholds: wgpu::MemoryBudgetThresholds::default(),
            backend_options:          wgpu::BackendOptions::default(),
            display:                  None
        })
    }


    async fn create_adapter_and_surface(
        instance: &wgpu::Instance,
        window:   Arc<WinitWindow>) -> Result<(wgpu::Adapter, wgpu::Surface<'static>), Error>
    {

        let surface = match instance.create_surface(window.clone()) {
            Ok(surface) => surface,
            Err(err)    => return Err(Error::FailedToCreateRenderer(err.to_string()))
        };

        match instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference:       wgpu::PowerPreference::default(),
                compatible_surface:     Some(&surface),
                force_fallback_adapter: false
            }
        ).await {
            Ok(adapter) => Ok((adapter, surface)),
            Err(err)    => Err(Error::FailedToCreateRenderer(err.to_string()))
        }
    }


    async fn create_device_and_queue(adapter: &wgpu::Adapter) -> Result<(wgpu::Device, wgpu::Queue), Error> {

        #[cfg(target_arch = "wasm32")]
        let limits = wgpu::Limits::downlevel_webgl2_default();

        #[cfg(not(target_arch = "wasm32"))]
        let limits = wgpu::Limits::default();

        match adapter.request_device(&wgpu::DeviceDescriptor {
            label:                 None,
            required_features:     wgpu::Features::empty(),
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            required_limits:       limits,
            memory_hints:          wgpu::MemoryHints::default(),
            trace:                 wgpu::Trace::Off
        }).await {
            Ok((device, queue)) => Ok((device, queue)),
            Err(err)            => Err(Error::FailedToCreateRenderer(err.to_string()))
        }
    }
}
