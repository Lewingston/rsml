
use crate::drawable::vertex_buffer::VertexBuffer;

pub struct DefaultRenderPipeline {

    render_pipeline: wgpu::RenderPipeline
}


impl DefaultRenderPipeline {


    #[must_use]
    pub fn new(
        device:                &wgpu::Device,
        target_surface_config: &wgpu::SurfaceConfiguration
    ) -> Self {

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Default Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("default_shader.wgsl").into())
        });

        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("Default Render Pipeline Layout"),
                bind_group_layouts: &[],
                immediate_size: 0
            });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label:  Some("Default Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module:              &shader,
                entry_point:         Some("vs_main"),
                buffers:             &[
                    VertexBuffer::get_layout()
                ],
                compilation_options: wgpu::PipelineCompilationOptions::default()
            },
            fragment: Some(wgpu::FragmentState {
                module:      &shader,
                entry_point: Some("fs_main"),
                targets:     &[Some(wgpu::ColorTargetState {
                    format:     target_surface_config.format,
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
                conservative:       false
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask:  !0,
                alpha_to_coverage_enabled: false,
            },
            multiview_mask: None,
            cache:          None
        });

        Self {
            render_pipeline
        }
    }


    #[must_use]
    pub fn get_pipeline(&self) -> &wgpu::RenderPipeline {

        &self.render_pipeline
    }
}
