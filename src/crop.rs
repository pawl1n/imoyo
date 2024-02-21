use image::RgbaImage;

use crate::background::Background;

pub struct Crop {
    padding: u32,
    bg: Background,
}

impl Crop {
    pub fn new(padding: u32, bg: Background) -> Self {
        Self { padding, bg }
    }

    pub fn crop_white(&self, image: &RgbaImage) -> RgbaImage {
        let (width, height) = image.dimensions();

        let mut min_x = width / 2;
        let mut min_y = height / 2;
        let mut max_x = width / 2;
        let mut max_y = height / 2;

        for y in 0..height {
            for x in 0..width {
                if !self.bg.is_background(*image.get_pixel(x, y)) {
                    if x < min_x && !self.bg.is_column_neighbours_white(image, x, y) {
                        min_x = x;
                    }
                    if y < min_y && !self.bg.is_row_neighbours_white(image, x, y) {
                        min_y = y;
                    }
                    if x > max_x && !self.bg.is_column_neighbours_white(image, x, y) {
                        max_x = x;
                    }
                    if y > max_y && !self.bg.is_row_neighbours_white(image, x, y) {
                        max_y = y;
                    }
                }
            }
        }

        let max_x: i32 = max_x as i32 + self.padding as i32;
        let min_x: i32 = min_x as i32 - self.padding as i32;
        let max_y: i32 = max_y as i32 + self.padding as i32;
        let min_y: i32 = min_y as i32 - self.padding as i32;

        let cropped_width = (max_x - min_x + 1) as u32;
        let cropped_height = (max_y - min_y + 1) as u32;

        let mut cropped_image = RgbaImage::new(cropped_width, cropped_height);

        for (x, y, pixel) in cropped_image.enumerate_pixels_mut() {
            if x as i32 + min_x < 0
                || x as i32 + min_x >= width as i32
                || y as i32 + min_y < 0
                || y as i32 + min_y >= height as i32
            {
                *pixel = self.bg.color;
            } else {
                *pixel = *image.get_pixel((x as i32 + min_x) as u32, (y as i32 + min_y) as u32);
            }
        }

        cropped_image
    }

    pub fn fill_to_square(&self, image: &RgbaImage) -> RgbaImage {
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
                *pixel = self.bg.color;
            } else {
                *pixel = *image.get_pixel(x - padding_x, y - padding_y);
            }
        }
        square_image.clone()
    }
}
