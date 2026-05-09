
use wgpu::util::DeviceExt;

use rsml::drawable::drawable::Color;
use rsml::drawable::drawable::Drawable;

use std::rc::Rc;
use std::cell::RefCell;


const CELL_SIZE: usize = 16;


struct MyApp {

}


impl rsml::App for MyApp {

    fn start(&mut self, context: &mut rsml::AppContext) {

        _ = context.create_window(MainWindow::new());
    }
}


struct Scene {

    game:        GameOfLife,
    frame_count: rsml::Text
}


impl Scene {

    fn new(width: u32, height: u32) -> Option<Self> {

        let window_height = height;

        let Ok(font) = rsml::Font::from_bytes(include_bytes!("./roboto.ttf")) else { return None; };
        let font = Rc::new(RefCell::new(font));

        let layout = fontdue::layout::LayoutSettings::default();

        let mut frame_count = rsml::Text::new("-", font, 16.0, Some(layout));
        frame_count.set_color(Color { r: 240, g: 240, b: 240, a: 255 });
        frame_count.get_transform().translate(cgmath::Vector3::<f32>{x: 0.0, y: window_height as f32, z: 1.0});

        Some(Self {
            game: GameOfLife::new(width, height),
            frame_count
        })
    }


    fn set_frame_rate(&mut self, frame_time: &std::time::Duration) {

        let frame_time: f32 = frame_time.as_micros() as f32;

        if frame_time > 0.0 {

            let frame_rate = 1_000_000.0 / frame_time;
            self.frame_count.set_text(&format!("{:.2}", frame_rate));

        } else {

            self.frame_count.set_text(&"0.0");
        }
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        self.game.draw(render_target);
        self.frame_count.draw(render_target);
    }


    pub fn resize(&mut self, width: u32, height: u32) {

        self.frame_count.get_transform().set_pos(cgmath::Point3::<f32>{x: 0.0, y: height as f32, z: 1.0});

        self.game = GameOfLife::new(width, height);
    }
}


struct GameOfLife {

    cells:       Vec<Vec<bool>>,
    mesh:        Mesh,
    width:       usize,
    height:      usize
}


impl GameOfLife {

    fn new(width: u32, height: u32) -> Self {

        let width:  usize = width  as usize / CELL_SIZE;
        let height: usize = height as usize / CELL_SIZE;

        println!("Width: {width} - Height: {height} - Total: {}", width * height);

        Self {
            cells: vec![vec![false; width]; height],
            mesh: Mesh::new(width, height, CELL_SIZE as f32, CELL_SIZE as f32),
            width,
            height
        }
    }


    fn draw(&self, render_target: &mut rsml::RenderTarget) {

        self.mesh.draw(render_target);
    }
}


struct Mesh {

    width: usize,
    height: usize,

    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer,

    transform: rsml::Transform,

    vertices: Vec<rsml::Vertex>
}


impl Mesh {

    pub fn new(width: usize, height: usize, cell_width: f32, cell_height: f32) -> Self {

        let vertices = Self::create_vertices(width, height, cell_width, cell_height);

        Self {
            transform: rsml::Transform::new(rsml::Renderer::get().get_device()),
            width,
            height,
            vertex_buffer: Self::create_vertex_buffer(&vertices),
            index_buffer:  Self::create_index_buffer(width, height),
            vertices
        }
    }


    fn create_vertices(width: usize, height: usize, cell_width: f32, cell_height: f32) -> Vec<rsml::Vertex> {

        let mut vertices: Vec<rsml::Vertex> = Vec::with_capacity(width * height * 4);

        let mut pos_x: f32 = 0.0;
        let mut pos_y: f32 = 0.0;

        let offset: f32 = 1.0;

        for _y in 0..height {
            for _x in 0..width {

                vertices.push(Self::create_vertex(pos_x, pos_y));
                vertices.push(Self::create_vertex(pos_x + cell_width - offset, pos_y));
                vertices.push(Self::create_vertex(pos_x, pos_y + cell_height - offset));
                vertices.push(Self::create_vertex(pos_x + cell_width - offset, pos_y + cell_height - offset));

                pos_x += cell_width;
            }

            pos_y += cell_height;
            pos_x = 0.0;
        }

        vertices
    }


