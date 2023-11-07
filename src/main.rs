mod image_tools;
use image_tools::*;

use image::io::Reader as ImageReader;
use std::path::Path;

const EXTENSION: &str = "jpg";

fn main() {
    let crop = std::env::args().any(|arg| arg.starts_with('-') && arg.contains('c'));
    let square = std::env::args().any(|arg| arg.starts_with('-') && arg.contains('s'));
    let padding = if crop { get_padding() } else { 0 };

    get_image_paths().iter().for_each(|image_path| {
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

        if crop {
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

        if square {
            image = fill_to_square(&image);
            let square_image_name = image_name.to_owned() + "-square." + EXTENSION;

            image
                .save(square_image_name.clone())
                .expect("Failed to save image");

            println!(
                "Saved square image {square_image_name} {:?}",
                image.dimensions()
            );
        }
    });
}

fn get_padding() -> u32 {
    std::env::args()
        .enumerate()
        .find(|(_, arg)| arg.starts_with('-') && arg.contains('p'))
        .map_or(0, |(i, _)| {
            std::env::args()
                .nth(i + 1)
                .expect("No padding given")
                .parse::<u32>()
                .expect("Invalid padding")
        })
}

fn get_image_paths() -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();

    std::env::args()
        .filter(|arg| !arg.starts_with('-'))
        .for_each(|arg| {
            if arg.starts_with("http") {
                let name = arg.split('/').last().expect("Invalid image name");
                let mut file = std::fs::File::create(name).expect("Failed to create temp file");

                reqwest::blocking::get(&arg)
                    .expect("Failed to download image")
                    .copy_to(&mut file)
                    .expect("Failed to save image");

                paths.push(name.to_owned());
            } else if Path::new(&arg).exists() {
                let path = Path::new(&arg);

                if path.is_dir() {
                    paths.extend(
                        path.read_dir()
                            .expect("Failed to read dir")
                            .map(|entry| {
                                entry
                                    .expect("Failed to read dir entry")
                                    .path()
                                    .to_str()
                                    .expect("Invalid unicode")
                                    .to_owned()
                            })
                            .filter(|name| is_image(name)),
                    );
                } else if is_image(path.to_str().expect("Invalid unicode")) {
                    paths.push(arg.to_owned());
                }
            } else {
                println!("Can't find image {arg}");
            }
        });

    paths
}

fn is_image(image_path: &str) -> bool {
    image_path.ends_with(".jpg")
        || image_path.ends_with(".png")
        || image_path.ends_with(".jpeg")
        || image_path.ends_with(".webp")
}
