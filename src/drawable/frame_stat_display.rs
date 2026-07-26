
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

#[cfg(target_arch = "wasm32")]
use web_time::{Instant, Duration};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Instant, Duration};


pub struct FrameStatDisplay {

    text:        Text,
    camera:      Rc<RefCell<Camera>>,
    last_update: Option<Instant>,
}


impl FrameStatDisplay {

    #[must_use]
    pub fn new(screen_width: u32, screen_height: u32) -> Self {

        let width  = screen_width  as f32;
        let height = screen_height as f32;

        let text = Text::new(
            "",
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

        let mut display = Self {
            text,
            camera,
            last_update: None
        };

        display.set_position(-width / 2.0, height / 2.0);

        display
    }


    pub fn draw(
        &mut self,
        render_target: &mut RenderTarget,
        frame_monitor: &FrameMonitor
    ) {

        match self.last_update {
            Some(time) => {
                if time.elapsed() >= Duration::from_millis(500) {
                    self.set_text(frame_monitor);
                }
            }
            None => self.set_text(frame_monitor)
        };


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

        self.last_update = Some(Instant::now());

        let fps = match monitor.get_fps() {
            Some(fps) => format!("{fps:.2}"),
            None => "-".to_string()
        };

        let format_time = |time: Option<Duration>|
            match time {
                Some(time) => format!("{:.1}ms", time.as_millis()),
                None => "-".to_string()
            };

        let acquire_surface = format_time(monitor.get_surface_acquisition_time());
        let discard_surface = format_time(monitor.get_surface_discard_time());
        let render_time     = format_time(monitor.get_render_time());
        let submit_time     = format_time(monitor.get_submit_time());

        let text = format!(r"FPS: {fps}
Acquire surface: {acquire_surface}
Discard surface: {discard_surface}
Render time: {render_time}
Submit frame time: {submit_time}
");

        self.text.set_text(&text);
    }
}
