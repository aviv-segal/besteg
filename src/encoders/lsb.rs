use image::DynamicImage;
use crate::{SteganographyMethod, BestegError};

pub struct LSB;

impl SteganographyMethod for LSB {
    fn encode(image: &mut DynamicImage, message: &[u8]) -> Result<DynamicImage, BestegError> {
        if !LSB::can_store_message_rgba(image, message.len()) {
            return Err(BestegError::MessageTooLarge);
        }
        let message = [message, &[0]].concat();
        
        let mut bits = message.iter().flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1));
        let mut rgba_image = image.to_rgba8();

        for pixel in rgba_image.pixels_mut() {
            for channel in pixel.0.iter_mut() {
                if let Some(bit) = bits.next() {
                    *channel = (*channel & 0xFE) | bit;
                }
            }
        }

        Ok(DynamicImage::ImageRgba8(rgba_image)) // Return the new image
    }

    fn decode(image: &DynamicImage) -> Result<Vec<u8>, BestegError> {
        let mut bits = Vec::new();
        let rgba_image = image.to_rgba8();

        for pixel in rgba_image.pixels() {
            for channel in pixel.0.iter() {
                bits.push(channel & 1);
            }
        }

        let mut bytes = Vec::new();
        for chunk in bits.chunks(8) {
            if chunk.len() == 8 {
                let byte = chunk.iter().fold(0, |acc, &bit| (acc << 1) | bit);
                if byte == 0 {
                    break;
                }
                bytes.push(byte);
            }
        }

        Ok(bytes)
    }
}

impl LSB {
    pub fn can_store_message_rgba(image: &DynamicImage, message_length: usize) -> bool {
        let total_pixels = image.width() as usize * image.height() as usize;
        let total_bits = total_pixels * 4; // RGBA has 4 channels per pixel
        message_length * 8 <= total_bits
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use image::DynamicImage;

    #[test]
    fn test_lsb_encode_decode() {
        let mut image = DynamicImage::new_rgba8(100, 100);
        let message = b"Hello, World!";
        let encoded = LSB::encode(&mut image, message).unwrap();
        let decoded = LSB::decode(&encoded).unwrap();

       println!("Decoded message: {}", String::from_utf8_lossy(&decoded));

    }
}

