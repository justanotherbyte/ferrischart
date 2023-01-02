//! Holds the crate's `Result` type and error enum

use image::ImageError;

#[derive(Debug)]
pub enum ChartErrors {
    ImageError,
}

pub type ChartResult<T> = Result<T, ChartErrors>;

impl From<ImageError> for ChartErrors {
    fn from(_: ImageError) -> Self {
        ChartErrors::ImageError
    }
}
