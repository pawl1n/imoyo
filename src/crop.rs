use image::{DynamicImage, GrayImage, RgbaImage};
use imageproc::edges;

use crate::background::Background;

pub struct Crop {
    padding: u32,
    bg: Background,
}

#[derive(Debug)]
struct ObjectInfo {
    min_x: i32,
    min_y: i32,
    max_x: i32,
    max_y: i32,
}

impl Crop {
    pub fn new(padding: u32, bg: Background) -> Self {
        Self { padding, bg }
    }

    pub fn crop_to_object(&self, image: &RgbaImage) -> RgbaImage {
        let object = self.object_info(image);

        self.get_object(image, object)
    }

    fn object_info(&self, image: &RgbaImage) -> ObjectInfo {
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

        ObjectInfo {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    fn get_object(&self, image: &RgbaImage, object: ObjectInfo) -> RgbaImage {
        let (width, height) = image.dimensions();
        let (width_new, height_new) = (
            (object.max_x - object.min_x + 1) as u32,
            (object.max_y - object.min_y + 1) as u32,
        );
        let mut object_image = RgbaImage::new(width_new, height_new);

        for (x, y, pixel) in object_image.enumerate_pixels_mut() {
            if x as i32 + object.min_x < 0
                || x as i32 + object.min_x >= width as i32
                || y as i32 + object.min_y < 0
                || y as i32 + object.min_y >= height as i32
            {
                *pixel = self.bg.color;
            } else {
                *pixel = *image.get_pixel(
                    (x as i32 + object.min_x) as u32,
                    (y as i32 + object.min_y) as u32,
                );
            }
        }

        object_image
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

    pub fn crop_to_edges_canny(
        &self,
        image: &RgbaImage,
        low_threshold: f32,
        high_threshold: f32,
        save_edges: bool,
    ) -> RgbaImage {
        let gray_image: GrayImage = DynamicImage::ImageRgba8(image.clone()).to_luma8();
        let edges = edges::canny(&gray_image, low_threshold, high_threshold);
        if save_edges {
            edges.save("edges.jpg").unwrap();
        }

        self.get_obj(image, &edges)
    }

    fn get_obj(&self, image: &RgbaImage, edges: &GrayImage) -> RgbaImage {
        let object = self.object_info_gray(edges);
        let (width_new, height_new) = (
            (object.max_x - object.min_x + 1) as u32,
            (object.max_y - object.min_y + 1) as u32,
        );
        let mut object_image = RgbaImage::new(width_new, height_new);

        for y in 0..height_new {
            let mut leftmost = false;

            let original_y = (y as i32 + object.min_y) as u32;

            let mut rightmost = 0;

            for x in 0..width_new {
                let original_x = (x as i32 + object.min_x) as u32;

                if edges.get_pixel(original_x, original_y)[0] == u8::MAX
                    || self.are_neighbours_edges(edges, original_x, original_y)
                {
                    rightmost = x;
                }
            }

            for x in 0..width_new {
                let original_x = (x as i32 + object.min_x) as u32;

                if !leftmost && edges.get_pixel(original_x, original_y)[0] == u8::MAX
                    || self.are_neighbours_edges(edges, original_x, original_y)
                {
                    leftmost = true;
                    object_image.put_pixel(x, y, *image.get_pixel(original_x, original_y));
                } else if !leftmost || x > rightmost {
                    object_image.put_pixel(x, y, self.bg.color);
                } else {
                    object_image.put_pixel(x, y, *image.get_pixel(original_x, original_y));
                }
            }
        }

        object_image
    }

    fn are_neighbours_edges(&self, image: &GrayImage, x: u32, y: u32) -> bool {
        let radius = 2;

        for i in x.saturating_sub(radius)..=x.saturating_add(radius).min(image.width()) {
            for j in y.saturating_sub(radius)..=y.saturating_add(radius).min(image.height()) {
                if j == y {
                    break;
                }
                if image.get_pixel(i, j)[0] == u8::MAX {
                    return true;
                }
            }
        }

        false
    }

    fn object_info_gray(&self, image: &GrayImage) -> ObjectInfo {
        let (width, height) = image.dimensions();
        let mut min_x = width / 2;
        let mut min_y = height / 2;
        let mut max_x = width / 2;
        let mut max_y = height / 2;

        for y in 0..height {
            for x in 0..width {
                if image.get_pixel(x, y)[0] != 0 {
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

        let max_x: i32 = max_x as i32 + self.padding as i32;
        let min_x: i32 = min_x as i32 - self.padding as i32;
        let max_y: i32 = max_y as i32 + self.padding as i32;
        let min_y: i32 = min_y as i32 - self.padding as i32;

        ObjectInfo {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }
}
