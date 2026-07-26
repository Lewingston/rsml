
use crate::renderer::camera::Camera;
use crate::renderer::camera::CameraParameters;
use crate::renderer::camera::ProjectionMode;
use crate::renderer::render_target::RenderTarget;
use crate::drawable::font;
use crate::drawable::text::Text;
use crate::drawable::drawable::Drawable;
use crate::drawable::drawable::Color;
use crate::window::frame_monitor::FrameMonitor;

use std::rc::Rc;
use std::cell::RefCell;


pub struct FrameStatDisplay {

    text:   Text,
    camera: Rc<RefCell<Camera>>
}


impl FrameStatDisplay {

    #[must_use]
    pub fn new(screen_width: u32, screen_height: u32) -> Self {

        let width  = screen_width  as f32;
        let height = screen_height as f32;

        let text = Text::new(
            "-",
            Rc::new(RefCell::new(font::get_embedded_font())),
            14.0,
            None);

        let mut cam_params = CameraParameters::default(
            width,
            height
        );
        cam_params.projection = ProjectionMode::ORTHOGRAPHIC;

        let camera = Camera::new(cam_params);

        let camera = Rc::new(RefCell::new(camera));

        let mut display = Self { text, camera };

        display.set_position(-width / 2.0, height / 2.0);

        display
    }


    pub fn draw(
        &mut self,
        render_target: &mut RenderTarget,
        frame_monitor: &FrameMonitor
    ) {

        self.set_text(frame_monitor);

        let old_cam = render_target.get_camera().clone();

        render_target.set_camera(self.camera.clone());

        self.text.draw(render_target);

        render_target.set_camera(old_cam);
    }


    pub fn screen_resized(&mut self, width: u32, height: u32) {

        let width  = width as f32;
        let height = height as f32;

        self.set_position(-width / 2.0, height / 2.0);

        let mut camera = self.camera.borrow_mut();

        let mut params = camera.get_parameters().clone();

        params.width  = width;
        params.height = height;

        camera.set_parameters(params);

    }


    pub fn set_front_color(&mut self, color: Color) {

        self.text.set_color(color);
    }


    fn set_position(&mut self, x: f32, y: f32) {

        let x = x + 4.0;

        self.text.get_transform().set_pos(
            cgmath::Point3::<f32>{x, y, z: 0.0}
        );
    }


    fn set_text(&mut self, monitor: &FrameMonitor) {

        let fps = monitor.get_fps();

        let text = format!("FPS: {:.2}", fps);

        self.text.set_text(&text);
    }
}
