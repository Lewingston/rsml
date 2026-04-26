
use wgpu::util::DeviceExt;
use cgmath::SquareMatrix;

use std::rc::Rc;
use std::sync::Arc;

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


impl Color {

    #[must_use]
    pub fn random() -> Self {

        use rand::RngExt;

        let mut rng = rand::rng();
        let r : u8 = rng.random();
        let g : u8 = rng.random();
        let b : u8 = rng.random();
        let a : u8 = 255;

        Self {
            r,
            g,
            b,
            a
        }
    }


    #[must_use]
    pub fn to_wgpu_color(&self) -> wgpu::Color {

        wgpu::Color {
            r: self.r as f64 / 255.0,
            g: self.g as f64 / 255.0,
            b: self.b as f64 / 255.0,
            a: self.a as f64 / 255.0
        }
    }


    #[must_use]
    pub fn to_srgb(&self) -> wgpu::Color {

        let convert = |c: u8| -> f64 {
            let c = c as f64 / 255.0;
            ((c + 0.055) / 1.055).powf(2.4)
        };

        wgpu::Color {
            r: convert(self.r),
            g: convert(self.g),
            b: convert(self.b),
            a: self.a as f64 / 255.0
        }
    }
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

    vertices: Vec<Vertex>,
    index_count: usize,

    render_pipeline: Arc<wgpu::RenderPipeline>,

    texture: Option<Rc<Texture>>,
}


impl Shape {


    #[must_use]
    pub fn get_transform(&mut self) -> &mut Transform {

        &mut self.transform
    }


    #[must_use]
    pub fn create_triangle() -> Self {

        let texture_pos = [0.0, 0.0];

        let vert = vec![
            Vertex { position: [ 0.0,  0.5, 0.0], texture_pos, color: Color{ r: 255, g:   0, b:   0, a: 255 }},
            Vertex { position: [-0.5, -0.5, 0.0], texture_pos, color: Color{ r:   0, g: 255, b:   0, a: 255 }},
            Vertex { position: [ 0.5, -0.5, 0.0], texture_pos, color: Color{ r:   0, g:   0, b: 255, a: 255 }}
        ];

        let indices: &[u16] = &[
            0, 1, 2
        ];

        Self {
            transform:       Transform::new(Renderer::get().get_device()),
            vertex_buffer:   Self::create_vertex_buffer(&vert),
            index_buffer:    Self::create_index_buffer(Renderer::get().get_device(), indices),
            vertices:        vert,
            index_count:     indices.len(),
            render_pipeline: Renderer::get().get_default_color_render_pipeline(),
            texture:         None,
        }
    }


    #[must_use]
    pub fn create_rectangle(width: f32, height: f32) -> Self {

        let color = Color{ r: 255, g: 0, b: 0, a: 255 };

        let vert = vec![
            Vertex { position: [-width / 2.0,  height / 2.0, 0.0], texture_pos: [0.0, 0.0], color },
            Vertex { position: [ width / 2.0,  height / 2.0, 0.0], texture_pos: [1.0, 0.0], color },
            Vertex { position: [ width / 2.0, -height / 2.0, 0.0], texture_pos: [1.0, 1.0], color },
            Vertex { position: [-width / 2.0, -height / 2.0, 0.0], texture_pos: [0.0, 1.0], color }
        ];

        let indices: &[u16] = &[
            2, 1, 0,
            0, 3, 2
        ];

        Self {
            transform:       Transform::new(Renderer::get().get_device()),
            vertex_buffer:   Self::create_vertex_buffer(&vert),
            index_buffer:    Self::create_index_buffer(Renderer::get().get_device(), indices),
            vertices:        vert,
            index_count:     indices.len(),
            render_pipeline: Renderer::get().get_default_color_render_pipeline(),
            texture:         None,
        }
    }


