
use thiserror::Error;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Error)]
pub enum Error {

    #[error("Unable to create renderer: {0}")]
    FailedToCreateRenderer(String),

    #[error("Failed to create window: {0}")]
    FailedToCreateWindow(String),

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
}
