use image::DynamicImage;
pub struct LSB;

impl LSB {
    pub fn encode(input: &[u8], image: Vec<u8>) -> Vec<u8> {
        todo!()
    }

    /// Encodes a message into an image with RGBA pixels using LSB.
    pub fn encode_rgba(image: &mut DynamicImage, message: &[u8]) -> DynamicImage{
        if !LSB::can_store_message_rgba(image, message.len()) {
            panic!("Message is too long to fit in the image");
        }
        let mut bits = message.iter().flat_map(|byte| (0..8).rev().map(move |i| (byte >> i) & 1));
    
        let mut rgba_image = image.to_rgba8(); // This gives us a mutable RgbaImage

        for pixel in rgba_image.pixels_mut() {
            for channel in pixel.0.iter_mut() {
                if let Some(bit) = bits.next() {
                    *channel = (*channel & 0xFE) | bit; 
                }   
            }
        }
        DynamicImage::ImageRgba8(rgba_image)

        }

    pub fn decode_rgba(image: &DynamicImage) -> Vec<u8> {
        let mut bits = Vec::new();
        let rgba_image = image.to_rgba8(); 
    
        for pixel in rgba_image.pixels() {
            for channel in pixel.0.iter() {
                let lsb = channel & 1;
                bits.push(lsb);
            }
        }
        let mut bytes = Vec::new();
        for chunk in bits.chunks(8) {
            if chunk.len() == 8 {
                let byte = chunk.iter().fold(0, |acc, &bit| (acc << 1) | bit);
                bytes.push(byte);
            }
        }
        bytes
    }
    pub fn can_store_message_rgba(image: &DynamicImage, message_length: usize) -> bool {
        let total_pixels = image.width() as usize * image.height() as usize;
        let total_bits = total_pixels * 4; 
        message_length * 8 <= total_bits 
    }
    
    pub fn save_image(image: DynamicImage, output_path: &str) {
        image.save(output_path).unwrap(); // Save the modified image
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode() {
        let input = b"Hello, world!";
        let mut image = image::open("/home/avivs/Desktop/besteg/example.png").unwrap();
        let encoded_image = LSB::encode_rgba(&mut image, input);
        let output = "/home/avivs/Desktop/besteg/output.png";
        assert_ne!(encoded_image, image);
        LSB::save_image(encoded_image, output);
    }
    #[test]
    fn test_decode() {
        let image = image::open("/home/avivs/Desktop/besteg/output.png").unwrap();
        let message = LSB::decode_rgba(&image);
        println!("Decoded message: {}", String::from_utf8_lossy(&message));
    }
}
