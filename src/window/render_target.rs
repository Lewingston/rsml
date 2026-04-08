
use crate::drawable::drawable::Drawable;


pub struct RenderTarget<'encoder> {

    render_pass: wgpu::RenderPass<'encoder>
}


impl<'a> RenderTarget<'a> {


    #[must_use]
    pub fn new(render_pass: wgpu::RenderPass<'a>) -> Self {

        Self { render_pass }
    }


    pub fn draw<T: Drawable>(&mut self, drawable: &T) {

        self.render_pass.set_pipeline(drawable.get_pipeline());

        let vertex_buffer = drawable.get_vertex_buffer();
        let index_buffer  = drawable.get_index_buffer();

        if let Some(buffer) = vertex_buffer {
            self.render_pass.set_vertex_buffer(0, buffer.slice(..));
        }

        if let Some(buffer) = index_buffer {
            self.render_pass.set_index_buffer(buffer.slice(..), wgpu::IndexFormat::Uint16);
        }

        if vertex_buffer.is_some() & index_buffer.is_none() {
            self.render_pass.draw(0..drawable.get_vertex_count(),  0..1);
        } else if index_buffer.is_some() {
            self.render_pass.draw_indexed(0..drawable.get_index_count(), 0, 0..1);
        }
    }
}
