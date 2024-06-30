use image::{DynamicImage, GrayImage, ImageBuffer, Luma};
use std::f32::consts::PI;

use crate::transform::binary_op::binary_op;
use crate::transform::utils::{apply_kernel_i32_gray, get_pixel_grayscale};

fn mean_filter(img: &GrayImage, kernel_size: u32) -> GrayImage {
    let (width, height) = img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    let k = kernel_size as i32;
    let offset = k / 2;

    for x in 0..width {
        for y in 0..height {
            let mut sum = 0;
            let mut count = 0;
            for dx in -offset..=offset {
                for dy in -offset..=offset {
                    let px = x as i32 + dx;
                    let py = y as i32 + dy;
                    sum += get_pixel_grayscale(img, px, py) as u32;
                    count += 1;
                }
            }
            let mean = (sum / count) as u8;
            result.put_pixel(x, y, Luma([mean]));
        }
    }
    result
}

fn median_filter(img: &GrayImage, kernel_size: u32) -> GrayImage {
    let (width, height) = img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    let k = kernel_size as i32;
    let offset = k / 2;
    let mut window = vec![];

    for x in 0..width {
        for y in 0..height {
            window.clear();
            for dx in -offset..=offset {
                for dy in -offset..=offset {
                    let px = x as i32 + dx;
                    let py = y as i32 + dy;
                    window.push(get_pixel_grayscale(img, px, py));
                }
            }
            window.sort_unstable();
            let median = window[window.len() / 2];
            result.put_pixel(x, y, Luma([median]));
        }
    }
    result
}

fn generate_gaussian_kernel(size: usize, sigma: f32) -> Vec<Vec<f32>> {
    let mut kernel = vec![vec![0.0; size]; size];
    let k = (size - 1) / 2;
    let sigma_sq = sigma * sigma;
    let mut sum = 0.0;

    for i in 0..size {
        for j in 0..size {
            let x = i as i32 - k as i32;
            let y = j as i32 - k as i32;
            let exp_part = -(x.pow(2) + y.pow(2)) as f32 / (2.0 * sigma_sq);
            kernel[i][j] = (1.0 / (2.0 * PI * sigma_sq)) * f32::exp(exp_part);
            sum += kernel[i][j];
        }
    }

    // Normalize
    for i in 0..size {
        for j in 0..size {
            kernel[i][j] /= sum;
        }
    }

    kernel
}

fn gaussian_filter(img: &GrayImage, kernel: &[Vec<f32>]) -> GrayImage {
    let (width, height) = img.dimensions();
    let mut result = ImageBuffer::new(width, height);
    let k = kernel.len() / 2;

    for x in 0..width {
        for y in 0..height {
            let mut sum = 0.0;
            for i in 0..kernel.len() {
                for j in 0..kernel.len() {
                    let dx = i as i32 - k as i32;
                    let dy = j as i32 - k as i32;
                    let pixel = get_pixel_grayscale(img, x as i32 + dx, y as i32 + dy);
                    sum += pixel as f32 * kernel[i][j];
                }
            }
            result.put_pixel(x, y, Luma([sum.clamp(0.0, 255.0) as u8]));
        }
    }
    result
}

pub fn mean(img: &DynamicImage, kernel_size: u32) -> DynamicImage {
    let gray_image = img.to_luma8();
    DynamicImage::ImageLuma8(mean_filter(&gray_image, kernel_size))
}

pub fn median(img: &DynamicImage, kernel_size: u32) -> DynamicImage {
    let gray_image = img.to_luma8();
    DynamicImage::ImageLuma8(median_filter(&gray_image, kernel_size))
}

pub fn gaussian(img: &DynamicImage, kernel_size: usize, sigma: f32) -> DynamicImage {
    let gray_image = img.to_luma8();
    let kernel = generate_gaussian_kernel(kernel_size, sigma);
    DynamicImage::ImageLuma8(gaussian_filter(&gray_image, &kernel))
}

pub fn sobel(img: &DynamicImage, direction: &str) -> DynamicImage {
    let img = img.to_luma8();
    let kernel = match direction {
        "h" => vec![vec![-1, 0, 1], vec![-2, 0, 2], vec![-1, 0, 1]],
        "v" => vec![vec![-1, -2, -1], vec![0, 0, 0], vec![1, 2, 1]],
        _ => vec![vec![0; 3]; 3], // Default to a null operation if no direction is specified
    };
    DynamicImage::ImageLuma8(apply_kernel_i32_gray(&img, &kernel))
}

pub fn laplacian(img: &DynamicImage, neighbors: u8) -> DynamicImage {
    let img = img.to_luma8();
    let kernel = match neighbors {
        4 => vec![vec![0, 1, 0], vec![1, -4, 1], vec![0, 1, 0]],
        8 => vec![vec![1, 1, 1], vec![1, -8, 1], vec![1, 1, 1]],
        _ => vec![vec![0; 3]; 3], // Default to 4 neighbors if an invalid value is given
    };
    DynamicImage::ImageLuma8(apply_kernel_i32_gray(&img, &kernel))
}

pub fn prewitt(img: &DynamicImage, direction: &str) -> DynamicImage {
    let img = img.to_luma8();
    let kernel = match direction {
        "h" => vec![vec![-1, 0, 1], vec![-1, 0, 1], vec![-1, 0, 1]],
        "v" => vec![vec![-1, -1, -1], vec![0, 0, 0], vec![1, 1, 1]],
        _ => vec![vec![0; 3]; 3], // Default to a null operation if no direction is specified
    };
    DynamicImage::ImageLuma8(apply_kernel_i32_gray(&img, &kernel))
}

pub fn roberts(img: &DynamicImage, direction: &str) -> DynamicImage {
    let img = img.to_luma8();
    let kernel = match direction {
        "\\" => vec![vec![-1, 0], vec![0, 1]],
        "/" => vec![vec![0, -1], vec![1, 0]],
        _ => vec![vec![0; 2]; 2], // Default to a null operation if no direction is specified
    };
    DynamicImage::ImageLuma8(apply_kernel_i32_gray(&img, &kernel))
}

pub fn sobel_sharpen(img: &DynamicImage, direction: &str) -> DynamicImage {
    let filtered_img = sobel(img, direction);
    binary_op(img, &filtered_img, "add")
}

pub fn laplacian_sharpen(img: &DynamicImage, neighbors: u8) -> DynamicImage {
    let filtered_img = laplacian(img, neighbors);
    binary_op(img, &filtered_img, "add")
}

pub fn prewitt_sharpen(img: &DynamicImage, direction: &str) -> DynamicImage {
    let filtered_img = prewitt(img, direction);
    binary_op(img, &filtered_img, "add")
}

pub fn roberts_sharpen(img: &DynamicImage, direction: &str) -> DynamicImage {
    let filtered_img = roberts(img, direction);
    binary_op(img, &filtered_img, "add")
}
