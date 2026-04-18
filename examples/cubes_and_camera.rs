
use rsml::drawable::drawable::Drawable;

use std::rc::Rc;
use std::cell::RefCell;


struct MainScene {

    pub center_cube: Cube,
    pub left_cube:   Cube,
    pub right_cube:  Cube,
    pub back_cube:   Cube,
    pub top_cube:    Cube,
    pub bottom_cube: Cube,
    pub front_cube:  Cube,

    pub camera_control: rsml::CameraController
}


impl MainScene {


    pub fn new(
        renderer: &rsml::Renderer,
        camera: &Rc<RefCell<rsml::Camera>>,
    ) -> Self {

            let mut center_cube = Cube::new(renderer);

            match rsml::Texture::from_bytes(
                renderer,
                include_bytes!("./test_image.png"),
                Some("test texture")
            ) {
                Ok(texture) => {
                    let t = Rc::new(texture);
                    center_cube.faces[0].set_texture(t.clone(), renderer);
                    center_cube.faces[1].set_texture(t.clone(), renderer);
                    center_cube.faces[2].set_texture(t.clone(), renderer);
                    center_cube.faces[3].set_texture(t.clone(), renderer);
                    center_cube.faces[4].set_texture(t.clone(), renderer);
                    center_cube.faces[5].set_texture(t.clone(), renderer);
                },
                Err(_err)   => { }
            };

            let mut left_cube = Cube::new(renderer);
            left_cube.move_x(-2.0, renderer.get_queue());

            let mut right_cube = Cube::new(renderer);
            right_cube.move_x(2.0, renderer.get_queue());

            let mut back_cube = Cube::new(renderer);
            back_cube.move_z(-2.0, renderer.get_queue());

            let mut top_cube = Cube::new(renderer);
            top_cube.move_y(2.0, renderer.get_queue());

            let mut bottom_cube = Cube::new(renderer);
            bottom_cube.move_y(-2.0, renderer.get_queue());

            let mut front_cube = Cube::new(renderer);
            front_cube.move_z(2.0, renderer.get_queue());

            let camera_control = rsml::CameraController::new(camera.clone());

            Self {
                center_cube,
                left_cube,
                right_cube,
                back_cube,
                top_cube,
                bottom_cube,
                front_cube,
                camera_control
            }
    }


    /*
    pub fn handle_key(
        &mut self,
        key_code: winit::keyboard::KeyCode,
        is_pressed: bool,
        queue: &wgpu::Queue)
    {

        use winit::keyboard::KeyCode as KeyCode;

        if !is_pressed {
            return;
        }

        let move_speed = 0.075;
        let rotation_speed = 12.0;

        match key_code {
            KeyCode::KeyW | KeyCode::ArrowUp => {

                self.cube.move_y(move_speed, queue);
            }
            KeyCode::KeyS | KeyCode::ArrowDown => {

                self.cube.move_y(-move_speed, queue);
            }
            KeyCode::KeyA | KeyCode::ArrowLeft => {

                self.cube.move_x(-move_speed, queue);
            }
            KeyCode::KeyD | KeyCode::ArrowRight => {

                self.cube.move_x(move_speed, queue);
            }
            KeyCode::KeyQ => {

                self.cube.rotate(cgmath::Deg(rotation_speed), queue);
            }
            KeyCode::KeyE => {

                self.cube.rotate(cgmath::Deg(-rotation_speed), queue);
            }
            _ => {}
        }
    }
    */
}


struct Cube {

    pub faces: [rsml::Shape; 6]
}


impl Cube {

