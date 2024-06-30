use image::{DynamicImage, Rgb, RgbImage};

use crate::transform::utils::{get_pixel_rgb, max_dimensions};

fn add(img1: &RgbImage, img2: &RgbImage) -> RgbImage {
    let (width, height) = max_dimensions(img1, img2);
    let mut result = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let p1 = get_pixel_rgb(img1, x, y).0;
            let p2 = get_pixel_rgb(img2, x, y).0;
            result.put_pixel(
                x,
                y,
                Rgb([
                    (p1[0] as u16 + p2[0] as u16).min(255) as u8,
                    (p1[1] as u16 + p2[1] as u16).min(255) as u8,
                    (p1[2] as u16 + p2[2] as u16).min(255) as u8,
                ]),
            );
        }
    }
    result
}

fn sub(img1: &RgbImage, img2: &RgbImage) -> RgbImage {
    let (width, height) = max_dimensions(img1, img2);
    let mut result = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let p1 = get_pixel_rgb(img1, x, y).0;
            let p2 = get_pixel_rgb(img2, x, y).0;
            result.put_pixel(
                x,
                y,
                Rgb([
                    p1[0].saturating_sub(p2[0]),
                    p1[1].saturating_sub(p2[1]),
                    p1[2].saturating_sub(p2[2]),
                ]),
            );
        }
    }
    result
}

fn mul(img1: &RgbImage, img2: &RgbImage) -> RgbImage {
    let (width, height) = max_dimensions(img1, img2);
    let mut result = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let p1 = get_pixel_rgb(img1, x, y).0;
            let p2 = get_pixel_rgb(img2, x, y).0;
            result.put_pixel(
                x,
                y,
                Rgb([
                    ((p1[0] as u16 * p2[0] as u16) / 255).min(255) as u8,
                    ((p1[1] as u16 * p2[1] as u16) / 255).min(255) as u8,
                    ((p1[2] as u16 * p2[2] as u16) / 255).min(255) as u8,
                ]),
            );
        }
    }
    result
}

fn div(img1: &RgbImage, img2: &RgbImage) -> RgbImage {
    let (width, height) = max_dimensions(img1, img2);
    let mut result = RgbImage::new(width, height);
    for x in 0..width {
        for y in 0..height {
            let p1 = get_pixel_rgb(img1, x, y).0;
            let p2 = get_pixel_rgb(img2, x, y).0;
            result.put_pixel(
                x,
                y,
                Rgb([
                    if p2[0] != 0 {
                        (p1[0] as u16 * 255 / p2[0] as u16).min(255)
                    } else {
                        255
                    } as u8,
                    if p2[1] != 0 {
                        (p1[1] as u16 * 255 / p2[1] as u16).min(255)
                    } else {
                        255
                    } as u8,
                    if p2[2] != 0 {
                        (p1[2] as u16 * 255 / p2[2] as u16).min(255)
                    } else {
                        255
                    } as u8,
                ]),
            );
        }
    }
    result
}

pub fn binary_op(img1: &DynamicImage, img2: &DynamicImage, op: &str) -> DynamicImage {
    let img1 = img1.to_rgb8();
    let img2 = img2.to_rgb8();
    let result = match op {
        "add" => add(&img1, &img2),
        "sub" => sub(&img1, &img2),
        "mul" => mul(&img1, &img2),
        "div" => div(&img1, &img2),
        _ => img1,
    };
    DynamicImage::ImageRgb8(result)
}
