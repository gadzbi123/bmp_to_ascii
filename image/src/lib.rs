mod char_brightness;
mod interpolation;
mod tools;
use crate::char_brightness::get_char_by_brightness_large;
use crate::interpolation::{interpolate, interpolated_index};
use crate::tools::index_as_u32;
use std::fs::read;

const COLORS_PER_PIXEL: usize = 3;
#[derive(Debug, Default, Clone)]
pub struct Image {
    value: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(path: &str) -> Self {
        let file = read(path).expect("File doesn't exist");
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
        result
    }
    pub fn draw(&self) {
        let mut ascii = String::new();
        for row in &self.value {
            let mut ascii_row = String::new();
            for pixel in row {
                ascii_row += get_char_by_brightness_large(*pixel);
            }
            ascii_row += "\n";
            ascii_row.push_str(&ascii);
            ascii = ascii_row;
        }
        print!("{}", ascii);
    }
    pub fn resize(&mut self, width: usize, height: usize) -> Self {
        let mut image_new = vec![vec![0_u8; width]; height];
        for (j, row) in image_new.iter_mut().enumerate() {
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
            value: image_new,
            width,
            height,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}