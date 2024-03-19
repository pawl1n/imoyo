mod args;
mod background;
mod crop;
mod scaler;

use background::Background;
use crop::Crop;

use image::io::Reader as ImageReader;
use image::DynamicImage;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;

fn main() {
    let args = args::Args::get();

    get_image_paths(&args.ignored)
        .iter()
        .for_each(|image_path| {
            let image_name = std::path::Path::new(&image_path)
                .file_stem()
                .unwrap_or_else(|| panic!("Invalid unicode for {image_path}"))
                .to_str()
                .unwrap_or_else(|| panic!("Invalid unicode for {image_path}"));

            let mut path = String::new();

            let mut image = ImageReader::open(image_path.clone())
                .unwrap_or_else(|err| panic!("Failed to open {image_path}: {err}"))
                .decode()
                .or_else(|_| {
                    ImageReader::new(BufReader::new(File::open(image_path)?))
                        .with_guessed_format()?
                        .decode()
                })
                .unwrap_or_else(|err| panic!("Failed to decode {image_path}: {err}"))
                .to_rgba8();

            let crop = Crop::new(args.padding, Background::white());

            if let Some(alpha_filter) = args.alpha_filter {
                if args.verbose {
                    println!("Applying alpha filter {alpha_filter} to image {image_name}");
                }
                image = background::filter_alpha(&image, alpha_filter);
                path.push_str("-a");
            }

            if args.crop {
                if args.verbose {
                    println!("Cropping image {image_name}");
                }
                image = crop.crop_to_object(&image);
                path.push_str("-c");
            }

            if args.square {
                if args.verbose {
                    println!("Cropping image {image_name} to square");
                }
                image = crop.fill_to_square(&image);
                path.push_str("-s");
            }

            if let Some(scaler) = &args.scaler {
                if args.verbose {
                    println!("Resizing image {image_name}");
                }
                image = scaler.resize(DynamicImage::ImageRgba8(image)).to_rgba8();
                path.push_str("-r");
            }

            if args.edge_detection.in_use {
                if args.verbose {
                    println!("Detecting edges in image {image_name}");
                }
                image = crop.crop_to_edges_canny(
                    &image,
                    args.edge_detection.low_threshold,
                    args.edge_detection.high_threshold,
                    args.verbose,
                );
                path.push_str("-e");
            }

            if !path.is_empty() {
                let bg = args
                    .background
                    .map_or(Background::white(), Background::from_rgb);

                if args.verbose {
                    println!("Setting background {:?}", bg.color);
                }

                image = bg.set_background(&image);

                image
                    .save(image_name.to_string() + &path + "-processed." + &args.extension)
                    .unwrap_or_else(|err| panic!("Failed to save image {image_name}: {err}"));

                println!("Processed image {image_name} {:?}", image.dimensions());
            }
        });
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
                } else {
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
        .unwrap_or_else(|| panic!("Invalid image name for {url}"))
        .split('?')
        .next()
        .unwrap_or_else(|| panic!("Invalid image name for {url}"));
    let mut file =
        File::create(name).unwrap_or_else(|_| panic!("Failed to create temp file {name}"));

    let mut buf: Vec<u8> = Vec::new();

    println!("Downloading image {name}");

    let _ = ureq::get(url)
        .call()
        .unwrap_or_else(|err| panic!("Failed to download image {name}: {err}"))
        .into_reader()
        .read_to_end(&mut buf);

    file.write_all(&buf)
        .unwrap_or_else(|err| panic!("Failed to save image {name}: {err}"));

    name.to_owned()
}

fn is_image(image_path: &str) -> bool {
    image_path.ends_with(".jpg")
        || image_path.ends_with(".png")
        || image_path.ends_with(".jpeg")
        || image_path.ends_with(".webp")
}
