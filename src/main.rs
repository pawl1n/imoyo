mod image_tools;
use image_tools::*;

use image::io::Reader as ImageReader;
use std::fs::File;
use std::io::Write;
use std::path::Path;

const EXTENSION: &str = "jpg";

fn main() {
    let mut ignored: Vec<usize> = Vec::new();

    let crop = std::env::args().any(|arg| arg.starts_with('-') && arg.contains('c'));
    let square = std::env::args().any(|arg| arg.starts_with('-') && arg.contains('s'));
    let padding = if crop {
        get_padding().map_or(0, |(i, p)| {
            ignored.push(i);
            ignored.push(i - 1);

            p
        })
    } else {
        0
    };

    get_image_paths(&ignored).iter().for_each(|image_path| {
        let image_name = std::path::Path::new(&image_path)
            .file_stem()
            .unwrap_or_else(|| panic!("Invalid unicode for {image_path}"))
            .to_str()
            .unwrap_or_else(|| panic!("Invalid unicode for {image_path}"));

        let mut image = ImageReader::open(image_path.clone())
            .unwrap_or_else(|err| panic!("Failed to open {image_path}: {err}"))
            .decode()
            .unwrap_or_else(|err| panic!("Failed to decode {image_path}: {err}"))
            .to_rgba8();

        if crop {
            image = crop_white(&image, padding);

            let cropped_image_name = image_name.to_owned() + "-cropped." + EXTENSION;
            image
                .save(cropped_image_name.clone())
                .unwrap_or_else(|err| panic!("Failed to save image {cropped_image_name}: {err}"));

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
                .unwrap_or_else(|err| panic!("Failed to save image {square_image_name}: {err}"));

            println!(
                "Saved square image {square_image_name} {:?}",
                image.dimensions()
            );
        }
    });
}

fn get_padding() -> Option<(usize, u32)> {
    std::env::args()
        .enumerate()
        .find(|(_, arg)| arg.starts_with('-') && arg.contains('p'))
        .map(|(i, _)| {
            (
                i + 1,
                std::env::args()
                    .nth(i + 1)
                    .expect("Missing padding")
                    .parse::<u32>()
                    .unwrap_or_else(|err| panic!("Failed to parse padding: {err}")),
            )
        })
}

fn get_image_paths(ignored: &[usize]) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();

    std::env::args()
        .enumerate()
        .filter(|(_, arg)| !arg.starts_with('-'))
        .filter(|(i, _)| !ignored.contains(i))
        .map(|(_, arg)| arg)
        .for_each(|arg| {
            if arg.starts_with("http") {
                paths.push(download_image(&arg));
            } else if Path::new(&arg).exists() {
                let path = Path::new(&arg);

                if path.is_dir() {
                    paths.extend(
                        path.read_dir()
                            .unwrap_or_else(|err| panic!("Failed to read dir {arg}: {err}"))
                            .map(|entry| {
                                entry
                                    .unwrap_or_else(|err| panic!("Failed to read dir entry: {err}"))
                                    .path()
                                    .to_str()
                                    .unwrap_or_else(|| panic!("Invalid unicode in {arg} dir"))
                                    .to_owned()
                            })
                            .filter(|name| is_image(name)),
                    );
                } else if is_image(
                    path.to_str()
                        .unwrap_or_else(|| panic!("Invalid unicode in {arg}")),
                ) {
                    paths.push(arg.to_owned());
                }
            } else {
                println!("Can't find image {arg}");
            }
        });

    paths
}

fn download_image(url: &str) -> String {
    let name = url
        .split('/')
        .last()
        .unwrap_or_else(|| panic!("Invalid image name for {url}"));
    let mut file =
        File::create(name).unwrap_or_else(|_| panic!("Failed to create temp file {name}"));

    let response = attohttpc::get(url)
        .send()
        .unwrap_or_else(|err| panic!("Failed to send request {url}: {err}"))
        .error_for_status()
        .unwrap_or_else(|err| panic!("Failed to download from {url}: {err}"))
        .bytes()
        .unwrap_or_else(|err| panic!("Failed to extract bytes from {url}: {err}"));

    file.write_all(&response)
        .unwrap_or_else(|err| panic!("Failed to save image {name}: {err}"));

    name.to_owned()
}

fn is_image(image_path: &str) -> bool {
    image_path.ends_with(".jpg")
        || image_path.ends_with(".png")
        || image_path.ends_with(".jpeg")
        || image_path.ends_with(".webp")
}
