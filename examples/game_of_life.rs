
use wgpu::util::DeviceExt;

use rsml::drawable::drawable::Color;
use rsml::drawable::drawable::Drawable;

use std::rc::Rc;
use std::cell::RefCell;


const CELL_SIZE: usize = 13;
const BLACK: rsml::Color = rsml::Color { r: 4, g: 4, b: 4, a: 255 };
const WHITE: rsml::Color = rsml::Color { r: 200, g: 200, b: 200, a: 255 };


struct MyApp {

}


impl rsml::App for MyApp {

    fn start(&mut self, context: &mut rsml::AppContext) {

        _ = context.create_window(MainWindow::new());
    }
}


struct Scene {

    game:          GameOfLife,
    frame_count:   rsml::Text,
    info_bar:      rsml::Text,
    window_width:  u32,
    window_height: u32,
    game_running:  bool,
}


impl Scene {

    fn new(width: u32, height: u32) -> Option<Self> {

        let window_width  = width;
        let window_height = height;

        let Ok(font) = rsml::Font::from_bytes(include_bytes!("./roboto.ttf")) else { return None; };
        let font = Rc::new(RefCell::new(font));

        let frame_count_layout = fontdue::layout::LayoutSettings {
            x: 0.0,
            y: 0.0,
            max_width: Some(250.0),
            max_height: Some(22.0),
            horizontal_align: fontdue::layout::HorizontalAlign::Right,
            vertical_align: fontdue::layout::VerticalAlign::Middle,
            ..fontdue::layout::LayoutSettings::default()
        };

        let mut frame_count = rsml::Text::new("-", font.clone(), 14.0, Some(frame_count_layout));
        frame_count.set_color(WHITE);
        Self::set_fps_pos(&mut frame_count, window_width, window_height);

        let info_bar_layout = fontdue::layout::LayoutSettings::default();
        let mut info_bar = rsml::Text::new(&Self::create_info_bar_text(), font, 14.0, Some(info_bar_layout));
        info_bar.set_color(WHITE);
        info_bar.get_transform().translate(cgmath::Vector3::<f32>{ x: 10.0, y: 20.0, z: 1.0 });

        Some(Self {
            game: Self::create_game(width, height),
            frame_count,
            info_bar,
            window_width,
            window_height,
            game_running: false
        })
    }


    fn create_game(width: u32, height: u32) -> GameOfLife {


        let h = height as f32;
        let bar_height = h.min(22.0);

        GameOfLife::new(width, height - bar_height as u32, 0.0, bar_height)
    }


    fn set_frame_rate(&mut self, frame_time: &std::time::Duration) {

        let frame_time: f32 = frame_time.as_micros() as f32;

        if frame_time > 0.0 {

            let frame_rate = 1_000_000.0 / frame_time;
            self.frame_count.set_text(&format!("FPS: {:.2}", frame_rate));

        } else {

            self.frame_count.set_text(&"0.0");
        }
    }


    fn create_info_bar_text() -> String {

        String::from("Next Step: [ s ]   Reset: [ x ]   Run/Pause [ SPACE ]")
    }


    fn draw(&mut self, render_target: &mut rsml::RenderTarget) {

        if self.game_running {
            self.game.make_step();
        }

        self.game.draw(render_target);
        self.frame_count.draw(render_target);
        self.info_bar.draw(render_target);
    }


    pub fn resize(&mut self, width: u32, height: u32) {

        self.window_width  = width;
        self.window_height = height;

        self.game_running = false;

        Self::set_fps_pos(&mut self.frame_count, width, height);

        self.game = Self::create_game(width, height);

    }


    fn set_fps_pos(frame_count: &mut rsml::Text, width: u32, _height: u32) {

        frame_count.get_transform().set_pos(
            cgmath::Point3::<f32>{
                x: width as f32 - 255.0,
                y: 20.0,
                z: 1.0
            }
        );
    }


