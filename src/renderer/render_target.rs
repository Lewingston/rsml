

pub struct RenderTarget<'encoder> {

    render_pass: wgpu::RenderPass<'encoder>
}


impl<'a> RenderTarget<'a> {


    #[must_use]
    pub fn new(render_pass: wgpu::RenderPass<'a>) -> Self {

        Self { render_pass }
    }


    #[must_use]
    pub fn get_render_pass<'b>(&'b mut self) -> &'b mut wgpu::RenderPass<'a> {

        &mut self.render_pass
    }
}
