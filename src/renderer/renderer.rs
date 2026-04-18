
use winit::window::Window as WinitWindow;

use crate::error::Error;

use crate::drawable::drawable::Vertex;
use crate::drawable::texture::Texture;

use crate::renderer::uniform::MatrixUniform;

use std::sync::Arc;
use std::rc::Rc;


pub struct Renderer {

    wgpu_instance: wgpu::Instance,
    wgpu_adapter:  wgpu::Adapter,
    device:        wgpu::Device,
    queue:         wgpu::Queue,

    default_color_render_pipeline:   Rc<wgpu::RenderPipeline>,
    default_texture_render_pipeline: Rc<wgpu::RenderPipeline>,
}


impl Renderer {


    #[must_use]
    pub fn get_device(&self) -> &wgpu::Device { &self.device }


    #[must_use]
    pub fn get_queue(&self) -> &wgpu::Queue { &self.queue }


    #[must_use]
    pub fn get_default_color_render_pipeline(&self) -> Rc<wgpu::RenderPipeline> {

        self.default_color_render_pipeline.clone()
    }


    #[must_use]
    pub fn get_default_texture_render_pipeline(&self) -> Rc<wgpu::RenderPipeline> {

        self.default_texture_render_pipeline.clone()
    }


    /// # Errors
    ///
    /// Returns an error if surface creation fails.
    pub async fn init_and_create_surface(window: Arc<WinitWindow>)
        -> Result<(Self, wgpu::Surface<'static>), Error>
    {

        let wgpu_instance = Renderer::create_instance();

        let (wgpu_adapter, surface) = Renderer::create_adapter_and_surface(&wgpu_instance, window).await?;

        let (device, queue) = Renderer::create_device_and_queue(&wgpu_adapter).await?;

        let surface_config = get_surface_config(&surface, &wgpu_adapter);

        let default_color_render_pipeline = Rc::new(
            create_default_color_render_pipeline(&device, &surface_config));

        let default_texture_render_pipeline = Rc::new(
            create_default_texture_render_pipeline(&device, &surface_config));

        Ok((Self {
            wgpu_instance,
            wgpu_adapter,
            device,
            queue,
            default_color_render_pipeline,
            default_texture_render_pipeline
        }, surface))
    }


    /// # Errors
    ///
    /// Returns an error if surface creation fails.
    pub fn create_surface(&self, window: Arc<WinitWindow>) -> Result<wgpu::Surface<'static>, Error> {

        match self.wgpu_instance.create_surface(window) {
            Ok(surface) => Ok(surface),
            Err(err)    => Err(Error::FailedToCreateWindowSurface(err.to_string()))
        }
    }


    pub fn get_surface_config(&self, surface: &wgpu::Surface) -> wgpu::SurfaceConfiguration {

        get_surface_config(surface, &self.wgpu_adapter)
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


fn get_surface_config(surface: &wgpu::Surface, adapter: &wgpu::Adapter) -> wgpu::SurfaceConfiguration {

    let surface_caps = surface.get_capabilities(adapter);

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


fn create_pipeline(
    device:               &wgpu::Device,
    shader:               &wgpu::ShaderModule,
    surface_config:       &wgpu::SurfaceConfiguration,
    pipeline_layout:      &wgpu::PipelineLayout,
    vertex_buffer_layout: wgpu::VertexBufferLayout,
) -> wgpu::RenderPipeline {

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label:  Some("Default render pipeline"),
        layout: Some(pipeline_layout),
        vertex: wgpu::VertexState {
            module:              shader,
            entry_point:         Some("vs_main"),
            buffers:             &[vertex_buffer_layout],
            compilation_options: wgpu::PipelineCompilationOptions::default()
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs_main"),
            targets: &[Some(wgpu::ColorTargetState {
                format:     surface_config.format,
                blend:      Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL
            })],
            compilation_options: wgpu::PipelineCompilationOptions::default()
        }),
        primitive: wgpu::PrimitiveState {
            topology:           wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face:         wgpu::FrontFace::Ccw,
            cull_mode:          Some(wgpu::Face::Back),
            polygon_mode:       wgpu::PolygonMode::Fill,
            unclipped_depth:    false,
            conservative:       false,
        },
        depth_stencil:           Some(wgpu::DepthStencilState {
            format:              wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: Some(true),
            depth_compare:       Some(wgpu::CompareFunction::Less),
            stencil:             wgpu::StencilState::default(),
            bias:                wgpu::DepthBiasState::default()
        }),
        multisample: wgpu::MultisampleState {
            count: 1,
            mask:  !0,
            alpha_to_coverage_enabled: false,
        },
        multiview_mask: None,
        cache:          None
    })
}


fn create_default_color_render_pipeline(
    device: &wgpu::Device,
    surface_config: &wgpu::SurfaceConfiguration
) -> wgpu::RenderPipeline {

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label:  Some("Default color shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("default_color_shader.wgsl").into())
    });

    let vertex_buffer_layout = Vertex::get_layout();

    let matrix_layout = MatrixUniform::get_bind_group_layout(device);

    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label:              Some("Color pipeline layout descriptor"),
            bind_group_layouts: &[
                Some(&matrix_layout), // Transformation matrix
                Some(&matrix_layout)  // Camera matrix
            ],
            immediate_size:     0
        });

    create_pipeline(
        device,
        &shader,
        surface_config,
        &pipeline_layout,
        vertex_buffer_layout)
}


fn create_default_texture_render_pipeline(
    device: &wgpu::Device,
    surface_config: &wgpu::SurfaceConfiguration
) ->wgpu::RenderPipeline {

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Default texture shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("default_texture_shader.wgsl").into())
    });

    let vertex_buffer_layout = Vertex::get_layout();

    let matrix_layout = MatrixUniform::get_bind_group_layout(device);

    let texture_layout = Texture::get_default_bind_group_layout(device);

    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Texture pipeline layout descriptor"),
            bind_group_layouts: &[
                Some(&matrix_layout),  // Transformation matrix
                Some(&matrix_layout),  // Camera matrix
                Some(&texture_layout)  // Texture
            ],
            immediate_size: 0
        });

    create_pipeline(
        device,
        &shader,
        surface_config,
        &pipeline_layout,
        vertex_buffer_layout)
}
