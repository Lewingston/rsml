
use crate::renderer::uniform::MatrixUniform;
use crate::renderer::renderer::Renderer;


#[derive(Clone)]
pub enum ProjectionMode {

    PERSPECTIVE,
    ORTHOGRAPHIC
}


#[derive(Clone)]
pub struct CameraParameters {

    pub pos:        cgmath::Point3<f32>,
    pub target:     cgmath::Point3<f32>,
    pub up:         cgmath::Vector3<f32>,
    pub width:      u32,
    pub height:     u32,
    pub fovy:       f32,
    pub znear:      f32,
    pub zfar:       f32,
    pub projection: ProjectionMode
}


pub struct Camera {

    parameters: CameraParameters,
    uniform:    MatrixUniform
}


const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::from_cols(
    cgmath::Vector4::new(1.0, 0.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 1.0, 0.0, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 0.0),
    cgmath::Vector4::new(0.0, 0.0, 0.5, 1.0)
);


impl Camera {


    #[must_use]
    pub fn get_bind_group(&self) -> &wgpu::BindGroup {

        self.uniform.get_bind_group()
    }


    #[must_use]
    pub fn get_parameters(&self) -> &CameraParameters {

        &self.parameters
    }


    #[must_use]
    pub fn new(width: u32, height: u32) -> Self {

        let parameters = CameraParameters::default(width, height);

        let uniform = MatrixUniform::new(Renderer::get_device(), parameters.get_matrix());

        Self {
            parameters,
            uniform
        }
    }


    pub fn set_parameters(&mut self, parameters: CameraParameters) {

        self.parameters = parameters;
        self.uniform.update(self.parameters.get_matrix());
    }
}


impl CameraParameters {


    #[must_use]
    pub fn default(width: u32, height: u32) -> Self {

        Self {
            pos:        (0.0, 0.0, 10.0).into(),
            target:     (0.0, 0.0, 0.0).into(),
            up:         cgmath::Vector3::unit_y(),
            width,
            height,
            fovy:       45.0,
            znear:      0.01,
            zfar:       100.0,
            projection: ProjectionMode::PERSPECTIVE
        }
    }


    fn get_matrix(&self) -> cgmath::Matrix4<f32> {

        let view = cgmath::Matrix4::look_at_rh(self.pos, self.target, self.up);

        let proj = match self.projection {
            ProjectionMode::PERSPECTIVE => {

                let aspect = self.width as f32 / self.height as f32;
                cgmath::perspective(cgmath::Deg(self.fovy), aspect, self.znear, self.zfar)
            }
            ProjectionMode::ORTHOGRAPHIC => {

                cgmath::ortho(
                    self.width  as f32 / -2.0,
                    self.width  as f32 / 2.0,
                    self.height as f32 / -2.0,
                    self.height as f32 / 2.0,
                    self.znear,
                    self.zfar)
            }
        };

        OPENGL_TO_WGPU_MATRIX * proj * view
    }
}
