use image::open;
use rayon::prelude::*;
use std::time::Instant;

fn process_image(image_name: &&str) {
    let input_path = format!("input/{image_name}");
    let mut image = open(input_path).unwrap().into_rgb8();
    let (width, height) = image.dimensions();
    (0..width).for_each(|x| {
        (0..height).for_each(|y| {
            let pixel = image.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
            let average = data[0] / 3 + data[1] / 3 + data[2] / 3;
            *pixel = image::Rgb([average, average, average]);
        })
    });
    let output_path = format!("output-multi/{image_name}");
    image.save(output_path).unwrap();
}

fn multi_threaded() -> u128 {
    let now = Instant::now();
    ["1.webp", "2.webp", "3.webp"]
        .par_iter()
        .for_each(process_image);
    now.elapsed().as_millis()
}

fn single_threaded() -> u128 {
    let now = Instant::now();
    ["1.webp", "2.webp", "3.webp"]
        .iter()
        .for_each(process_image);
    now.elapsed().as_millis()
}

fn main() {
    let single_threaded_time: u128 = (0..10).map(|_| single_threaded()).sum::<u128>() / 10;
    println!("time for single threaded: {single_threaded_time}ms");
    let multi_threaded_time: u128 = (0..10).map(|_| multi_threaded()).sum::<u128>() / 10;
    println!("time for multi threaded: {multi_threaded_time}ms");
}
