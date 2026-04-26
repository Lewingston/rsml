
use wgpu::util::DeviceExt;

use crate::renderer::renderer::Renderer;
use crate::renderer::render_target::RenderTarget;
use crate::renderer::uniform::MatrixUniform;
use crate::renderer::uniform::ColorUniform;
use crate::drawable::drawable::Drawable;
use crate::drawable::drawable::Transform;
use crate::drawable::drawable::Color;
use crate::drawable::texture::Texture;
use crate::drawable::font::Font;
use crate::drawable::font::CharParams;

use std::sync::Arc;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashSet;

use once_cell::unsync::OnceCell;

//static TEXT_RENDER_PIPELINE: std::sync::OnceLock<Arc<wgpu::RenderPipeline>> = std::sync::OnceLock::new();

thread_local! {
    static TEXT_RENDER_PIPELINE: OnceCell<Arc<wgpu::RenderPipeline>> = OnceCell::new();
}


pub struct Text {

    transform: Transform,
    font:      Rc<RefCell<Font>>,
    font_size: f32,

    render_pipeline: Arc<wgpu::RenderPipeline>,

    character_sprites: Vec<CharSpriteInstance>,

    instance_buffer: wgpu::Buffer,

    color_uniform: ColorUniform,

    layout_settings: fontdue::layout::LayoutSettings
}


#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CharSpriteInstance {

    pub pos_x:  f32,
    pub pos_y:  f32,
    pub width:  f32,
    pub height: f32,

    pub tex_x:  u32,
    pub tex_y:  u32,
    pub tex_w:  u32,
    pub tex_h:  u32
}


impl Text {

    #[must_use]
    pub fn get_transform(&mut self) -> &mut Transform {

        &mut self.transform
    }


    pub fn new(
        text:      &str,
        font:      Rc<RefCell<Font>>,
        font_size: f32,
        layout:    Option<fontdue::layout::LayoutSettings>
    ) -> Self {

        let layout = layout.unwrap_or(fontdue::layout::LayoutSettings::default());

        let characters = Self::calculate_layout(text, &mut *font.borrow_mut(), font_size, &layout);

        let instance_buffer = Self::create_instance_buffer(&characters);

        let color_uniform = ColorUniform::new(Color { r: 0, g: 0, b: 0, a: 255 });

        Self {
            transform: Transform::new(Renderer::get().get_device()),
            font,
            font_size,
            render_pipeline:   get_default_text_render_pipeline(),
            character_sprites: characters,
            instance_buffer:   instance_buffer,
            color_uniform,
            layout_settings:   layout
        }
    }


    pub fn set_color(&self, color: Color) {

        self.color_uniform.update(color);
    }


    pub fn set_text(&mut self, text: &str) {

        self.character_sprites = Self::calculate_layout(
            text,
            &mut *self.font.borrow_mut(),
            self.font_size,
            &self.layout_settings);

        self.instance_buffer = Self::create_instance_buffer(&self.character_sprites);
    }


    fn calculate_layout(
        text:            &str,
        font:            &mut Font,
        font_size:       f32,
        layout_settings: &fontdue::layout::LayoutSettings
    ) -> Vec<CharSpriteInstance> {

        use fontdue::layout::{
            Layout,
            CoordinateSystem,
            TextStyle
        };

        let mut layout : Layout = Layout::new(CoordinateSystem::PositiveYUp);
        layout.reset(layout_settings);

        layout.append(
            &[font.get_fontdue_font()],
            &TextStyle::new(text, font_size, 0)
        );

        let unique_chars: HashSet<char> = text.chars().collect();
        for c in unique_chars {
            _ = font.get_char(c, font_size);
        }

        layout.glyphs().iter().map(|glyph| {

            let char_params = match font.get_char(glyph.parent, font_size) {
                Ok(params) => { params }
                Err(_) => { &CharParams { x: 0, y: 0, w: 0, h: 0 } }
            };

            CharSpriteInstance {
                pos_x:  glyph.x,
                pos_y:  glyph.y,
                width:  glyph.width  as f32,
                height: glyph.height as f32,

                tex_x:  char_params.x,
                tex_y:  char_params.y,
                tex_w:  char_params.w,
                tex_h:  char_params.h
            }

        }).collect()
    }


    fn create_instance_buffer(char_sprites: &Vec<CharSpriteInstance>) -> wgpu::Buffer {

        Renderer::get().get_device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some( "Text character instance buffer"),
                contents: bytemuck::cast_slice(char_sprites),
                usage:    wgpu::BufferUsages::VERTEX
            }
        )
    }
}


impl Drawable for Text {

    fn draw(&self, render_target: &mut RenderTarget) {

        if self.character_sprites.is_empty() {
            return;
        }

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

        pass.set_bind_group(3, self.color_uniform.get_bind_group(), &[]);

        pass.set_vertex_buffer(0, self.instance_buffer.slice(..));

        pass.draw(0..6, 0..self.character_sprites.len() as _);
    }
}


pub fn get_default_text_render_pipeline() -> Arc<wgpu::RenderPipeline> {

    //TEXT_RENDER_PIPELINE.get_or_init(|| Arc::new(create_default_text_render_pipeline())).clone()

    TEXT_RENDER_PIPELINE.with(|p| {
        p.get_or_init(|| Arc::new(create_default_text_render_pipeline())).clone()
    })
}


fn create_default_text_render_pipeline() -> wgpu::RenderPipeline {

    let renderer = Renderer::get();
    let device   = renderer.get_device();

    let matrix_layout  = MatrixUniform::get_bind_group_layout(device);
    let texture_layout = Texture::get_default_bind_group_layout(device);
    let color_layout   = ColorUniform::get_bind_group_layout(device);

    let pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Default text pipeline layout descriptor"),
            bind_group_layouts: &[
                Some(&matrix_layout),  // Transformation matrix
                Some(&matrix_layout),  // Camera matrix
                Some(&texture_layout), // Texture
                Some(&color_layout),   // Text color
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
                offset:          std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                shader_location: 1,
                format:          wgpu::VertexFormat::Uint32x4
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
                format:     Renderer::get().get_default_surface_config().format,
                blend:      Some(wgpu::BlendState::ALPHA_BLENDING),
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
