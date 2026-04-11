
use crate::renderer::camera::Camera;

use std::rc::Rc;
use std::cell::RefCell;


pub struct CameraController {

    camera:         Rc<RefCell<Camera>>,
    zoom_speed:     f32,
    rotation_speed: f32
}


#[derive(Debug)]
struct SphericalPos {

    pub r:        f32,
    pub azimuth:  f32,
    pub altitude: f32,
}


impl CameraController {


    pub fn new(camera: Rc<RefCell<Camera>>) -> Self {

        Self {
            camera,
            zoom_speed:     0.15,
            rotation_speed: 12.0 / 180.0
        }
    }


    pub fn keyboard_input(&mut self, key_code: winit::keyboard::KeyCode, is_pressed: bool) -> bool {

        use winit::keyboard::KeyCode as KeyCode;
        use cgmath::EuclideanSpace;

        if !is_pressed {
            return false;
        }

        const POLAR_DEAD_SCONE: f32 = 0.01;

        let mut param = self.camera.borrow().get_parameters().clone();
        let mut pos = SphericalPos::from_pos(param.pos - param.target);

        match key_code {
            KeyCode::KeyW | KeyCode::ArrowUp => {

                pos.altitude = f32::max(pos.altitude - self.rotation_speed, POLAR_DEAD_SCONE);
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {

                pos.altitude = f32::min(pos.altitude + self.rotation_speed, std::f32::consts::PI - POLAR_DEAD_SCONE);
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {

                pos.azimuth -= self.rotation_speed;
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {

                pos.azimuth += self.rotation_speed;
            }
            KeyCode::BracketRight | KeyCode::ShiftLeft => {

                pos.r = f32::max(0.01, pos.r - self.zoom_speed);
            }
            KeyCode::Slash | KeyCode::ControlLeft => {

                pos.r += self.zoom_speed;
            }
            _ => { return false; }
        }

        param.pos = cgmath::Point3::from_vec(pos.to_pos() + param.target.to_vec());
        self.camera.borrow_mut().set_parameters(param);
        true
    }


    pub fn update_camera(&self, queue: &wgpu::Queue) {

        self.camera.borrow().update(queue);
    }
}


impl SphericalPos {


    pub fn from_pos(pos: cgmath::Vector3<f32>) -> Self {

        use cgmath::InnerSpace;

        let r = pos.magnitude();
        let azimuth = pos.x.atan2(pos.z);
        let altitude = (pos.z.hypot(pos.x)).atan2(pos.y);

        Self {
            r,
            azimuth,
            altitude
        }
    }


    pub fn to_pos(&self) -> cgmath::Vector3<f32> {

        let x = self.r * self.azimuth.sin() * self.altitude.sin();
        let y = self.r * self.altitude.cos();
        let z = self.r * self.altitude.sin() * self.azimuth.cos();

        cgmath::Vector3::new(x, y, z)
    }
}