    pub fn handle_mouse_event(&mut self, mouse_event: &MouseEvent) {

        self.game.handle_mouse_event(&MouseEvent {
            left_button_pressed:  mouse_event.left_button_pressed,
            right_button_pressed: mouse_event.right_button_pressed,
            pos_x:                mouse_event.pos_x,
            pos_y:                self.window_height as f32 - mouse_event.pos_y
        });
    }


    pub fn key_pressed(&mut self, key: winit::keyboard::KeyCode) {

        use winit::keyboard::KeyCode as KeyCode;

        match key {
            KeyCode::KeyS  => {
                self.game.make_step();
            }
            KeyCode::KeyX  => {
                self.game_running = false;
                self.game = Self::create_game(self.window_width, self.window_height);
            }
            KeyCode::Space => {
                self.game_running = !self.game_running;
            }
            _ => {}
        }
    }
}


struct GameOfLife {

    cells:   Vec<Vec<bool>>,
    mesh:    Mesh,
    width:   usize,
    height:  usize,
    pos_x:   f32,
    pos_y:   f32,
}


impl GameOfLife {

    fn new(width: u32, height: u32, pos_x: f32, pos_y: f32) -> Self {

        let cells_x: usize = width  as usize / CELL_SIZE;
        let cells_y: usize = height as usize / CELL_SIZE;

        println!("Width: {cells_x} - Height: {cells_y} - Total: {}", cells_x * cells_y);

        let offset_x = (width - cells_x as u32 * CELL_SIZE as u32) as f32 / 2.0;
        let offset_y = (height - cells_y as u32 * CELL_SIZE as u32) as f32 / 2.0;

        let pos_x = pos_x + offset_x;
        let pos_y = pos_y + offset_y;

        let mut mesh = Mesh::new(cells_x, cells_y, CELL_SIZE as f32, CELL_SIZE as f32);
        mesh.set_offset(pos_x, pos_y);

        Self {
            cells:  vec![vec![false; cells_x]; cells_y],
            mesh,
            width:  cells_x,
            height: cells_y,
            pos_x,
            pos_y,
        }
    }


    fn draw(&self, render_target: &mut rsml::RenderTarget) {

        self.mesh.draw(render_target);
    }


    fn handle_mouse_event(&mut self, mouse_event: &MouseEvent) {

        let width  = (self.width  * CELL_SIZE) as f32;
        let height = (self.height * CELL_SIZE) as f32;

        if mouse_event.pos_x < self.pos_x ||
           mouse_event.pos_x > self.pos_x + width ||
           mouse_event.pos_y < self.pos_y ||
           mouse_event.pos_y > self.pos_y + height {

               return;
        }

        let x = (mouse_event.pos_x - self.pos_x) as usize / CELL_SIZE;
        let y = (mouse_event.pos_y - self.pos_y) as usize / CELL_SIZE;

        if x < self.width && y < self.height {

            if mouse_event.left_button_pressed && !mouse_event.right_button_pressed {

                self.cells[y][x] = true;
                self.mesh.set_cell(x, y, true);
                self.mesh.update();

            } else if mouse_event.right_button_pressed && !mouse_event.left_button_pressed {

                self.cells[y][x] = false;
                self.mesh.set_cell(x, y, false);
                self.mesh.update();
            }
        }
    }


    fn make_step(&mut self) {

        let mut new_cells: Vec<Vec<bool>> = vec![vec![false; self.width]; self.height];

        for y in 0..self.height {
            for x in 0..self.width {

                let count = self.count_life_neighbours(x, y);

                if self.is_alive(x as i32, y as i32) &&
                   count < 2 || count > 3 {

                    new_cells[y][x] = false;
                    self.mesh.set_cell(x, y, false);

                } else if !self.is_alive(x as i32, y as i32) && count == 3 {

                    new_cells[y][x] = true;
                    self.mesh.set_cell(x, y, true);

                } else {

                    new_cells[y][x] = self.cells[y][x];
                }
            }
        }

        self.cells = new_cells;

        self.mesh.update();
    }


    fn count_life_neighbours(&self, x: usize, y: usize) -> u8 {

        let mut count = 0;

        let x = x as i32;
        let y = y as i32;

        for iy in -1..2 {
            for ix in -1..2 {
                if ix != 0 || iy != 0 {
                    count += if self.is_alive(x + ix, y + iy) { 1 } else { 0 };
                }
            }
        }

        count
    }


