
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {

    #[error("Failed to init console logger: {0}")]
    FailedToInitConsoleLogger(String),

    #[error("Failed to create event loop: {0}")]
    FailedToCreateEventLoop(String),

    #[error("Failed to start app: {0}")]
    FailedToStartApp(String),

    #[error("Unable to create renderer: {0}")]
    FailedToCreateRenderer(String),

    #[error("Failed to create window: {0}")]
    FailedToCreateWindow(String),

    #[error("Failed to acquire HTML element {0}")]
    FailedToAcquireHtmlElement(String),

    #[error("Failed to create window surface: {0}")]
    FailedToCreateWindowSurface(String),

    #[error("Failed to load image: {0}")]
    FailedToLoadImage(String),

    #[error("Failed to load font: {0}")]
    FailedToLoadFont(String),

    #[error("Failed to open font file: {0}")]
    FailedToOpenFontFile(String),

    #[error("Failed to create font texture: {0}")]
    FailedToCreateFontTexture(String),

    #[error("Failed to create font texture image: {0}")]
    FailedToCreateFontTextureImage(String),

    #[error("Create window surface with async web handler")]
    CreateSurfaceWithAsyncWebHandler()
}
