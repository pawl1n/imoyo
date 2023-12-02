use image::imageops::FilterType;

use crate::upscaler::Upscaler;

#[derive(Debug)]
pub struct Args {
    pub crop: bool,
    pub square: bool,
    pub upscaler: Option<Upscaler>,
    pub padding: u32,
    pub ignored: Vec<usize>,
    pub extension: String,
}

impl Args {
    pub fn get() -> Self {
        let mut ignored: Vec<usize> = vec![0];

        let crop = std::env::args()
            .skip(1)
            .any(|arg| arg.starts_with('-') && arg.contains('c'));
        let square = std::env::args()
            .skip(1)
            .any(|arg| arg.starts_with('-') && arg.contains('s'));
        let padding = if crop {
            Self::get_parameter("p", &mut ignored).map_or(0, |p| {
                p.parse::<u32>()
                    .unwrap_or_else(|err| panic!("Failed to parse padding: {err}"))
            })
        } else {
            0
        };

        let upscaler = Self::read_upscaler(&mut ignored);

        let extension =
            Self::get_parameter("e", &mut ignored).unwrap_or_else(|| String::from("jpg"));

        Self {
            crop,
            square,
            upscaler,
            padding,
            ignored,
            extension,
        }
    }

    fn get_parameter(name: &str, ignored: &mut Vec<usize>) -> Option<String> {
        std::env::args()
            .enumerate()
            .skip(1)
            .find(|(_, arg)| arg.starts_with('-') && arg.contains(name))
            .map(|(i, _)| {
                ignored.push(i);
                ignored.push(i + 1);

                std::env::args()
                    .nth(i + 1)
                    .expect("Missing parameter value")
            })
    }

    fn read_upscaler(ignored: &mut Vec<usize>) -> Option<Upscaler> {
        let filter_type: FilterType =
            Self::get_parameter("u", ignored).map(|f| match f.as_str() {
                "n" => FilterType::Nearest,
                "t" => FilterType::Triangle,
                "c" => FilterType::CatmullRom,
                "g" => FilterType::Gaussian,
                "l" => FilterType::Lanczos3,
                &_ => panic!("Unknown filter type: {f}"),
            })?;

        let width = Self::get_parameter("w", ignored).map(|w| {
            w.parse::<u32>()
                .unwrap_or_else(|err| panic!("Failed to parse width: {err}"))
        })?;

        Some(Upscaler::new(filter_type, width))
    }
}
