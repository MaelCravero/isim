mod common;
mod geometry;
mod image;
mod scene;

use std::fs::File;
use std::path::Path;

use common::Color;
use image::Image;

fn save_image(path: &str, image: Image) {
    let path = Path::new(&path);
    let file = File::create(path);

    if let Ok(mut file) = file {
        match image.to_ppm(&mut file) {
            Ok(_) => println!("Success!"),
            _ => println!("Could not write: {}", path.display()),
        }
    } else {
        println!("Could not open: {}", path.display())
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut image = Image::new(400, 500);
    (0..100).for_each(|x| (0..50).for_each(|y| image.set(x, y, common::RED)));

    save_image(&args[1], image)
}
