use image::{imageops::FilterType, Rgb};

use crate::scaler::Scaler;

#[derive(Debug)]
pub struct Args {
    pub crop: bool,
    pub square: bool,
    pub scaler: Option<Scaler>,
    pub padding: u32,
    pub ignored: Vec<usize>,
    pub extension: String,
    pub alpha_filter: Option<u8>,
    pub background: Option<Rgb<u8>>,
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

        let upscaler = Self::read_scaler(&mut ignored);

        let extension =
            Self::get_parameter("e", &mut ignored).unwrap_or_else(|| String::from("jpg"));

        let alpha_filter = Self::get_parameter("a", &mut ignored).map(|a| {
            a.parse::<u8>()
                .unwrap_or_else(|err| panic!("Failed to parse alpha filter: {err}"))
        });

        let background = Self::get_parameter("b", &mut ignored).map(|b| {
            let rgb = b
                .split(',')
                .map(|x| {
                    x.trim()
                        .parse::<u8>()
                        .unwrap_or_else(|err| panic!("Failed to parse background: {err}"))
                })
                .collect::<Vec<u8>>();
            if rgb.len() != 3 {
                panic!("Background must be r,g,b");
            }

            Rgb([rgb[0], rgb[1], rgb[2]])
        });

        Self {
            crop,
            square,
            scaler: upscaler,
            padding,
            ignored,
            extension,
            alpha_filter,
            background,
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

    fn read_scaler(ignored: &mut Vec<usize>) -> Option<Scaler> {
        let width = Self::get_parameter("w", ignored).map(|w| {
            w.parse::<u32>()
                .unwrap_or_else(|err| panic!("Failed to parse width: {err}"))
        })?;

        let filter_type: FilterType =
            Self::get_parameter("f", ignored).map_or(FilterType::Lanczos3, |f| match f.as_str() {
                "n" => FilterType::Nearest,
                "t" => FilterType::Triangle,
                "c" => FilterType::CatmullRom,
                "g" => FilterType::Gaussian,
                "l" => FilterType::Lanczos3,
                &_ => panic!("Unknown filter type: {f}"),
            });

        println!("Filter: {filter_type:?}");

        Some(Scaler::new(filter_type, width))
    }
}
