pub fn interpolated_index(i: usize, old: usize, new: usize) -> f64 {
    (i as f64) * old as f64 / new as f64
}
pub fn interpolate(file: &[Vec<u8>], mut x: f64, mut y: f64, width: usize, height: usize) -> u8 {
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
    let res = file[y_trunc][x_trunc] as f64 * (1.0 - x) * (1.0 - y)
        + file[y_full][x_trunc] as f64 * x * (1.0 - y)
        + file[y_trunc][x_full] as f64 * (1.0 - x) * y
        + file[y_full][x_full] as f64 * x * y;
    // println!("({},{})={}", x, y, res);
    res as u8
}
