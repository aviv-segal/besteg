use image::DynamicImage;
use besteg::{Besteg, encoders::lsb::LSB};

fn main() {
    let mut image = DynamicImage::new_rgba8(100, 100);
    let message = b"Hello, World!";

    // Encode
    match Besteg::encode::<LSB>(&mut image, message) {
        Ok(encoded_image) => {
            encoded_image.save("encoded_image.png").unwrap();
            println!("Message encoded and saved successfully!");
        }
        Err(e) => println!("Error encoding image: {:?}", e),
    }

    // Decode
    let encoded_image = image::open("encoded_image.png").unwrap();
    match Besteg::decode::<LSB>(&encoded_image) {
        Ok(decoded_message) => println!("Decoded message: {}", String::from_utf8_lossy(&decoded_message)),
        Err(e) => println!("Error decoding image: {:?}", e),
    }
}
