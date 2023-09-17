use image::Image;

const NEW_WIDTH: usize = 100;
const NEW_HEIGHT: usize = 100;

fn main() {
    let mut image_old = Image::new("./images/xd_25.bmp");
    image_old.draw();
    let image_new = image_old.resize(NEW_WIDTH, NEW_HEIGHT);
    println!("new:");
    image_new.draw();
}