    fn create_vertex_buffer(vertices: &[rsml::Vertex]) -> wgpu::Buffer {

        rsml::Renderer::get().get_device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("gol_mesh_vertex_buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage:    wgpu::BufferUsages::VERTEX
            }
        )
    }


    fn create_index_buffer(width: usize, height: usize) -> wgpu::Buffer {

        let mut indices: Vec<u32> = Vec::with_capacity(width * height * 6);

        let cell_count: u32 = (width * height) as u32;

        for ii in 0..cell_count {

            indices.push(ii * 4 + 0);
            indices.push(ii * 4 + 1);
            indices.push(ii * 4 + 2);
            indices.push(ii * 4 + 2);
            indices.push(ii * 4 + 1);
            indices.push(ii * 4 + 3);
        }

        rsml::Renderer::get().get_device().create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label:    Some("gol_mesh_index_buffer"),
                contents: bytemuck::cast_slice(&indices),
                usage:    wgpu::BufferUsages::INDEX
            }
        )
    }


    fn create_vertex(x: f32, y: f32) -> rsml::Vertex {

        rsml::Vertex {
            position:    [x, y, 0.0],
            texture_pos: [0.0, 0.0],
            color:       Color { r: 4, g: 4, b: 4, a: 255 }
        }
    }


    pub fn draw(&self, render_target: &mut rsml::RenderTarget)
    {

        let camera = render_target.get_camera();

        let pass: &mut wgpu::RenderPass = render_target.get_render_pass();

        pass.set_pipeline(rsml::Renderer::get().get_default_color_render_pipeline().as_ref());

        // TODO: Bind transform
        pass.set_bind_group(0, self.transform.get_bind_group(), &[]);

        pass.set_bind_group(1, camera.borrow().get_bind_group(), &[]);

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        let index_count = (self.width * self.height * 6) as u32;

        pass.draw_indexed(0..index_count, 0, 0..1);
    }
}


struct MainWindow {

    scene: Option<Scene>
}


impl MainWindow {

    fn new() -> Self {

        Self {
            scene: None
        }
    }


    fn position_camera(
        camera: &mut rsml::renderer::camera::Camera,
        width: u32,
        height: u32
    ) {

        let width:  f32 = width  as f32;
        let height: f32 = height as f32;

        let mut cam_params = camera.get_parameters().clone();
        cam_params.projection = rsml::renderer::camera::ProjectionMode::ORTHOGRAPHIC;
        cam_params.pos    = cgmath::Point3::<f32> { x: width / 2.0, y: height / 2.0, z: 10.0 };
        cam_params.target = cgmath::Point3::<f32> { x: width / 2.0, y: height / 2.0, z: 0.0 };
        camera.set_parameters(cam_params);
    }
}


impl rsml::Window for MainWindow {

    fn start(&mut self, context: rsml::WindowContext) {

        let mut camera = context.camera.borrow_mut();

        MainWindow::position_camera(&mut camera, context.get_width(), context.get_height());

        context.window_config.background_color = Color { r: 24, g: 26, b: 27, a: 255 };
        context.window_config.adjust_camera_on_resize = true;

        self.scene = Scene::new(context.get_width(), context.get_height());
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget, frame_monitor: &rsml::FrameMonitor) {

        let Some(scene) = &mut self.scene else { return; };

        scene.set_frame_rate(&frame_monitor.get_frame_time());

        scene.draw(render_target);
    }


    fn event(&mut self, event: winit::event::WindowEvent, context: rsml::WindowContext) {

        let Some(scene) = &mut self.scene else { return; };

        match event {
            winit::event::WindowEvent::Resized(size) => {

                MainWindow::position_camera(&mut context.camera.borrow_mut(), size.width, size.height);
                scene.resize(size.width, size.height);
            }
            _ => {}
        }
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
