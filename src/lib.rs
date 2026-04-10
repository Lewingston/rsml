
mod app;

pub use app::App;

pub use app::start;

pub use app::AppContext;

mod window;

pub use window::Window;
pub use window::RenderTarget;
pub use window::WindowContext;

pub mod drawable;

pub use drawable::vertex_buffer::VertexBuffer;
pub use drawable::drawable::TextureVertex;
pub use drawable::drawable::ColorVertex;
pub use drawable::drawable::Shape;
pub use drawable::drawable::Sprite;
pub use drawable::texture::Texture;

mod error;