    pub fn new(renderer: &rsml::Renderer) -> Self {

        let mut front = rsml::Shape::create_rectangle(renderer, 1.0, 1.0);
        front.get_transform().translate(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: 0.5 });
        front.get_transform().move_origin(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: -0.5 });
        front.get_transform().update(renderer.get_queue());
        front.set_color(rsml::Color::random(), renderer.get_device());

        let mut top = rsml::Shape::create_rectangle(renderer, 1.0, 1.0);
        top.get_transform().rotate_x(cgmath::Rad::<f32>::from(cgmath::Deg(-90.0)));
        top.get_transform().translate(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: 0.5});
        top.get_transform().move_origin(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: -0.5 });
        top.get_transform().update(renderer.get_queue());
        top.set_color(rsml::Color::random(), renderer.get_device());

        let mut left = rsml::Shape::create_rectangle(renderer, 1.0, 1.0);
        left.set_color(rsml::Color::random(), renderer.get_device());
        left.get_transform().rotate_y(cgmath::Rad::<f32>::from(cgmath::Deg(-90.0)));
        left.get_transform().translate(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: 0.5 });
        left.get_transform().move_origin(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: -0.5 });
        left.get_transform().update(renderer.get_queue());

        let mut right = rsml::Shape::create_rectangle(renderer, 1.0, 1.0);
        right.set_color(rsml::Color::random(), renderer.get_device());
        right.get_transform().rotate_y(cgmath::Rad::<f32>::from(cgmath::Deg(90.0)));
        right.get_transform().translate(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: 0.5 });
        right.get_transform().move_origin(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: -0.5 });
        right.get_transform().update(renderer.get_queue());

        let mut back = rsml::Shape::create_rectangle(renderer, 1.0, 1.0);
        back.set_color(rsml::Color::random(), renderer.get_device());
        back.get_transform().rotate_y(cgmath::Rad::<f32>::from(cgmath::Deg(180.0)));
        back.get_transform().translate(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: 0.5 });
        back.get_transform().move_origin(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: -0.5 });
        back.get_transform().update(renderer.get_queue());

        let mut bottom = rsml::Shape::create_rectangle(renderer, 1.0, 1.0);
        bottom.set_color(rsml::Color::random(), renderer.get_device());
        bottom.get_transform().rotate_x(cgmath::Rad::<f32>::from(cgmath::Deg(90.0)));
        bottom.get_transform().translate(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: 0.5 });
        bottom.get_transform().move_origin(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: -0.5 });
        bottom.get_transform().update(renderer.get_queue());

        Self {
            faces: [
                front,
                top,
                left,
                right,
                back,
                bottom
            ]
        }
    }


    pub fn draw(&self, render_target: &mut rsml::RenderTarget) {

        for face in &self.faces {

            face.draw(render_target);
        }
    }

    pub fn move_x(&mut self, x: f32, queue: &wgpu::Queue) {

        for face in &mut self.faces {

            face.get_transform().translate(cgmath::Vector3::<f32>{ x: x, y: 0.0, z: 0.0 });
            face.get_transform().update(queue);
        }
    }


    pub fn move_y(&mut self, y: f32, queue: &wgpu::Queue) {

        for face in &mut self.faces {

            face.get_transform().translate(cgmath::Vector3::<f32>{ x: 0.0, y: y, z: 0.0 });
            face.get_transform().update(queue);
        }
    }


    pub fn move_z(&mut self, z: f32, queue: &wgpu::Queue) {

        for face in &mut self.faces {

            face.get_transform().translate(cgmath::Vector3::<f32>{ x: 0.0, y: 0.0, z: z });
            face.get_transform().update(queue);
        }
    }


    /*
    pub fn rotate(&mut self, deg: cgmath::Deg<f32>, queue: &wgpu::Queue) {

        // front
        self.faces[0].get_transform().rotate_y(cgmath::Rad::<f32>::from(deg));
        // top
        self.faces[1].get_transform().rotate_z(cgmath::Rad::<f32>::from(deg));
        // left
        self.faces[2].get_transform().rotate_y(cgmath::Rad::<f32>::from(deg));
        // right
        self.faces[3].get_transform().rotate_y(cgmath::Rad::<f32>::from(deg));
        // back
        self.faces[4].get_transform().rotate_y(cgmath::Rad::<f32>::from(deg));
        // bottom
        self.faces[5].get_transform().rotate_z(cgmath::Rad::<f32>::from(-deg));

        for face in &mut self.faces {

            face.get_transform().update(queue);
        }
    }
    */
}


struct MainWindow {

    scene: Option<MainScene>
}


impl MainWindow {

    fn new() -> Self {
        Self {
            scene: None
        }
    }
}


impl rsml::Window for MainWindow {


    fn start(&mut self, context: rsml::WindowContext) {

        self.scene = Some(MainScene::new(context.renderer, context.camera));
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        let Some(scene) = &self.scene else { return; };

        scene.center_cube.draw(render_target);
        scene.left_cube.draw(render_target);
        scene.right_cube.draw(render_target);
        scene.back_cube.draw(render_target);
        scene.top_cube.draw(render_target);
        scene.bottom_cube.draw(render_target);
        scene.front_cube.draw(render_target);
    }


    fn event(&mut self, event: winit::event::WindowEvent, context: rsml::WindowContext) {

        if let winit::event::WindowEvent::KeyboardInput {
            event: winit::event::KeyEvent {
                physical_key: winit::keyboard::PhysicalKey::Code(code),
                state: key_state,
                ..
            },
            ..
        } = event {

            let Some(scene) = &mut self.scene else { return; };

            //scene.handle_key(code, key_state.is_pressed(), context.renderer.get_queue());

            let camera_moved = scene.camera_control.keyboard_input(code, key_state.is_pressed());

            if camera_moved {
                scene.camera_control.update_camera(context.renderer.get_queue());
            }
        }
    }
}


struct App {

}


impl rsml::App for App {

    fn start(&mut self, context: &mut rsml::AppContext) {

        _ = context.create_window(MainWindow::new());
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(App{})?;

    Ok(())
}
