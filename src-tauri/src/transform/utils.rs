use image::{GenericImageView, GrayImage, Luma, Rgb, RgbImage};

pub fn max_dimensions<I>(img1: &I, img2: &I) -> (u32, u32)
where
    I: GenericImageView,
{
    let width = img1.width().max(img2.width());
    let height = img1.height().max(img2.height());
    (width, height)
}

pub fn get_pixel_rgb(img: &RgbImage, x: u32, y: u32) -> Rgb<u8> {
    if x < img.width() && y < img.height() {
        *img.get_pixel(x, y)
    } else {
        Rgb([0, 0, 0])
    }
}

pub fn get_pixel_gray(img: &GrayImage, x: u32, y: u32) -> Luma<u8> {
    if x < img.width() && y < img.height() {
        *img.get_pixel(x, y)
    } else {
        Luma([0])
    }
}

pub fn get_pixel_grayscale_safer(img: &GrayImage, x: i32, y: i32) -> u8 {
    if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
        img.get_pixel(x as u32, y as u32).0[0]
    } else {
        0
    }
}
