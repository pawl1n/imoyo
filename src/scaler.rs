use image::imageops::FilterType;
use image::DynamicImage;

#[derive(Debug)]
pub struct Scaler {
    filter_type: FilterType,
    width: u32,
}

impl Scaler {
    pub fn new(filter_type: FilterType, width: u32) -> Self {
        Self { filter_type, width }
    }

    pub fn resize(&self, image: DynamicImage) -> DynamicImage {
        let height = (image.height() as f32 / image.width() as f32 * self.width as f32) as u32;

        image.resize(self.width, height, self.filter_type)
    }
}
