extern crate image;
use image::{GenericImageView};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;


fn main() {
    let img = image::open("./image.png").unwrap();
    let (width, height) = img.dimensions();
    let image_gray = img.to_luma();
    println!("width {:?}, height {:?}", width, height); 
    println!("{:?}", image_gray.get_pixel(2489, 1554)); 

    let path = Path::new("asciiart.txt");
    let display = path.display();
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    for i in 0..height{
        for j in 0..width{
            let pixel = image_gray.get_pixel(j, i);
            let character = get_character_for_grayscale(pixel.channels().first().unwrap()) as u8;
            file.write(&[character]);
        }
        write!(file, "\n");
    }
}

fn get_character_for_grayscale(value: &u8) -> char {
    let GRAYS:String = "$@B%8&WM#*oahkbdpqwmZO0QLCJUYXzcvunxrjft/|()1{}[]?-_+~<>i!lI;:,^`\'. ".to_string();

    let length = GRAYS.len() - 1;
    let val = (*value) as usize;
    let ceil = ((length * val) / 255) as f64;

    let index = ceil.ceil() as usize;
    let result = GRAYS.as_bytes()[index] as char;
    result
}