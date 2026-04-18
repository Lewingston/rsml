
use rsml::drawable::drawable::Drawable;

use std::rc::Rc;
use std::cell::RefCell;


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

    pub camera_control: rsml::CameraController
}


impl MainScene {


    pub fn new(
        camera: &Rc<RefCell<rsml::Camera>>,
    ) -> Self {

            let camera_control = rsml::CameraController::new(camera.clone());

            Self {
                camera_control
            }
    }
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

        self.scene = Some(MainScene::new(context.camera));
    }


    fn draw(&mut self, _render_target: &mut rsml::RenderTarget) {

        //let Some(scene) = &self.scene else { return; };
    }


    fn event(&mut self, event: winit::event::WindowEvent, _context: rsml::WindowContext) {

        if let winit::event::WindowEvent::KeyboardInput {
            event: winit::event::KeyEvent {
                physical_key: winit::keyboard::PhysicalKey::Code(code),
                state: key_state,
                ..
            },
            ..
        } = event {

            let Some(scene) = &mut self.scene else { return; };

            scene.camera_control.keyboard_input(code, key_state.is_pressed());
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

    fn start(&mut self, _context: rsml::WindowContext) {

        println!("SecondaryWindow start");

        self.scene = Some(SecondaryScene {
            square: rsml::Shape::create_rectangle(0.5, 0.5)
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
