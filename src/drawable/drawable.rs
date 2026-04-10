
use wgpu::util::DeviceExt;

use std::rc::Rc;

use crate::renderer::Renderer;
use crate::renderer::render_target::RenderTarget;
use crate::renderer::camera::Camera;
use crate::drawable::texture::Texture;


pub trait Drawable {

    fn draw(&self, render_target: &mut RenderTarget);

}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ColorVertex {
    position: [f32; 3],
    color:    Color
}


impl ColorVertex {

    #[must_use]
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode:    wgpu::VertexStepMode::Vertex,
            attributes:   &[
                wgpu::VertexAttribute { // Position
                    offset:          0,
                    shader_location: 0,
                    format:          wgpu::VertexFormat::Float32x3
                },
                wgpu::VertexAttribute { // Color
                    offset:          std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format:          wgpu::VertexFormat::Unorm8x4
                }
            ]
        }
    }
}


#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct TextureVertex {
    position:    [f32; 3],
    texture_pos: [f32; 2]
}


impl TextureVertex {

    #[must_use]
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode:    wgpu::VertexStepMode::Vertex,
            attributes:   &[
                wgpu::VertexAttribute {
                    offset:          0,
                    shader_location: 0,
                    format:          wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset:          std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format:          wgpu::VertexFormat::Float32x2
                }
            ]
        }
    }
}


pub struct Shape {

    vertex_buffer: wgpu::Buffer,
    index_buffer:  wgpu::Buffer,

    _vertices: Vec<ColorVertex>,
    index_count: usize,

    render_pipeline: Rc<wgpu::RenderPipeline>
}


impl Shape {

    #[must_use]
    pub fn create_triangle(renderer: &Renderer) -> Self {

        let vertices = vec![
            ColorVertex { position: [ 0.0,  0.5, 0.0], color: Color{ r: 255, g:   0, b:   0, a: 255 }},
            ColorVertex { position: [-0.5, -0.5, 0.0], color: Color{ r:   0, g: 255, b:   0, a: 255 }},
            ColorVertex { position: [ 0.5, -0.5, 0.0], color: Color{ r:   0, g:   0, b: 255, a: 255 }}
        ];

        let indices: &[u16] = &[
            0, 1, 2
        ];

        let device = renderer.get_device();

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("Triangle vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage:    wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("Triangle index buffer"),
                contents: bytemuck::cast_slice(indices),
                usage:    wgpu::BufferUsages::INDEX
            }
        );

        let index_count = indices.len();

        let render_pipeline = renderer.get_default_color_render_pipeline();

        Self {
            vertex_buffer,
            index_buffer,
            _vertices: vertices,
            index_count,
            render_pipeline
        }
    }


    #[must_use]
    pub fn create_rectangle(renderer: &Renderer, width: f32, height: f32) -> Self {

        let color = Color{ r: 255, g: 0, b: 0, a: 255 };

        let vertices = vec![
            ColorVertex { position: [-width / 2.0,  height / 2.0, 0.0], color },
            ColorVertex { position: [ width / 2.0,  height / 2.0, 0.0], color },
            ColorVertex { position: [ width / 2.0, -height / 2.0, 0.0], color },
            ColorVertex { position: [-width / 2.0, -height / 2.0, 0.0], color }
        ];

        let indices: &[u16] = &[
            2, 1, 0,
            0, 3, 2
        ];

        let device = renderer.get_device();

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("Rectangle vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage:    wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("Rectangle vertex buffer"),
                contents: bytemuck::cast_slice(indices),
                usage:    wgpu::BufferUsages::INDEX
            }
        );

        let index_count = indices.len();

        let render_pipeline = renderer.get_default_color_render_pipeline();

        Self {
            vertex_buffer,
            index_buffer,
            _vertices: vertices,
            index_count,
            render_pipeline
        }
    }
}


impl Drawable for Shape {

    fn draw(&self, render_target: &mut RenderTarget) {

        let camera = render_target.get_camera();

        let pass : &mut wgpu::RenderPass  = render_target.get_render_pass();

        pass.set_pipeline(self.render_pipeline.as_ref());

        pass.set_bind_group(0, camera.get_bind_group(), &[]);

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        let index_count = u32::try_from(self.index_count).unwrap_or_default();

        pass.draw_indexed(0..index_count, 0, 0..1);
    }
}


pub struct Sprite {

    vertex_buffer: wgpu::Buffer,
    index_buffer:  wgpu::Buffer,

    index_count:  usize,

    render_pipeline: Rc<wgpu::RenderPipeline>,

    _texture: Rc<Texture>,

    texture_bind_group: wgpu::BindGroup
}


impl Sprite {


    #[must_use]
    pub fn new(
        renderer: &Renderer,
        texture:  Rc<Texture>
    ) -> Self {

        let width = 1.0;
        let height = 1.0;

        let vertices = vec![
            TextureVertex { position: [-width / 2.0,  height / 2.0, 0.0], texture_pos: [ 0.0, 0.0 ]},
            TextureVertex { position: [ width / 2.0,  height / 2.0, 0.0], texture_pos: [ 1.0, 0.0 ]},
            TextureVertex { position: [ width / 2.0, -height / 2.0, 0.0], texture_pos: [ 1.0, 1.0 ]},
            TextureVertex { position: [-width / 2.0, -height / 2.0, 0.0], texture_pos: [ 0.0, 1.0 ]}
        ];

        let indices: &[u16] = &[
            2, 1, 0,
            0, 3, 2
        ];

        let device = renderer.get_device();

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("Sprite vertex buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage:    wgpu::BufferUsages::VERTEX
            }
        );

        let index_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("Sprite vertex buffer"),
                contents: bytemuck::cast_slice(indices),
                usage:    wgpu::BufferUsages::INDEX
            }
        );

        let index_count = indices.len();

        let render_pipeline = renderer.get_default_texture_render_pipeline();

        let texture_bind_group = Self::create_bind_group(
            renderer,
            texture.get_view(),
            texture.get_sampler());

        Self {
            vertex_buffer,
            index_buffer,
            index_count,
            render_pipeline,
            _texture: texture,
            texture_bind_group
        }
    }


    fn create_bind_group(
        renderer:        &Renderer,
        texture_view:    &wgpu::TextureView,
        texture_sampler: &wgpu::Sampler
    ) -> wgpu::BindGroup {

        let layout = Texture::get_default_bind_group_layout(renderer.get_device());

        renderer.get_device().create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(texture_view)
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(texture_sampler)
                    }
                ],
                label: Some("texture bind group")
            }
        )
    }
}


impl Drawable for Sprite {

    fn draw(&self, render_target: &mut RenderTarget) {

        let camera = render_target.get_camera();

        let pass : &mut wgpu::RenderPass = render_target.get_render_pass();

        pass.set_pipeline(self.render_pipeline.as_ref());

        pass.set_bind_group(0, &self.texture_bind_group, &[]);

        pass.set_bind_group(1, camera.get_bind_group(), &[]);

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        let index_count = u32::try_from(self.index_count).unwrap_or_default();

        pass.draw_indexed(0..index_count, 0, 0..1);
    }
}
