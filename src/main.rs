use std::fs::read;
use std::mem::size_of;
use std::rc::Rc;

fn index_as_u32(file: &Vec<u8>, index: usize) -> usize {
    let a = file.as_slice();
    let b = a[index..index + 4]
        .try_into()
        .unwrap_or_else(|x| panic!("X is {}", x));
    u32::from_le_bytes(b) as usize
}
const COLORS_PER_PIXEL: usize = 3;
fn write_from_file(file: Vec<u8>) {
    let mut next_row = index_as_u32(&file, 0x0A) as usize;
    let width = index_as_u32(&file, 0x12);
    let height = index_as_u32(&file, 0x16);
    let row_width = (width * COLORS_PER_PIXEL + 1) as usize;
    let mut result = String::new();
    while next_row < (height as usize * row_width) {
        let mut row_str = String::new();
        let row: Vec<u8> = file
            .iter()
            .skip(next_row)
            .take(row_width)
            .cloned()
            .collect();
        let pixels = row.chunks(COLORS_PER_PIXEL as usize);
        for pixel in pixels {
            if pixel.len() == 1 {
                row_str += "\n";
                continue;
            }
            let gray_scaled =
                0.2989 * pixel[0] as f64 + 0.5870 * pixel[1] as f64 + 0.1140 * pixel[2] as f64;

            match gray_scaled {
                alpha if alpha > 203.0 => row_str += "@",
                alpha if alpha > 153.0 => row_str += "b",
                alpha if alpha > 101.0 => row_str += "(",
                alpha if alpha > 51.0 => row_str += "^",
                _ => row_str += ".",
            }
        }
        row_str.push_str(&result);
        result = row_str;
        next_row += row_width;
    }
    dbg!(next_row, width, height, row_width);
    print!("{}", &result);
}
fn file_to_2d_vec(file: Vec<u8>) -> Vec<Vec<f64>> {
    let new_file = Vec::from([0; 20 * 20]);
    let mut next_row = index_as_u32(&file, 0x0A);
    let width = index_as_u32(&file, 0x12);
    let height = index_as_u32(&file, 0x16);
    let row_width = (width * COLORS_PER_PIXEL + 1);
    let mut result = Vec::new();
    while next_row < (height * row_width) {
        let row_file: Vec<u8> = file
            .iter()
            .skip(next_row)
            .take(row_width - 1)
            .cloned()
            .collect();
        let pixels = row_file.chunks_exact(COLORS_PER_PIXEL);
        let mut row = Vec::new();
        for pixel in pixels {
            let gray_scaled =
                0.2989 * pixel[0] as f64 + 0.5870 * pixel[1] as f64 + 0.1140 * pixel[2] as f64;
            row.push(gray_scaled);
        }
        result.push(row);
        next_row += row_width;
    }
    result
}
fn interpolated_index(i: usize, old: usize, new: usize) -> f64 {
    let separator = old as f64 / new as f64;
    (i as f64) * separator //
}
fn interpolate_2(file: &[Vec<f64>], index_interpolated: f64, jndex_interpolated: f64) -> u8 {
    let index_trunc = f64::floor(index_interpolated);
    let index_full = f64::ceil(index_interpolated);
    let jndex_trunc = f64::floor(jndex_interpolated);
    let jndex_full = f64::ceil(jndex_interpolated);
    let pixel_row0 = (index_full - index_interpolated)
        * file[jndex_trunc as usize][index_trunc as usize]
        + (index_interpolated - index_trunc) * file[jndex_trunc as usize][index_full as usize];
    let pixel_row1 = (index_full - index_interpolated)
        * file[jndex_full as usize][index_trunc as usize]
        + (index_interpolated - index_trunc) * file[jndex_full as usize][index_full as usize];
    let pixel = (jndex_full - jndex_interpolated) * pixel_row0
        + (jndex_interpolated - jndex_trunc) * pixel_row1;
    pixel as u8
}
fn interpolate(file: &[Vec<f64>], mut x: f64, mut y: f64, width: usize, height: usize) -> u8 {
    let x_trunc = f64::floor(x) as usize;
    let y_trunc = f64::floor(y) as usize;
    let x_full = if (f64::ceil(x) as usize) < width {
        f64::ceil(x) as usize
    } else {
        x_trunc
    };
    let y_full = if (f64::ceil(y) as usize) < height {
        f64::ceil(y) as usize
    } else {
        y_trunc
    };
    x %= 1.0;
    y %= 1.0;
    let res = file[y_trunc][x_trunc] * (1.0 - x % 1.0) * (1.0 - y)
        + file[y_full][x_trunc] * x * (1.0 - y)
        + file[y_trunc][x_full] * (1.0 - x) * y
        + file[y_full][x_full] * x * y;
    println!("({},{})={}", x, y, res);
    res as u8
}
fn draw_from_2d(image: Vec<Vec<u8>>) {
    let mut ascii = String::new();
    for row in image {
        let mut ascii_row = String::new();
        for pixel in row {
            match pixel {
                pixel if pixel > 203 => ascii_row += "@",
                pixel if pixel > 153 => ascii_row += "b",
                pixel if pixel > 101 => ascii_row += "(",
                pixel if pixel > 51 => ascii_row += "^",
                _ => ascii_row += ".",
            }
        }
        ascii_row += "\n";
        ascii_row.push_str(&ascii);
        ascii = ascii_row;
    }
    print!("{}", ascii);
}
const NEW_WIDTH: usize = 102;
const NEW_HEIGHT: usize = 55;
fn main() {
    dbg!(std::env::current_dir());
    let file = read("./images/ubuntu.bmp").unwrap();
    let old_width = index_as_u32(&file, 0x12);
    let old_height = index_as_u32(&file, 0x16);
    // MAX size 275 x 50
    let image_old = file_to_2d_vec(file);
    let mut image_new = vec![vec![0_u8; NEW_WIDTH]; NEW_HEIGHT];
    for (j, row) in image_new.iter_mut().enumerate() {
        let jndex_interpolated = interpolated_index(j, old_height, NEW_HEIGHT); //[3/4,6/4,9/4] = .75,1.5,2.25
        for (i, value) in row.iter_mut().enumerate() {
            let index_interpolated = interpolated_index(i, old_width, NEW_WIDTH); //[3/5,6/5,9/5] = .6 1.2 1.8
            *value = interpolate(
                &image_old,
                index_interpolated,
                jndex_interpolated,
                old_width,
                old_height,
            );
        }
    }
    draw_from_2d(image_new);
    dbg!(60333.3270375 as u8);
}
