use image::{Rgb, RgbImage, Rgba, RgbaImage};

const WHITE: Rgba<u8> = Rgba([255, 255, 255, 0]);

#[derive(Debug)]
pub struct Background {
    pub color: Rgba<u8>,
    delta: u8,
}

impl Background {
    pub fn white() -> Self {
        Self {
            color: WHITE,
            delta: 3,
        }
    }

    pub fn from_rgb(rgb: Rgb<u8>) -> Self {
        Self {
            color: Rgba([rgb.0[0], rgb.0[1], rgb.0[2], 0]),
            delta: 3,
        }
    }

    pub fn is_background(&self, pixel: Rgba<u8>) -> bool {
        let rgb = pixel.0;

        if rgb[3] == 0 {
            return true;
        }

        rgb.iter().all(|x| x > &(u8::MAX - self.delta))
    }

    pub fn is_row_neighbours_white(&self, image: &RgbaImage, x: u32, y: u32) -> bool {
        let trigger: u8 = 3;
        let mut count: u8 = 0;

        for i in x.saturating_sub(1)..=x.saturating_add(1) {
            if i < image.width() && !self.is_background(*image.get_pixel(i, y)) {
                count += 1;
            }

            if count >= trigger {
                return false;
            }
        }

        true
    }

    pub fn is_column_neighbours_white(&self, image: &RgbaImage, x: u32, y: u32) -> bool {
        let trigger: u8 = 3;
        let mut count: u8 = 0;

        for i in y.saturating_sub(1)..=y.saturating_add(1) {
            if i < image.height() && !self.is_background(*image.get_pixel(x, i)) {
                count += 1;
            }

            if count >= trigger {
                return false;
            }
        }

        true
    }

    pub fn rgb_pixel_with_background(&self, pixel: Rgba<u8>) -> Rgb<u8> {
        let alpha = pixel.0[3] as f32 / 255.0;

        Rgb([
            (self.color.0[0] as f32 * (1.0 - alpha) + pixel.0[0] as f32 * alpha) as u8,
            (self.color.0[1] as f32 * (1.0 - alpha) + pixel.0[1] as f32 * alpha) as u8,
            (self.color.0[2] as f32 * (1.0 - alpha) + pixel.0[2] as f32 * alpha) as u8,
        ])
    }

    pub fn set_background(&self, image: &RgbaImage) -> RgbImage {
        let (width, height) = image.dimensions();
        let mut new_image = RgbImage::new(width, height);

        for y in 0..height {
            for x in 0..width {
                new_image.put_pixel(x, y, self.rgb_pixel_with_background(*image.get_pixel(x, y)));
            }
        }

        new_image
    }
}

pub fn filter_alpha(image: &RgbaImage, alpha_filter: u8) -> RgbaImage {
    let (width, height) = image.dimensions();

    let mut new_image = RgbaImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);

            new_image.put_pixel(
                x,
                y,
                if pixel.0[3] < alpha_filter {
                    Rgba([0, 0, 0, 0])
                } else {
                    *pixel
                },
            );
        }
    }

    new_image
}
