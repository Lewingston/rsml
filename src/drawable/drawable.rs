
use wgpu::util::DeviceExt;
use cgmath::SquareMatrix;

use std::rc::Rc;

use crate::renderer::Renderer;
use crate::renderer::render_target::RenderTarget;
use crate::renderer::uniform::MatrixUniform;

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
pub struct Vertex {
    position:    [f32; 3],
    texture_pos: [f32; 2],
    color:       Color
}


impl Vertex {

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
                wgpu::VertexAttribute { // Texture coordinates
                    offset:          std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format:          wgpu::VertexFormat::Float32x2
                },
                wgpu::VertexAttribute { // Color
                    offset:          std::mem::size_of::<[f32; 5]>() as wgpu::BufferAddress,
                    shader_location: 2,
                    format:          wgpu::VertexFormat::Unorm8x4
                }
            ]
        }
    }
}


pub struct Shape {

    transform: Transform,

    vertex_buffer: wgpu::Buffer,
    index_buffer:  wgpu::Buffer,

    _vertices: Vec<Vertex>,
    index_count: usize,

    render_pipeline: Rc<wgpu::RenderPipeline>,

    _texture: Option<Rc<Texture>>,
    texture_bind_group: Option<wgpu::BindGroup>
}


pub struct Transform {

    matrix:  cgmath::Matrix4<f32>,
    uniform: MatrixUniform
}


impl Shape {


    #[must_use]
    pub fn get_transform(&mut self) -> &mut Transform {

        &mut self.transform
    }


    #[must_use]
    pub fn create_triangle(renderer: &Renderer) -> Self {

        let texture_pos = [0.0, 0.0];

        let vertices = vec![
            Vertex { position: [ 0.0,  0.5, 0.0], texture_pos, color: Color{ r: 255, g:   0, b:   0, a: 255 }},
            Vertex { position: [-0.5, -0.5, 0.0], texture_pos, color: Color{ r:   0, g: 255, b:   0, a: 255 }},
            Vertex { position: [ 0.5, -0.5, 0.0], texture_pos, color: Color{ r:   0, g:   0, b: 255, a: 255 }}
        ];

        let indices: &[u16] = &[
            0, 1, 2
        ];

        Self {
            transform:          Transform::new(renderer.get_device()),
            vertex_buffer:      Self::create_vertex_buffer(renderer.get_device(), &vertices),
            index_buffer:       Self::create_index_buffer(renderer.get_device(), indices),
            _vertices:          vertices,
            index_count:        indices.len(),
            render_pipeline:    renderer.get_default_color_render_pipeline(),
            _texture:           None,
            texture_bind_group: None
        }
    }


    #[must_use]
    pub fn create_rectangle(renderer: &Renderer, width: f32, height: f32) -> Self {

        let color = Color{ r: 255, g: 0, b: 0, a: 255 };
        let texture_pos = [0.0, 0.0];

        let vertices = vec![
            Vertex { position: [-width / 2.0,  height / 2.0, 0.0], texture_pos, color },
            Vertex { position: [ width / 2.0,  height / 2.0, 0.0], texture_pos, color },
            Vertex { position: [ width / 2.0, -height / 2.0, 0.0], texture_pos, color },
            Vertex { position: [-width / 2.0, -height / 2.0, 0.0], texture_pos, color }
        ];

        let indices: &[u16] = &[
            2, 1, 0,
            0, 3, 2
        ];

        Self {
            transform:          Transform::new(renderer.get_device()),
            vertex_buffer:      Self::create_vertex_buffer(renderer.get_device(), &vertices),
            index_buffer:       Self::create_index_buffer(renderer.get_device(), indices),
            _vertices:          vertices,
            index_count:        indices.len(),
            render_pipeline:    renderer.get_default_color_render_pipeline(),
            _texture:           None,
            texture_bind_group: None
        }
    }


