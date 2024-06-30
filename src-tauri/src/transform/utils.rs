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

pub fn get_pixel_grayscale(img: &GrayImage, x: i32, y: i32) -> u8 {
    if x >= 0 && x < img.width() as i32 && y >= 0 && y < img.height() as i32 {
        img.get_pixel(x as u32, y as u32).0[0]
    } else {
        0
    }
}

pub fn apply_kernel_i32_gray(img: &GrayImage, kernel: &[Vec<i32>]) -> GrayImage {
    let (width, height) = img.dimensions();
    let k = kernel.len() as i32 / 2;
    let mut result = GrayImage::new(width, height);

    for x in 0..width {
        for y in 0..height {
            let mut sum = 0i32;
            for i in 0..kernel.len() {
                for j in 0..kernel.len() {
                    let dx = i as i32 - k;
                    let dy = j as i32 - k;
                    let px = x as i32 + dx;
                    let py = y as i32 + dy;

                    if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                        let val = img.get_pixel(px as u32, py as u32).0[0] as i32;
                        sum += val * kernel[i][j];
                    }
                }
            }
            let pixel_value = sum.clamp(0, 255) as u8;
            result.put_pixel(x, y, Luma([pixel_value]));
        }
    }
    result
}
