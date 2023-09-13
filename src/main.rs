use std::fs::{read, read_link};

fn index_as_u32(file: &Vec<u8>, index: usize) -> u32 {
    let a = file.as_slice();
    let b = a[index..index + 4]
        .try_into()
        .unwrap_or_else(|x| panic!("X is {}", x));
    u32::from_le_bytes(b)
}
static COLOR_PER_PIXEL: u32 = 3;
fn main() {
    let file_str = read("./images/xd.bmp").unwrap();
    let start = index_as_u32(&file_str, 0x0A);
    let width = index_as_u32(&file_str, 0x12);
    let height = index_as_u32(&file_str, 0x16);
    let mut start_of_row = start as usize;
    let row_width = (width * COLOR_PER_PIXEL + 1) as usize;
    let mut result = String::new();
    while start_of_row < height as usize * row_width {
        let row: Vec<u8> = file_str
            .iter()
            .skip(start_of_row)
            .take(row_width)
            .cloned()
            .collect();
        let pixels = row.chunks(COLOR_PER_PIXEL as usize);
        for pixel in pixels {
            match pixel {
                [0, 0, 0] => result += " ",
                [255, 255, 255] => result += "#",
                [_] => result += "\n",
                _ => result += "x",
            }
        }
        start_of_row += row_width;
    }
    dbg!(start, width, height);
    print!("{}", &result);
    // let pixels: Vec<(u8, u8, u8)> = file_str.chunks(3);
}
