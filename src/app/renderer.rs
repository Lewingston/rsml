
use winit::window::Window as WinitWindow;

use crate::error::Error;


pub struct Renderer {

    wgpu_instance: wgpu::Instance,
    wgpu_adapter:  wgpu::Adapter,
    device:        wgpu::Device,
    queue:         wgpu::Queue,
}


impl Renderer {

    pub async fn new(window: &WinitWindow) -> Result<Self, Error> {

        let wgpu_instance = Renderer::create_instance();

        let wgpu_adapter = Renderer::create_adapter(&wgpu_instance, window).await?;

        let (device, queue) = Renderer::create_device_and_queue(&wgpu_adapter).await?;

        Ok(Self {
            wgpu_instance,
            wgpu_adapter,
            device,
            queue
        })
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


    async fn create_adapter(instance: &wgpu::Instance, window: &WinitWindow) -> Result<wgpu::Adapter, Error> {

        let surface = match instance.create_surface(window) {
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
            Ok(adapter) => Ok(adapter),
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
