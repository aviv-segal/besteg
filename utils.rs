pub mod image_utils {
    use image::{DynamicImage, ImageFormat};
    use crate::BestegError;

    pub fn load_image(path: &str) -> Result<DynamicImage, BestegError> {
        image::open(path).map_err(|e| BestegError::ImageProcessingError(e.to_string()))
    }

    pub fn save_image(image: &DynamicImage, path: &str) -> Result<(), BestegError> {
        image.save(path).map_err(|e| BestegError::ImageProcessingError(e.to_string()))
    }

    pub fn detect_format(image: &DynamicImage) -> ImageFormat {
        // Add logic if needed for specific formats
        ImageFormat::Png // Default to PNG for now
    }
}
