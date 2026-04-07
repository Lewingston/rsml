
use crate::drawable::vertex_buffer::VertexBuffer;


pub struct RenderTarget<'encoder> {

    render_pass: wgpu::RenderPass<'encoder>
}


impl<'a> RenderTarget<'a> {


    #[must_use]
    pub fn new(render_pass: wgpu::RenderPass<'a>) -> Self {

        Self { render_pass }
    }


    pub fn draw(&mut self, vertex_buffer: &VertexBuffer, render_pipeline: &wgpu::RenderPipeline) {

        self.render_pass.set_pipeline(render_pipeline);
        self.render_pass.set_vertex_buffer(0, vertex_buffer.get_buffer().slice(..));
        self.render_pass.draw(0..3, 0..1);
    }
}
