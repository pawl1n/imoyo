mod image_tools;
use image_tools::*;

use image::io::Reader as ImageReader;

const EXTENSION: &str = "jpg";

fn main() {
    let mut image_path = std::env::args().last().expect("No image name given");

    if image_path.starts_with("http") {
        let name = image_path.split('/').last().expect("Invalid image name");

        let mut file = std::fs::File::create(name).expect("Failed to create temp file");
        reqwest::blocking::get(&image_path)
            .expect("Failed to download image")
            .copy_to(&mut file)
            .expect("Failed to save image");

        image_path = name.to_owned();
    }

    let image_name = std::path::Path::new(&image_path)
        .file_stem()
        .expect("Can't read file stem name")
        .to_str()
        .expect("Invalid unicode");

    let mut image = ImageReader::open(image_path.clone())
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image")
        .to_rgba8();

    let mut padding: u32 = 0;
    if let Some(i) = std::env::args()
        .enumerate()
        .find(|(_, arg)| arg.starts_with('-') && arg.contains('p'))
        .map(|(i, _)| i)
    {
        padding = std::env::args()
            .nth(i + 1)
            .expect("No padding given")
            .parse::<u32>()
            .expect("Invalid padding");
    }

    if std::env::args().any(|arg| arg.starts_with('-') && arg.contains('c')) {
        image = crop_white(&image, padding);
        let cropped_image_name = image_name.to_owned() + "-cropped." + EXTENSION;
        image
            .save(cropped_image_name.clone())
            .expect("Failed to save image");

        println!(
            "Saved cropped image {cropped_image_name} {:?}",
            image.dimensions()
        );
    }

    if std::env::args().any(|arg| arg.starts_with('-') && arg.contains('s')) {
        let square_image = fill_to_square(&image);
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
