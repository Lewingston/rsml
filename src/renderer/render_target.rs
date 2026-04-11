
use crate::renderer::camera::Camera;

use std::rc::Rc;
use std::cell::RefCell;


pub struct RenderTarget<'encoder> {

    render_pass: wgpu::RenderPass<'encoder>,
    camera:      Rc<RefCell<Camera>>
}


impl<'a> RenderTarget<'a> {


    #[must_use]
    pub fn new(
        render_pass: wgpu::RenderPass<'a>,
        camera:      Rc<RefCell<Camera>>
    ) -> Self {

        Self { render_pass, camera }
    }


    #[must_use]
    pub fn get_render_pass<'b>(&'b mut self) -> &'b mut wgpu::RenderPass<'a> {

        &mut self.render_pass
    }


    #[must_use]
    pub fn get_camera(&self) -> Rc<RefCell<Camera>> {

        self.camera.clone()
    }
}
