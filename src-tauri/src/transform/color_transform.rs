use image::imageops;
use image::{DynamicImage, GrayImage, Luma, RgbImage};
use imageproc;

pub fn to_gray(img: DynamicImage) -> DynamicImage {
    DynamicImage::ImageLuma8(img.to_luma8())
}

pub fn to_binary(img: DynamicImage, threshold: Option<f32>) -> DynamicImage {
    let img = img.to_luma8();
    let threshold = threshold.unwrap_or(0.5);
    let (width, height) = img.dimensions();
    let mut binary_img = GrayImage::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let new_pixel = if (pixel[0] as f32 / 255.) > threshold {
            Luma([255u8])
        } else {
            Luma([0u8])
        };
        binary_img.put_pixel(x, y, new_pixel);
    }

    DynamicImage::ImageLuma8(binary_img)
}

pub fn invert(img: DynamicImage) -> DynamicImage {
    match img {
        DynamicImage::ImageLuma8(img) => invert_gray(DynamicImage::ImageLuma8(img)),
        DynamicImage::ImageRgb8(img) => invert_rgb(DynamicImage::ImageRgb8(img)),
        _ => invert_rgb(img),
    }
}

fn invert_rgb(img: DynamicImage) -> DynamicImage {
    let mut img = img.to_rgb8();
    for pixel in img.pixels_mut() {
        pixel.0.iter_mut().for_each(|p| *p = 255 - *p);
    }
    DynamicImage::ImageRgb8(img)
}

fn invert_gray(img: DynamicImage) -> DynamicImage {
    let mut img = img.to_luma8();
    for pixel in img.pixels_mut() {
        pixel[0] = 255 - pixel[0];
    }
    DynamicImage::ImageLuma8(img)
}

pub fn exponential(img: DynamicImage, exponent: Option<f32>) -> DynamicImage {
    let exponent = exponent.unwrap_or(1.0);
    match img {
        DynamicImage::ImageLuma8(img) => exponential_gray(DynamicImage::ImageLuma8(img), exponent),
        DynamicImage::ImageRgb8(img) => exponential_rgb(DynamicImage::ImageRgb8(img), exponent),
        _ => exponential_rgb(img, exponent),
    }
}

fn exponential_rgb(img: DynamicImage, exponent: f32) -> DynamicImage {
    println!("Exponentiating image, exponent: {}", exponent);
    let img = img.to_rgb8();
    let (width, height) = img.dimensions();
    let mut exp_img = RgbImage::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let new_pixel = image::Rgb([
            (((pixel[0] as f32) / 255.).powf(exponent) * 255.) as u8,
            (((pixel[1] as f32) / 255.).powf(exponent) * 255.) as u8,
            (((pixel[2] as f32) / 255.).powf(exponent) * 255.) as u8,
        ]);
        exp_img.put_pixel(x, y, new_pixel);
    }

    DynamicImage::ImageRgb8(exp_img)
}

fn exponential_gray(img: DynamicImage, exponent: f32) -> DynamicImage {
    let img = img.to_luma8();
    let (width, height) = img.dimensions();
    let mut exp_img = GrayImage::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels() {
        let new_pixel = image::Luma([(pixel[0] as f32).powf(exponent) as u8]);
        exp_img.put_pixel(x, y, new_pixel);
    }

    DynamicImage::ImageLuma8(exp_img)
}

pub fn hist_equalize(img: DynamicImage) -> DynamicImage {
    // Histogram equalization is a bit more involved.
    // Using the `histogram` crate to simplify this:
    use imageproc::contrast::equalize_histogram;

    let img = img.to_luma8();
    let equalized_img = equalize_histogram(&img);
    DynamicImage::ImageLuma8(equalized_img)
}
