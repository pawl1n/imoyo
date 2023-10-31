use image::{Rgba, RgbaImage};

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 1]);

fn is_white(pixel: &Rgba<u8>) -> bool {
    let delta = 3;
    let rgb = pixel.0;

    if rgb[3] == 0 {
        return true;
    }

    rgb.iter().all(|x| x > &(u8::MAX - delta))
}

fn is_row_neighbours_white(image: &RgbaImage, x: u32, y: u32) -> bool {
    let trigger: u8 = 3;
    let mut count: u8 = 0;

    for i in x.saturating_sub(1)..=x.saturating_add(1) {
        if i < image.width() && !is_white(image.get_pixel(i, y)) {
            count += 1;
        }

        if count >= trigger {
            return false;
        }
    }

    true
}

fn is_column_neighbours_white(image: &RgbaImage, x: u32, y: u32) -> bool {
    let trigger: u8 = 3;
    let mut count: u8 = 0;

    for i in y.saturating_sub(1)..=y.saturating_add(1) {
        if i < image.height() && !is_white(image.get_pixel(x, i)) {
            count += 1;
        }

        if count >= trigger {
            return false;
        }
    }

    true
}

pub fn crop_white(image: &RgbaImage) -> RgbaImage {
    let (width, height) = image.dimensions();

    let mut min_x = width / 2;
    let mut min_y = height / 2;
    let mut max_x = width / 2;
    let mut max_y = height / 2;

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);

            if !is_white(pixel) {
                if x < min_x && !is_column_neighbours_white(image, x, y) {
                    min_x = x;
                }
                if y < min_y && !is_row_neighbours_white(image, x, y) {
                    min_y = y;
                }
                if x > max_x && !is_column_neighbours_white(image, x, y) {
                    max_x = x;
                }
                if y > max_y && !is_row_neighbours_white(image, x, y) {
                    max_y = y;
                }
            }
        }
    }

    let cropped_width = max_x - min_x;
    let cropped_height = max_y - min_y;

    let mut cropped_image = RgbaImage::new(cropped_width, cropped_height);

    for (x, y, pixel) in cropped_image.enumerate_pixels_mut() {
        *pixel = *image.get_pixel(x + min_x, y + min_y);
    }

    cropped_image
}

pub fn fill_to_square(image: &RgbaImage) -> RgbaImage {
    let (width, height) = image.dimensions();

    if width == height {
        return image.clone();
    }

    let side = width.max(height);
    let mut square_image = RgbaImage::new(side, side);
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