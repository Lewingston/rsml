
use wgpu::util::DeviceExt;

use std::rc::Rc;

use crate::app::renderer::Renderer;


pub trait Drawable {

    fn get_vertex_buffer(&self) -> Option<&wgpu::Buffer>;

    fn get_vertex_count(&self) -> u32;

    fn get_index_buffer(&self) -> Option<&wgpu::Buffer>;

    fn get_index_count(&self) -> u32;

    fn get_pipeline(&self) -> &wgpu::RenderPipeline;

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

    vertices: Vec<ColorVertex>,
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
            vertices,
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
            vertices,
            index_count,
            render_pipeline
        }
    }
}


impl Drawable for Shape {

    fn get_vertex_buffer(&self) -> Option<&wgpu::Buffer> {

        Some(&self.vertex_buffer)
    }

    fn get_vertex_count(&self) -> u32 {

        u32::try_from(self.vertices.len()).unwrap_or_default()
    }

    fn get_index_buffer(&self) -> Option<&wgpu::Buffer> {

        Some(&self.index_buffer)
    }

    fn get_index_count(&self) -> u32 {

        u32::try_from(self.index_count).unwrap_or_default()
    }

    fn get_pipeline(&self) -> &wgpu::RenderPipeline {

        self.render_pipeline.as_ref()
    }
}


/*
pub struct Sprite {

}
*/
