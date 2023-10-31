mod image_tools;
use image_tools::*;

use image::io::Reader as ImageReader;

const EXTENSION: &str = "jpg";

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
    let cropped_image_name = image_name.to_owned() + "-cropped." + EXTENSION;

    cropped_image
        .save(cropped_image_name.clone())
        .expect("Failed to save image");

    println!(
        "Saved cropped image {cropped_image_name} {:?}",
        cropped_image.dimensions()
    );

    if std::env::args().any(|arg| arg == "-s") {
        let square_image = fill_to_square(&cropped_image);
        let square_image_name = image_name.to_owned() + "-square." + EXTENSION;

        square_image
            .save(square_image_name.clone())
            .expect("Failed to save image");

        println!(
            "Saved square image {square_image_name} {:?}",
            square_image.dimensions()
        );
    }
}
