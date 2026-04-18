
use wgpu::util::DeviceExt;


pub struct MatrixUniform {

    buffer:     wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}


impl MatrixUniform {


    #[must_use]
    pub fn new(device: &wgpu::Device, matrix: cgmath::Matrix4::<f32>) -> Self {

        let buffer     = Self::create_buffer(device, matrix);
        let bind_group = Self::create_bind_group(device, &buffer);

        Self {
            buffer,
            bind_group
        }
    }


    pub fn update(&self, queue: &wgpu::Queue, matrix: cgmath::Matrix4<f32>) {

        queue.write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[Self::get_matrix(matrix)]));
    }


    #[must_use]
    pub fn get_bind_group(&self) -> &wgpu::BindGroup {

        &self.bind_group
    }


    #[must_use]
    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {

        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding:    0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty:         wgpu::BindingType::Buffer {
                            ty:                 wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size:   None,
                        },
                        count: None
                    }
                ],
                label: Some("matrix bind group layout")
            }
        )
    }


    fn create_buffer(device: &wgpu::Device, matrix: cgmath::Matrix4::<f32>) -> wgpu::Buffer {

        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("matrix buffer"),
                contents: bytemuck::cast_slice(&[Self::get_matrix(matrix)]),
                usage:    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        )
    }


    fn create_bind_group(device: &wgpu::Device, buffer: &wgpu::Buffer) -> wgpu::BindGroup {

        let layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None
                        },
                        count: None
                    }
                ],
                label: Some("matrix bind group layout")
            }
        );

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout:  &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding:  0,
                    resource: buffer.as_entire_binding()
                }
            ],
            label: Some("matrix bind group")
        })
    }


    fn get_matrix(matrix: cgmath::Matrix4<f32>) -> [[f32; 4]; 4] {

        matrix.into()
    }
}