    #[must_use]
    pub fn create_sprite(
        renderer: &Renderer,
        width:    f32,
        height:   f32,
        texture:  Rc<Texture>
    ) -> Self {

        let color = Color{ r: 255, g: 255, b: 255, a: 255 };

        let vertices = vec![
            Vertex { position: [-width / 2.0,  height / 2.0, 0.0], texture_pos: [ 0.0, 0.0 ], color },
            Vertex { position: [ width / 2.0,  height / 2.0, 0.0], texture_pos: [ 1.0, 0.0 ], color },
            Vertex { position: [ width / 2.0, -height / 2.0, 0.0], texture_pos: [ 1.0, 1.0 ], color },
            Vertex { position: [-width / 2.0, -height / 2.0, 0.0], texture_pos: [ 0.0, 1.0 ], color }
        ];

        let indices: &[u16] = &[
            2, 1, 0,
            0, 3, 2
        ];

        let texture_bind_group = Self::create_texture_bind_group(
            renderer.get_device(),
            texture.get_view(),
            texture.get_sampler()
        );

        Self {
            transform:          Transform::new(renderer.get_device()),
            vertex_buffer:      Self::create_vertex_buffer(renderer.get_device(), &vertices),
            index_buffer:       Self::create_index_buffer(renderer.get_device(), indices),
            _vertices:          vertices,
            index_count:        indices.len(),
            render_pipeline:    renderer.get_default_texture_render_pipeline(),
            _texture:           Some(texture),
            texture_bind_group: Some(texture_bind_group)
        }
    }


    fn create_vertex_buffer(device: &wgpu::Device, vertices: &Vec<Vertex>) -> wgpu::Buffer {

        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("vertex buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage:    wgpu::BufferUsages::VERTEX
            }
        )
    }


    fn create_index_buffer(device: &wgpu::Device, indices: &[u16]) -> wgpu::Buffer {

        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("index buffer"),
                contents: bytemuck::cast_slice(indices),
                usage:    wgpu::BufferUsages::INDEX
            }
        )
    }


    fn create_texture_bind_group(
        device:          &wgpu::Device,
        texture_view:    &wgpu::TextureView,
        texture_sampler: &wgpu::Sampler
    ) -> wgpu::BindGroup {

        device.create_bind_group(
            &wgpu::BindGroupDescriptor {
                layout: &Texture::get_default_bind_group_layout(device),
                entries: &[
                    wgpu::BindGroupEntry {
                        binding:  0,
                        resource: wgpu::BindingResource::TextureView(texture_view)
                    },
                    wgpu::BindGroupEntry {
                        binding:  1,
                        resource: wgpu::BindingResource::Sampler(texture_sampler)
                    }
                ],
                label: Some("texture bind group")
            }
        )
    }
}


impl Transform {


    pub fn new(device: &wgpu::Device) -> Self {

        let matrix  = cgmath::Matrix4::<f32>::identity();
        let uniform = MatrixUniform::new(device, matrix);

        Self {
            matrix,
            uniform
        }
    }


    pub fn get_bind_group(&self) -> &wgpu::BindGroup {

        self.uniform.get_bind_group()
    }


    pub fn translate(&mut self, v: cgmath::Vector3<f32>) {

        self.matrix =
            cgmath::Matrix4::<f32>::from_translation(v) * self.matrix;
    }


    pub fn rotate_x(&mut self, angle: cgmath::Rad<f32>) {

        self.matrix =
            self.matrix * cgmath::Matrix4::<f32>::from_angle_x(angle);
    }


    pub fn rotate_y(&mut self, angle: cgmath::Rad<f32>) {

        self.matrix =
            self.matrix * cgmath::Matrix4::<f32>::from_angle_y(angle);
    }


    pub fn rotate_z(&mut self, angle: cgmath::Rad<f32>) {

        self.matrix =
            self.matrix * cgmath::Matrix4::<f32>::from_angle_z(angle);
    }


    pub fn scale(&mut self, v: cgmath::Vector3<f32>) {

        self.matrix =
            self.matrix * cgmath::Matrix4::<f32>::from_nonuniform_scale(v.x, v.y, v.z);
    }


    pub fn update(&self, queue: &wgpu::Queue) {

        self.uniform.update(queue, self.matrix);
    }
}


impl Drawable for Shape {

    fn draw(&self, render_target: &mut RenderTarget) {

        let camera = render_target.get_camera();

        let pass : &mut wgpu::RenderPass = render_target.get_render_pass();

        pass.set_pipeline(self.render_pipeline.as_ref());

        pass.set_bind_group(0, self.transform.get_bind_group(), &[]);

        pass.set_bind_group(1, camera.borrow().get_bind_group(), &[]);

        pass.set_bind_group(2, &self.texture_bind_group, &[]);

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        let index_count = u32::try_from(self.index_count).unwrap_or_default();

        pass.draw_indexed(0..index_count, 0, 0..1);
    }
}
