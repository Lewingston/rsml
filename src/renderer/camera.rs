
use wgpu::util::DeviceExt;


pub struct CameraParameters {

    pub pos:    cgmath::Point3<f32>,
    pub target: cgmath::Point3<f32>,
    pub up:     cgmath::Vector3<f32>,
    pub aspect: f32,
    pub fovy:   f32,
    pub znear:  f32,
    pub zfar:   f32,
}


pub struct Camera {

    parameters: CameraParameters,
    _buffer:    wgpu::Buffer,
    bind_group: wgpu::BindGroup
}


const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0)
);


impl Camera {


    #[must_use]
    pub fn get_bind_group(&self) -> &wgpu::BindGroup {

        &self.bind_group
    }


    #[must_use]
    pub fn new(device: &wgpu::Device, aspect_ratio: f32) -> Self {

        let parameters = CameraParameters::default(aspect_ratio);

        let buffer = Self::create_buffer(device, &parameters);

        let bind_group = Self::create_bind_group(device, &buffer);

        Self {
            parameters,
            _buffer : buffer,
            bind_group,
        }
    }


    #[must_use]
    pub fn create_buffer(device: &wgpu::Device, parameters: &CameraParameters) -> wgpu::Buffer {

        device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Camera buffer"),
                contents: bytemuck::cast_slice(&[parameters.get_camera_matrix()]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST
            }
        )
    }


    #[must_use]
    pub fn create_bind_group(
        device: &wgpu::Device,
        buffer: &wgpu::Buffer)
    -> wgpu::BindGroup {

        let layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty:                 wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size:   None,
                        },
                        count: None
                    }
                ],
                label: Some("Camera bind group layout")
            }
        );

        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }
            ],
            label: Some("Camera bind group")
        })
    }


    #[must_use]
    pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {

        device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::VERTEX,
                        ty: wgpu::BindingType::Buffer {
                            ty:                 wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size:   None,
                        },
                        count: None
                    }
                ],
                label: Some("Camera bind group layout")
            }
        )
    }
}


impl CameraParameters {


    #[must_use]
    pub fn default(aspect_ratio: f32) -> Self {

        Self {
            pos:    (0.0, 0.0, 2.0).into(),
            target: (0.0, 0.0, 0.0).into(),
            up:     cgmath::Vector3::unit_y(),
            aspect: aspect_ratio,
            fovy:   45.0,
            znear:  0.01,
            zfar:   100.0,
        }
    }


    fn get_camera_matrix(&self) -> CameraUniform {

        let view   = cgmath::Matrix4::look_at_rh(self.pos, self.target, self.up);
        let proj   = cgmath::perspective(cgmath::Deg(self.fovy), self.aspect, self.znear, self.zfar);
        let matrix = OPENGL_TO_WGPU_MATRIX * proj * view;

        CameraUniform::from_matrix(matrix)
    }
}


#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
struct CameraUniform {
    view_proj_matrix: [[f32; 4]; 4]
}


impl CameraUniform {

    fn from_matrix(matrix: cgmath::Matrix4<f32>) -> Self {

        Self {
            view_proj_matrix: matrix.into()
        }
    }
}
