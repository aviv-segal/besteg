

pub fn findEncoding(file_type: string){

}
use crate::encoding::EncodingMethod;

pub fn select_encoding_method(file_type: String) -> Option<EncodingMethod> {
    match file_type.to_lowercase().as_str() {
        "png" => Some(EncodingMethod::LSB),   // PNG supports LSB well
        "bmp" => Some(EncodingMethod::LSB),   // BMP is also great for LSB
        "jpeg" | "jpg" => Some(EncodingMethod::DCT), // JPEG might require DCT-based methods
        // Add other file types and methods here
        _ => None, // Unknown or unsupported file type
    }
}
pub fn get_file_exte(filename: String) -> String {

    //Change it to a canonical file path.
    let path = Path::new(&filename).canonicalize().expect(
        "Expecting an existing filename",
    );

    let filepath = path.to_str();
    let name = filepath.unwrap().split('/');
    let names: Vec<&str> = name.collect();
    let extension = names.last().expect("File extension can not be read.");
    let extens: Vec<&str> = extension.split(".").collect();

    extens[1..(extens.len())].join(".").to_string()
}
