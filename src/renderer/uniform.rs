
use crate::renderer::renderer::Renderer;
use crate::drawable::drawable::Color;

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


    pub fn update(&self, matrix: cgmath::Matrix4<f32>) {

        Renderer::get().get_queue().write_buffer(&self.buffer, 0, bytemuck::cast_slice(&[Self::get_matrix(matrix)]));
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

        let layout = Self::get_bind_group_layout(device);

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


pub struct ColorUniform {

    buffer:     wgpu::Buffer,
    bind_group: wgpu::BindGroup
}


impl ColorUniform {


    #[must_use]
    pub fn new(color: Color) -> Self {

        let renderer   = Renderer::get();
        let device     = renderer.get_device();
        let buffer     = Self::create_buffer(device, color);
        let bind_group = Self::create_bind_group(device, &buffer);

        Self {
            buffer,
            bind_group
        }
    }


    pub fn update(&self, color: Color) {

        Renderer::get().get_queue().write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&Self::get_color_vector(color))
        );
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
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty:         wgpu::BindingType::Buffer {
                            ty:                 wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size:   None
                        },
                        count: None
                    }
                ],
                label: Some("color bind group layout")
            }
        )
    }


    fn create_buffer(device: &wgpu::Device, color: Color) -> wgpu::Buffer {

        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("color buffer"),
                contents: bytemuck::cast_slice(&Self::get_color_vector(color)),
                usage:    wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        )
    }


    fn create_bind_group(device: &wgpu::Device, buffer: &wgpu::Buffer) -> wgpu::BindGroup {

        let layout = Self::get_bind_group_layout(device);

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout:  &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding:  0,
                    resource: buffer.as_entire_binding()
                }
            ],
            label:   Some("color bind group")
        })
    }


    fn get_color_vector(color: Color) -> [f32; 4]
    {
        let r = color.r as f32 / 255.0;
        let g = color.g as f32 / 255.0;
        let b = color.b as f32 / 255.0;
        let a = color.a as f32 / 255.0;

        [r, g, b, a]
    }

}
