This project is a all in one Steganography tool I'm making to make Steganography easier to use.


# Besteg
Steganography is complicated, especially for non-technincal people so I wanted to create a tool that is easy to use and supports almost all of the most popular image formats and Steganography methods.


## Usage
There are structs for each encoding method and each class have encode and decode methods.


Example:
```rust
fn main(){
    // For now it only works for RGBA.
    let mut image = DynamicImage::new_rgba8(100, 100);
    let message = b"Hello, World!"; // The message you want to encode.
    let encoded = LSB::encode(&mut image, message).unwrap();
    let decoded = LSB::decode(&encoded).unwrap();

    println!("Decoded message: {}", String::from_utf8_lossy(&decoded));
}
```