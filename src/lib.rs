extern crate image;

pub mod encoders {
    pub mod lsb; // LSB encoding/decoding module
    // Add more modules for other methods, e.g., pub mod dct;
}

use image::DynamicImage;


/// Errors that can occur during encoding/decoding
#[derive(Debug)]
pub enum BestegError {
    UnsupportedFormat,
    MessageTooLarge,
    ImageProcessingError(String),
    InvalidInput(String),
}

/// Trait for implementing different steganography methods
pub trait SteganographyMethod {
    fn encode(image: &mut DynamicImage, message: &[u8]) -> Result<DynamicImage, BestegError>;
    fn decode(image: &DynamicImage) -> Result<Vec<u8>, BestegError>;
}

/// Encapsulation of the Besteg steganography tool
pub struct Besteg;

impl Besteg {
    /// Encodes a message into an image using the specified method
    pub fn encode<M: SteganographyMethod>(image: &mut DynamicImage,message: &[u8],) -> Result<DynamicImage, BestegError> {
        M::encode(image, message)
    }

    /// Decodes a hidden message from an image using the specified method
    pub fn decode<M: SteganographyMethod>(image: &DynamicImage,) -> Result<Vec<u8>, BestegError> {
        M::decode(image)
    }
}
