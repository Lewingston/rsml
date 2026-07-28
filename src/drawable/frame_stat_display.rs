
use crate::renderer::camera::Camera;
use crate::renderer::camera::CameraParameters;
use crate::renderer::camera::ProjectionMode;
use crate::renderer::render_target::RenderTarget;
use crate::drawable::font;
use crate::drawable::text::Text;
use crate::drawable::drawable::Drawable;
use crate::drawable::drawable::Color;
use crate::window::frame_monitor::FrameMonitor;
use crate::renderer::renderer::Renderer;

use std::rc::Rc;
use std::cell::RefCell;

#[cfg(target_arch = "wasm32")]
use web_time::{Instant, Duration};

#[cfg(not(target_arch = "wasm32"))]
use std::time::{Instant, Duration};


#[derive(PartialEq)]
pub enum DisplayMode {
    None,
    Fps,
    FrameStatistics,
    HardwareInfo
}


pub struct FrameStatDisplay {

    text:         Text,
    camera:       Rc<RefCell<Camera>>,
    last_update:  Option<Instant>,
    info:         wgpu::AdapterInfo,
    display_mode: DisplayMode
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

        let mut cam_params = CameraParameters{
            width,
            height,
            ..Default::default()
        };
        cam_params.projection = ProjectionMode::ORTHOGRAPHIC;

        let camera = Camera::new(cam_params);

        let camera = Rc::new(RefCell::new(camera));

        let mut display = Self {
            text,
            camera,
            last_update:  None,
            info:         Renderer::get().get_adapter().get_info(),
            display_mode: DisplayMode::Fps
        };

        display.set_position(-width / 2.0, height / 2.0);

        display
    }


    pub fn set_display_mode(&mut self, mode: DisplayMode) {

        self.display_mode = mode;
        self.last_update = None;
    }


    pub fn toggle_display_mode(&mut self) {

        match self.display_mode {
            DisplayMode::None => self.display_mode = DisplayMode::Fps,
            DisplayMode::Fps => self.display_mode = DisplayMode::FrameStatistics,
            DisplayMode::FrameStatistics => self.display_mode = DisplayMode::HardwareInfo,
            DisplayMode::HardwareInfo => self.display_mode = DisplayMode::None
        }

        self.last_update = None;
    }


    pub fn draw(
        &mut self,
        render_target: &mut RenderTarget,
        frame_monitor: &FrameMonitor
    ) {

        if self.display_mode == DisplayMode::None {
            return;
        }

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

        if monitor.get_fps().is_some() {
            self.last_update = Some(Instant::now());
        }

        let text = match self.display_mode {
            DisplayMode::None => "".to_string(),
            DisplayMode::Fps => Self::get_fps_string(monitor),
            DisplayMode::FrameStatistics => {
                let fps   = Self::get_fps_string(monitor);
                let frame = Self::get_frame_info_string(monitor);
                format!("{fps}\n{frame}")
            }
            DisplayMode::HardwareInfo => {
                let fps   = Self::get_fps_string(monitor);
                let frame = Self::get_frame_info_string(monitor);
                let info  = self.adapter_info_to_string();
                format!("{fps}\n{frame}\n\n{info}")
            }
        };

        self.text.set_text(&text);
    }


    #[must_use]
    fn get_fps_string(monitor: &FrameMonitor) -> String {

        match monitor.get_fps() {
            Some(fps) => format!("FPS: {fps:.2}"),
            None => "FPS: -".to_string()
        }
    }


    #[must_use]
    fn get_frame_info_string(monitor: &FrameMonitor) -> String {

        let format_time = |time: Option<Duration>|
            match time {
                Some(time) => format!("{:.1}ms", time.as_millis()),
                None => "-".to_string()
            };

        let acquire_surface = format_time(monitor.get_surface_acquisition_time());
        let discard_surface = format_time(monitor.get_surface_discard_time());
        let render_time     = format_time(monitor.get_render_time());
        let submit_time     = format_time(monitor.get_submit_time());

        format!(r"Acquire surface: {acquire_surface}
Discard surface: {discard_surface}
Render time: {render_time}
Submit frame time: {submit_time}")
    }


    #[must_use]
    fn adapter_info_to_string(&self) -> String {

        let info = &self.info;

        let name        = &info.name;
        let vendor      = info.vendor;
        let device      = info.device;
        let device_type = info.device_type;
        let driver      = &info.driver;
        let driver_info = &info.driver_info;
        let backend     = info.backend;

        format!(r"Name: {name}
Vendor: {vendor}
Device: {device}
Device type: {device_type:?}
Driver: {driver}
Driver info: {driver_info}
Backend: {backend}")
    }
}
