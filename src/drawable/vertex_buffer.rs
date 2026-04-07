
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color:    [f32; 3]
}


pub struct VertexBuffer {

    buffer: wgpu::Buffer
}


impl VertexBuffer {


    #[must_use]
    pub fn new (
        device:   &wgpu::Device,
        vertices: &'static[Vertex]
    ) -> Self {

        let buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage:    wgpu::BufferUsages::VERTEX
            }
        );

        Self { buffer }
    }


    #[must_use]
    pub fn get_buffer(&self) -> &wgpu::Buffer {

        &self.buffer
    }


    #[must_use]
    pub fn create_triangle(device: &wgpu::Device) -> Self {

        let vertices = &[
            Vertex { position: [ 0.0,  0.5, 0.0], color: [1.0, 0.0, 0.0] },
            Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
            Vertex { position: [ 0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] }
        ];

        Self::new(device, vertices)
    }


    #[must_use]
    pub fn get_layout() -> wgpu::VertexBufferLayout<'static> {

        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute { // Position
                    offset:          0,
                    shader_location: 0,
                    format:          wgpu::VertexFormat::Float32x3
                },
                wgpu::VertexAttribute { // Color
                    offset:          std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format:          wgpu::VertexFormat::Float32x3
                }
            ]
        }
    }
}
