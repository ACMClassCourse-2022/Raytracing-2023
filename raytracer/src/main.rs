use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};

fn main() {
    let path = std::path::Path::new("output/book1/image1.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let width = 256;
    let height = 256;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };

    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);

            let r: f64 = (i as f64) / ((width - 1) as f64) * 255.999;
            let g: f64 = (j as f64) / ((height - 1) as f64) * 255.999;
            let b: f64 = 0.25 * 255.999;
            *pixel = image::Rgb([r as u8, g as u8, b as u8]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
