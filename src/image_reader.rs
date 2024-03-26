use std::{fs::File, io::BufReader};

use image::{io::Reader, DynamicImage};

pub struct ImageProperties<T: From<DynamicImage>> {
    pub name: String,
    pub image: T,
}

impl<T: From<DynamicImage>> ImageProperties<T> {
    pub fn read(image_path: &str) -> Result<ImageProperties<T>, String> {
        let name = Self::read_image_name(image_path)?;
        let image: T = Self::read_image(image_path)?.into();

        Ok(ImageProperties { name, image })
    }
    fn read_image_name(image_path: &str) -> Result<String, String> {
        std::path::Path::new(image_path)
            .file_stem()
            .ok_or_else(|| format!("Invalid unicode for {image_path}"))?
            .to_str()
            .map_or(Err(format!("Invalid unicode for {image_path}")), |name| {
                Ok(name.to_string())
            })
    }

    fn read_image(image_path: &str) -> Result<DynamicImage, String> {
        Reader::open(image_path)
            .map_err(|err| format!("Failed to open {image_path}: {err}"))?
            .decode()
            .or_else(|_| {
                Reader::new(BufReader::new(File::open(image_path)?))
                    .with_guessed_format()?
                    .decode()
            })
            .map_err(|err| format!("Failed to decode {image_path}: {err}"))
    }
}
