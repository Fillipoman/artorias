use image;
use image::Rgba;
use image::GenericImageView;
fn main() {
    let img = image::open("D:/Dropbox/Screenshots/Nausicca.png").unwrap();
    let mut red_total:u32 = 0;
    let mut green_total:u32 = 0;
    let mut blue_total:u32 = 0;
    for pixel in img.pixels() {
	let x = pixel.0;
	let y = pixel.1;
	let data = pixel.2;
	red_total += data[0] as u32;
	green_total += data[1] as u32;
	blue_total += data[2] as u32;
    }
    println!("dimensions {:?}", img.dimensions());
    println!("red {:?}", red_total);
    println!("green {:?}", green_total);
    println!("blue {:?}", blue_total);
    println!("{:?}", img.color());
    println!("Hello, new world!");
}
