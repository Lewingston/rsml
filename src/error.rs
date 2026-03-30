
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {

    #[error("Unable to create renderer: {0}")]
    FailedToCreateRenderer(String),

    #[error("Failed to create window: {0}")]
    FailedToCreateWindow(String),
}
