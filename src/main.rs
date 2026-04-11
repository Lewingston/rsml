
use rsml::drawable::drawable::Drawable;

use std::rc::Rc;

struct MyApp {

}

impl rsml::App for MyApp {

    fn start(&mut self, context: &mut rsml::AppContext) {

        println!("APP STARTED!");

        _ = context.create_window(MainWindow::new());
        _ = context.create_window(SecondaryWindow::new());
    }
}


struct MainScene {

    pub triangle: rsml::Shape,
    pub sprite:   rsml::Shape,

    pub camera_control: rsml::CameraController
}


struct MainWindow {

    scene: Option<MainScene>
}

impl MainWindow {

    fn new() -> Self {

        Self{
            scene: None
        }
    }
}

impl rsml::Window for MainWindow {

    fn start(&mut self, context: rsml::WindowContext) {

        println!("MainWindow start");

        let texture = Rc::new(match rsml::Texture::from_bytes(
            context.renderer,
            include_bytes!("./test_image.png"),
            Some("test texture")
        ) {
            Ok(texture) => texture,
            Err(_err)   => { return; }
        });

        self.scene = Some(MainScene {
            triangle:       rsml::Shape::create_triangle(context.renderer),
            sprite:         rsml::Shape::create_sprite(context.renderer, 2.0, 2.0, texture),
            camera_control: rsml::CameraController::new(context.camera.clone())
        });
    }

    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        let Some(scene) = &self.scene else { return; };

        scene.triangle.draw(render_target);
        scene.sprite.draw(render_target);
    }

    fn event(&mut self, event: winit::event::WindowEvent, context: rsml::WindowContext) {

        //println!("MainWindow event: {event:?}");

        if let winit::event::WindowEvent::KeyboardInput {
            event: winit::event::KeyEvent {
                physical_key: winit::keyboard::PhysicalKey::Code(code),
                state: key_state,
                ..
            },
            ..
        } = event {

            let Some(scene) = &mut self.scene else { return; };

            let camera_moved = scene.camera_control.keyboard_input(code, key_state.is_pressed());

            if camera_moved {
                scene.camera_control.update_camera(context.renderer.get_queue());
            }
        }
    }
}

struct SecondaryScene {

    pub square: rsml::Shape
}

struct SecondaryWindow {

    scene: Option<SecondaryScene>
}

impl SecondaryWindow {

    fn new() -> Self {

        Self{
            scene: None
        }
    }
}

impl rsml::Window for SecondaryWindow {

    fn start(&mut self, context: rsml::WindowContext) {

        println!("SecondaryWindow start");

        self.scene = Some(SecondaryScene {
            square: rsml::Shape::create_rectangle(context.renderer, 0.5, 0.5)
            //square: rsml::Shape::create_rectangle(context.renderer, 100.0, 100.0)
        });
    }

    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        let Some(scene) = &self.scene else { return; };

        scene.square.draw(render_target);
    }

    fn event(&mut self, event: winit::event::WindowEvent, _context: rsml::WindowContext) {

        println!("SecondaryWindow event: {event:?}");
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
