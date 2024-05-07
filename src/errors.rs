use std::fmt::Display;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ThreeBodyError {
    ImageError {
        #[from]
        source: image::ImageError,
    },
    EXRError{
        #[from]
        source: exr::error::Error
    }
}

impl Display for ThreeBodyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type Result<T> = std::result::Result<T, ThreeBodyError>;
