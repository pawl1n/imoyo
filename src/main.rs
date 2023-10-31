mod image_tools;
use image_tools::*;

use image::io::Reader as ImageReader;

fn main() {
    let image_path = std::env::args().last().expect("No image name given");
    let image_name = std::path::Path::new(&image_path)
        .file_stem()
        .expect("Can't read file stem name")
        .to_str()
        .expect("Invalid unicode");

    let image = ImageReader::open(image_path.clone())
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let cropped_image = crop_white(&image.to_rgba8());
    println!("Cropped image size: {:?}", cropped_image.dimensions());

    if std::env::args().any(|arg| arg == "-s") {
        cropped_image
            .save(image_name.to_owned() + "-cropped.jpg")
            .expect("Failed to save image");

        let square_image = fill_to_square(&cropped_image);
        println!("Square image size: {:?}", square_image.dimensions());

        square_image
            .save(image_name.to_owned() + "-square.jpg")
            .expect("Failed to save image");
    }
}
