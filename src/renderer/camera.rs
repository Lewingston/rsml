
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
    pub width:      f32,
    pub height:     f32,
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
    pub fn new(width: f32, height: f32) -> Self {

        let parameters = CameraParameters::default(width, height);

        let uniform = MatrixUniform::new(Renderer::get().get_device(), parameters.get_matrix());

        Self {
            parameters,
            uniform
        }
    }


    pub fn set_parameters(&mut self, parameters: CameraParameters) {

        self.parameters = parameters;
        self.update();
    }


    pub fn set_projection_mode(&mut self, mode: ProjectionMode) {

        self.parameters.projection = mode;
        self.update();
    }


    fn update(&self) {

        self.uniform.update(self.parameters.get_matrix());
    }
}


impl CameraParameters {

    #[must_use]
    pub fn default(width: f32, height: f32) -> Self {

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

    #[must_use]
    pub fn get_left(&self) -> f32 { self.width / -2.0 }

    #[must_use]
    pub fn get_right(&self) -> f32 { self.width / 2.0 }

    #[must_use]
    pub fn get_top(&self) -> f32 { self.height / 2.0 }

    #[must_use]
    pub fn get_bottom(&self) -> f32 { self.height / -2.0 }

    #[must_use]
    fn get_matrix(&self) -> cgmath::Matrix4<f32> {

        let view = self.get_view_matrix();

        let proj = self.get_projection_matrix();

        OPENGL_TO_WGPU_MATRIX * proj * view
    }

    #[must_use]
    fn get_view_matrix(&self) -> cgmath::Matrix4<f32> {

        cgmath::Matrix4::look_at_rh(self.pos, self.target, self.up)
    }

    #[must_use]
    fn get_projection_matrix(&self) -> cgmath::Matrix4<f32> {

        match self.projection {
            ProjectionMode::PERSPECTIVE => {

                let aspect = self.width as f32 / self.height as f32;
                cgmath::perspective(cgmath::Deg(self.fovy), aspect, self.znear, self.zfar)
            }
            ProjectionMode::ORTHOGRAPHIC => {

                cgmath::ortho(
                    self.get_left(),
                    self.get_right(),
                    self.get_bottom(),
                    self.get_top(),
                    self.znear,
                    self.zfar)
            }
        }
    }

    #[must_use]
    pub fn ndc_to_world_coordinates(&self, x: f32, y: f32, z: f32) ->
        cgmath::Point3<f32> {

        use cgmath::SquareMatrix;

        let Some(inv) = self.get_matrix().invert() else {
            return cgmath::Point3::<f32>{ x, y, z };
        };

        let ndc = cgmath::Vector4::<f32> { x, y, z, w: 1.0 };

        let mut world_pos = inv * ndc;
        world_pos /= world_pos.w;

        cgmath::Point3::<f32>{
            x: world_pos.x,
            y: world_pos.y,
            z: world_pos.z
        }
    }
}
