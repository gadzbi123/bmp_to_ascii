mod char_brightness;
mod interpolation;
mod tools;
use crate::char_brightness::get_char_by_brightness;
use crate::interpolation::{interpolate, interpolated_index};
use crate::tools::index_as_u32;
use rand::Rng;
use std::ffi::OsStr;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;

const COLORS_PER_PIXEL: usize = 3;
#[derive(Debug, Default, Clone)]
pub struct Image {
    value: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn demo_file() -> PathBuf {
        let images = match fs::read_dir("./images") {
            Ok(images) => images,
            Err(e) => {
                eprintln!("demo images read error: {e}");
                exit(e.raw_os_error().unwrap_or(1))
            }
        };
        let rng = rand::thread_rng().gen_range(0..5);
        for (i, image) in images.enumerate() {
            if rng == i {
                return image.unwrap().path();
            }
        }
        return PathBuf::new();
    }
    pub fn new(path: PathBuf) -> Self {
        let file = match fs::read(path) {
            Ok(file) => file,
            Err(e) => {
                eprintln!("file read error: {e}");
                exit(e.raw_os_error().unwrap_or(1));
            }
        };
        let width = index_as_u32(&file, 0x12);
        let height = index_as_u32(&file, 0x16);
        let image = Self::file_to_image(file, width, height);
        Self {
            value: image,
            width,
            height,
        }
    }
    fn file_to_image(file: Vec<u8>, width: usize, height: usize) -> Vec<Vec<u8>> {
        let mut next_row = index_as_u32(&file, 0x0A);
        let pix_width = width * COLORS_PER_PIXEL;
        let padding = if pix_width % 4 == 0 {
            0
        } else {
            4 - pix_width % 4
        };
        let mut result = Vec::new();
        while next_row < (height * (pix_width + padding)) {
            let row_file: Vec<u8> = file
                .iter()
                .skip(next_row)
                .take(pix_width)
                .cloned()
                .collect();
            let pixels = row_file.chunks_exact(COLORS_PER_PIXEL);
            let mut row = Vec::new();
            for pixel in pixels {
                let gray_scaled =
                    0.2989 * pixel[0] as f64 + 0.5870 * pixel[1] as f64 + 0.1140 * pixel[2] as f64;
                row.push(gray_scaled as u8);
            }
            result.push(row);
            next_row += pix_width + padding;
        }
        result.reverse();
        result
    }
    fn to_string(&self, inverse: bool) -> String {
        let mut result = String::new();
        let color_mode = if !inverse {
            get_char_by_brightness::regular
        } else {
            get_char_by_brightness::inverse
        };
        for row in &self.value {
            let mut result_row = String::new();
            for pixel in row {
                result_row += color_mode(*pixel);
            }
            result_row += "\n";
            result += result_row.as_str();
        }
        result
    }
    pub fn save(&self, path: PathBuf, inverse: bool) -> std::io::Result<()> {
        let mut file = fs::File::create(path)?;
        let ascii = self.to_string(inverse);
        file.write_all(ascii.as_bytes())?;
        Ok(())
    }
    pub fn draw(&self, inverse: bool) {
        let ascii = self.to_string(inverse);
        print!("{}", ascii);
    }
    pub fn resize(&mut self, width: usize, height: usize) -> Self {
        let mut value = vec![vec![0_u8; width]; height];
        for (j, row) in value.iter_mut().enumerate() {
            let j_interpolated = interpolated_index(j, self.height, height);
            for (i, value) in row.iter_mut().enumerate() {
                let i_interpolated = interpolated_index(i, self.width, width);
                *value = interpolate(
                    &self.value,
                    i_interpolated,
                    j_interpolated,
                    self.height,
                    self.width,
                );
            }
        }
        Self {
            value,
            width,
            height,
        }
    }
}