    fn is_alive(&self, x: i32, y: i32) -> bool {

        if x >= 0 && x < self.width as i32 {
            if y >= 0 && y < self.height as i32 {

                return self.cells[y as usize][x as usize];
            }
        }

        false
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


    pub fn set_offset(&mut self, x: f32, y: f32) {

        self.transform.set_pos(cgmath::Point3 { x, y, z: 0.0 });
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
                usage:    wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST
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
            color:       BLACK
        }
    }


    pub fn draw(&self, render_target: &mut rsml::RenderTarget)
    {

        let camera = render_target.get_camera();

        let pass: &mut wgpu::RenderPass = render_target.get_render_pass();

        pass.set_pipeline(rsml::Renderer::get().get_default_color_render_pipeline().as_ref());

        pass.set_bind_group(0, self.transform.get_bind_group(), &[]);

        pass.set_bind_group(1, camera.borrow().get_bind_group(), &[]);

        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint32);

        let index_count = (self.width * self.height * 6) as u32;

        pass.draw_indexed(0..index_count, 0, 0..1);
    }


    fn set_cell(&mut self, x: usize, y: usize, state: bool) {

        if x >= self.width || y >= self.height {
            return;
        }

        let index = (y * self.width + x) * 4;

        self.vertices[index + 0].color = if state { WHITE } else { BLACK };
        self.vertices[index + 1].color = if state { WHITE } else { BLACK };
        self.vertices[index + 2].color = if state { WHITE } else { BLACK };
        self.vertices[index + 3].color = if state { WHITE } else { BLACK };
    }


    fn update(&self) {

        rsml::Renderer::get().get_queue().write_buffer(
            &self.vertex_buffer,
            0,
            bytemuck::cast_slice(&self.vertices)
        );
    }
}


#[derive(Debug, Clone)]
struct MouseEvent {

    left_button_pressed:  bool,
    right_button_pressed: bool,
    pos_x: f32,
    pos_y: f32
}


struct MainWindow {

    scene: Option<Scene>,
    mouse_state: MouseEvent
}


impl MainWindow {

    fn new() -> Self {

        Self {
            scene: None,
            mouse_state: MouseEvent {
                left_button_pressed:  false,
                right_button_pressed: false,
                pos_x: 0.0,
                pos_y: 0.0
            }
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
            winit::event::WindowEvent::MouseInput{
                state,
                button,
                ..
            } => {

                if button == winit::event::MouseButton::Left {

                    if state == winit::event::ElementState::Pressed {
                        self.mouse_state.left_button_pressed = true;
                    } else {
                        self.mouse_state.left_button_pressed = false;
                    }

                    self.handle_mouse_event(self.mouse_state.clone());

                } else if button == winit::event::MouseButton::Right {

                    if state == winit::event::ElementState::Pressed {
                        self.mouse_state.right_button_pressed = true;
                    } else {
                        self.mouse_state.right_button_pressed = false;
                    }

                    self.handle_mouse_event(self.mouse_state.clone());
                }

            }
            winit::event::WindowEvent::CursorMoved { position, .. } => {

                self.mouse_state.pos_x = position.x as f32;
                self.mouse_state.pos_y = position.y as f32;

                self.handle_mouse_event(self.mouse_state.clone());
            }
            winit::event::WindowEvent::KeyboardInput {
                event: winit::event::KeyEvent {
                    physical_key: winit::keyboard::PhysicalKey::Code(code),
                    state:        key_state,
                    ..
                },
                ..
            } => {

                if key_state.is_pressed() {
                    scene.key_pressed(code);
                }
            }
            _ => {}
        }
    }
}


impl MainWindow {

    fn handle_mouse_event(&mut self, mouse_event: MouseEvent) {

        let Some(scene) = &mut self.scene else { return; };

        scene.handle_mouse_event(&mouse_event);
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    rsml::start(MyApp{})?;

    Ok(())
}
