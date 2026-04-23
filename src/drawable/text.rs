
use wgpu::util::DeviceExt;

use crate::renderer::renderer::Renderer;
use crate::renderer::render_target::RenderTarget;
use crate::renderer::uniform::MatrixUniform;
use crate::drawable::drawable::Drawable;
use crate::drawable::drawable::Transform;
use crate::drawable::texture::Texture;
use crate::drawable::font::Font;

use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;


static TEXT_RENDER_PIPELINE: std::sync::OnceLock<Arc<wgpu::RenderPipeline>> = std::sync::OnceLock::new();


pub struct Text {

    transform: Transform,
    text:      String,
    font:      Rc<RefCell<Font>>,
    font_size: f32,

    render_pipeline: Arc<wgpu::RenderPipeline>,

    character_sprites: Vec<CharSpriteInstance>,

    instance_buffer: wgpu::Buffer
}


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CharSpriteInstance {

    pub pos_x:  f32,
    pub pos_y:  f32,
    pub width:  f32,
    pub height: f32,

    pub tex_x:  f32,
    pub tex_y:  f32,
    pub tex_w:  f32,
    pub tex_h:  f32
}


impl Text {

    #[must_use]
    pub fn get_transform(&mut self) -> &mut Transform {

        &mut self.transform
    }


    pub fn new(text: String, font: Rc<RefCell<Font>>, font_size: f32) -> Self {

        let character = CharSpriteInstance {
            pos_x:  0.0,
            pos_y:  0.0,
            width:  80.0,
            height: 80.0,

            tex_x: 0.0,
            tex_y: 0.0,
            tex_w: 0.0,
            tex_h: 0.0
        };

        let characters: Vec<CharSpriteInstance> = [character].to_vec();

        let instance_buffer = Renderer::get_device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("Text character instance buffer"),
                contents: bytemuck::cast_slice(&characters),
                usage:    wgpu::BufferUsages::VERTEX
            }
        );

        Self {
            transform: Transform::new(Renderer::get_device()),
            text,
            font,
            font_size,
            render_pipeline:   get_default_text_render_pipeline(),
            character_sprites: characters,
            instance_buffer:   instance_buffer
        }
    }
}


impl Drawable for Text {

    fn draw(&self, render_target: &mut RenderTarget) {

        let camera = render_target.get_camera();

        let pass : &mut wgpu::RenderPass = render_target.get_render_pass();

        let texture = match self.font.borrow_mut().get_texture(self.font_size) {
            Ok(texture) => texture,
            Err(_) => { return; }
        };

        pass.set_pipeline(self.render_pipeline.as_ref());

        pass.set_bind_group(0, self.transform.get_bind_group(), &[]);

        pass.set_bind_group(1, camera.borrow().get_bind_group(), &[]);

        pass.set_bind_group(2, texture.get_bind_group(), &[]);

        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));

        pass.draw(0..6, 0..self.character_sprites.len() as _);
    }
}


pub fn get_default_text_render_pipeline() -> Arc<wgpu::RenderPipeline> {

    TEXT_RENDER_PIPELINE.get_or_init(|| Arc::new(create_default_text_render_pipeline())).clone()
}


fn create_default_text_render_pipeline() -> wgpu::RenderPipeline {

    let device = Renderer::get_device();

    let matrix_layout  = MatrixUniform::get_bind_group_layout(device);
    let texture_layout = Texture::get_default_bind_group_layout(device);

    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Default text pipeline layout descriptor"),
            bind_group_layouts: &[
                Some(&matrix_layout),  // Transformation matrix
                Some(&matrix_layout),  // Camera matrix
                Some(&texture_layout), // Texture
            ],
            immediate_size: 0
        });

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Default text shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("default_text_shader.wgsl").into())
    });

    let index_buffer_layout = wgpu::VertexBufferLayout {
        array_stride: std::mem::size_of::<CharSpriteInstance>() as wgpu::BufferAddress,
        step_mode:    wgpu::VertexStepMode::Instance,
        attributes: &[
            wgpu::VertexAttribute {
                offset:          0,
                shader_location: 0,
                format:          wgpu::VertexFormat::Float32x4
            },
            wgpu::VertexAttribute {
                offset:          0,
                shader_location: 1,
                format:          wgpu::VertexFormat::Float32x4
            }
        ]
    };

    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label:  Some("Default text render pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module:              &shader,
            entry_point:         Some("vs_main"),
            buffers:             &[index_buffer_layout],
            compilation_options: wgpu::PipelineCompilationOptions::default()
        },
        fragment: Some(wgpu::FragmentState {
            module:      &shader,
            entry_point: Some("fs_main"),
            targets:     &[Some(wgpu::ColorTargetState {
                format:     Renderer::get_default_surface_config().format,
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
        depth_stencil: Some(wgpu::DepthStencilState {
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
