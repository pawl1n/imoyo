// use std::io::Cursor;
use image::{io::Reader as ImageReader, Rgb, RgbImage};

const WHITE: Rgb<u8> = Rgb([255, 255, 255]);

fn main() {
    let image_name = std::env::args().nth(1).expect("No image name given");

    let image = ImageReader::open(image_name)
        .expect("Failed to open image")
        .decode()
        .expect("Failed to decode image");

    let cropped_image = crop_white(&image.to_rgb8());
    cropped_image
        .save("cropped.jpg")
        .expect("Failed to save image");

    let square_image = fill_to_square(&cropped_image);
    square_image
        .save("square.jpg")
        .expect("Failed to save image");
}

fn crop_white(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();

    let mut min_x = width / 2;
    let mut min_y = height / 2;
    let mut max_x = width / 2;
    let mut max_y = height / 2;

    for x in 0..width {
        for y in 0..height {
            let pixel = image.get_pixel(x, y);

            if pixel != &WHITE {
                if x < min_x {
                    min_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
            }
        }
    }

    let cropped_width = max_x - min_x;
    let cropped_height = max_y - min_y;

    let mut cropped_image = RgbImage::new(cropped_width, cropped_height);

    for (x, y, pixel) in cropped_image.enumerate_pixels_mut() {
        *pixel = *image.get_pixel(x + min_x, y + min_y);
    }

    cropped_image
}

fn fill_to_square(image: &RgbImage) -> RgbImage {
    let (width, height) = image.dimensions();

    if width == height {
        return image.clone();
    }

    let side = width.max(height);
    let mut square_image = RgbImage::new(side, side);
    let padding_y = ((side - height) as f32 / 2.0).ceil() as u32;
    let padding_x = ((side - width) as f32 / 2.0).ceil() as u32;

    for (x, y, pixel) in square_image.enumerate_pixels_mut() {
        if x < padding_x || x >= side - padding_x || y < padding_y || y >= side - padding_y {
            *pixel = WHITE;
        } else {
            *pixel = *image.get_pixel(x - padding_x, y - padding_y);
        }
    }
    square_image.clone()
}