    #[must_use]
    pub fn create_sprite(
        width:   f32,
        height:  f32,
        texture: Rc<Texture>
    ) -> Self {

        let color = Color{ r: 255, g: 255, b: 255, a: 255 };

        let vert = vec![
            Vertex { position: [-width / 2.0,  height / 2.0, 0.0], texture_pos: [ 0.0, 0.0 ], color },
            Vertex { position: [ width / 2.0,  height / 2.0, 0.0], texture_pos: [ 1.0, 0.0 ], color },
            Vertex { position: [ width / 2.0, -height / 2.0, 0.0], texture_pos: [ 1.0, 1.0 ], color },
            Vertex { position: [-width / 2.0, -height / 2.0, 0.0], texture_pos: [ 0.0, 1.0 ], color }
        ];

        let indices: &[u16] = &[
            2, 1, 0,
            0, 3, 2
        ];

        Self {
            transform:       Transform::new(Renderer::get().get_device()),
            vertex_buffer:   Self::create_vertex_buffer(&vert),
            index_buffer:    Self::create_index_buffer(Renderer::get().get_device(), indices),
            vertices:        vert,
            index_count:     indices.len(),
            render_pipeline: Renderer::get().get_default_texture_render_pipeline(),
            texture:         Some(texture),
        }
    }


    pub fn set_color(&mut self, color: Color) {

        for vertex in &mut self.vertices {

            vertex.color = color;
        }

        self.vertex_buffer = Self::create_vertex_buffer(&self.vertices);
    }


    pub fn set_texture(&mut self, texture: Rc<Texture>) {

        self.texture         = Some(texture);
        self.render_pipeline = Renderer::get().get_default_texture_render_pipeline();
    }


    fn create_vertex_buffer(vertices: &[Vertex]) -> wgpu::Buffer {

        Renderer::get().get_device().create_buffer_init(
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
}


pub struct Transform {

    matrix:  cgmath::Matrix4<f32>,
    origin:   cgmath::Vector3<f32>,
    uniform:  MatrixUniform
}


impl Transform {


    #[must_use]
    pub fn new(device: &wgpu::Device) -> Self {

        let matrix  = cgmath::Matrix4::<f32>::identity();
        let origin   = cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: 0.0 };

        let uniform  = MatrixUniform::new(device, matrix);

        Self {
            matrix,
            origin,
            uniform
        }
    }


    #[must_use]
    pub fn get_bind_group(&self) -> &wgpu::BindGroup {

        self.uniform.get_bind_group()
    }


    pub fn translate(&mut self, v: cgmath::Vector3<f32>) {

        self.matrix =
            cgmath::Matrix4::<f32>::from_translation(v) * self.matrix;

        self.update();
    }


    pub fn rotate_x(&mut self, angle: cgmath::Rad<f32>) {

        self.matrix =
            self.matrix * cgmath::Matrix4::<f32>::from_angle_x(angle);

        self.update();
    }


    pub fn rotate_y(&mut self, angle: cgmath::Rad<f32>) {

        self.matrix =
            self.matrix * cgmath::Matrix4::<f32>::from_angle_y(angle);

        self.update();
    }


    pub fn rotate_z(&mut self, angle: cgmath::Rad<f32>) {

        self.matrix =
            self.matrix * cgmath::Matrix4::<f32>::from_angle_z(angle);

        self.update();
    }


    pub fn scale(&mut self, v: cgmath::Vector3<f32>) {

        self.matrix =
            self.matrix * cgmath::Matrix4::<f32>::from_nonuniform_scale(v.x, v.y, v.z);

        self.update();
    }


    pub fn move_origin(&mut self, v: cgmath::Vector3<f32>) {

        self.origin += v;
        self.update();
    }


    pub fn set_origin(&mut self, v: cgmath::Vector3<f32>) {

        self.origin = v;
        self.update();
    }


    fn update(&self) {

        let move_to_origin = cgmath::Matrix4::<f32>::from_translation(-self.origin);
        let move_back      = cgmath::Matrix4::<f32>::from_translation(self.origin);

        let matrix = move_back * self.matrix * move_to_origin;

        self.uniform.update(matrix);
    }
}


impl Drawable for Shape {

    fn draw(&self, render_target: &mut RenderTarget) {

        let camera = render_target.get_camera();

        let pass : &mut wgpu::RenderPass = render_target.get_render_pass();

        pass.set_pipeline(self.render_pipeline.as_ref());

        pass.set_bind_group(0, self.transform.get_bind_group(), &[]);

        pass.set_bind_group(1, camera.borrow().get_bind_group(), &[]);

        match &self.texture {
            Some(texture) => {
                pass.set_bind_group(2, texture.get_bind_group(), &[]);
            }
            None => {}
        };

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);

        let index_count = u32::try_from(self.index_count).unwrap_or_default();

        pass.draw_indexed(0..index_count, 0, 0..1);
    }
}
