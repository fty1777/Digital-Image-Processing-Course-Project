use image::{DynamicImage, GrayImage, Luma};
use rustfft::{num_complex::Complex, FftPlanner};

fn dft_2d_complex(
    data: &Vec<Vec<Complex<f64>>>,
    planner: &mut FftPlanner<f64>,
    inverse: bool,
) -> Vec<Vec<Complex<f64>>> {
    let height = data.len();
    let width = data[0].len();
    let mut transformed = vec![vec![Complex::new(0.0, 0.0); width]; height];

    // Row-wise (I)DFT
    for (i, row) in data.iter().enumerate() {
        let mut buffer = row.clone();
        let fft = if inverse {
            planner.plan_fft_inverse(width)
        } else {
            planner.plan_fft_forward(width)
        };
        fft.process(&mut buffer);
        if inverse {
            for val in buffer.iter_mut() {
                *val /= width as f64;
            }
        }
        transformed[i] = buffer;
    }

    // Column-wise (I)DFT
    for j in 0..width {
        let mut col: Vec<Complex<f64>> = transformed.iter().map(|row| row[j]).collect();
        let fft = if inverse {
            planner.plan_fft_inverse(height)
        } else {
            planner.plan_fft_forward(height)
        };
        fft.process(&mut col);
        if inverse {
            for val in col.iter_mut() {
                *val /= height as f64;
            }
        }
        for i in 0..height {
            transformed[i][j] = col[i];
        }
    }

    transformed
}

fn dft_2d_grayscale(gray_img: &GrayImage, inverse: bool, with_log: bool) -> GrayImage {
    let (width, height) = gray_img.dimensions();

    let mut data: Vec<Vec<Complex<f64>>> =
        vec![vec![Complex::new(0.0, 0.0); width as usize]; height as usize];
    for (x, y, pixel) in gray_img.enumerate_pixels() {
        data[y as usize][x as usize] = Complex::new(pixel.0[0] as f64, 0.0);
    }

    let mut planner = FftPlanner::new();
    let dft_result = dft_2d_complex(&data, &mut planner, inverse);

    let mut k = 0.0f64;
    let mut log_values: Vec<Vec<f64>> = vec![vec![0.0; width as usize]; height as usize];
    for y in 0..height as usize {
        for x in 0..width as usize {
            let log_value = if with_log {
                (1.0f64 + dft_result[y][x].norm()).ln()
            } else {
                dft_result[y][x].norm()
            };
            log_values[y][x] = log_value;
            if log_value > k {
                k = log_value;
            }
        }
    }
    let c = 255.0 / k;

    // D(x, y) = c log(1 + |F(x, y)|)
    let mut output_img = GrayImage::new(width, height);
    for y in 0..height as usize {
        for x in 0..width as usize {
            let pixel_value = (c * log_values[y][x]).clamp(0.0, 255.0) as u8;
            output_img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }

    output_img
}

fn shift_to_center_f64(data: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
    let height = data.len();
    let width = data[0].len();
    let mut shifted_data = vec![vec![0.0f64; width]; height];

    let half_width = width / 2;
    let half_height = height / 2;

    for y in 0..height {
        for x in 0..width {
            let new_x = (x + half_width) % width;
            let new_y = (y + half_height) % height;
            shifted_data[new_y][new_x] = data[y][x];
        }
    }

    shifted_data
}

fn apply_dft_2d(gray_img: &GrayImage) -> GrayImage {
    dft_2d_grayscale(gray_img, false, true)
}

fn apply_dft_2d_no_log(gray_img: &GrayImage) -> GrayImage {
    dft_2d_grayscale(gray_img, false, false)
}

fn apply_idft_2d(gray_img: &GrayImage) -> GrayImage {
    dft_2d_grayscale(gray_img, true, true)
}

fn apply_dft_idft_2d(gray_img: &GrayImage) -> GrayImage {
    let (width, height) = gray_img.dimensions();

    let mut data: Vec<Vec<Complex<f64>>> =
        vec![vec![Complex::new(0.0, 0.0); width as usize]; height as usize];
    for (x, y, pixel) in gray_img.enumerate_pixels() {
        data[y as usize][x as usize] = Complex::new(pixel.0[0] as f64, 0.0);
    }

    let mut planner = FftPlanner::new();
    let dft_result = dft_2d_complex(&data, &mut planner, false);
    let idft_result = dft_2d_complex(&dft_result, &mut planner, true);

    let mut output_img = GrayImage::new(width, height);
    for y in 0..height as usize {
        for x in 0..width as usize {
            let pixel_value = idft_result[y][x].re.clamp(0.0, 255.0) as u8;
            output_img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }

    output_img
}

fn apply_shift_to_center(img: &GrayImage) -> GrayImage {
    let (width, height) = img.dimensions();
    let mut shifted_img = GrayImage::new(width, height);

    let half_width = width / 2;
    let half_height = height / 2;

    for y in 0..height {
        for x in 0..width {
            let new_x = (x + half_width) % width;
            let new_y = (y + half_height) % height;
            let pixel = img.get_pixel(x, y);
            shifted_img.put_pixel(new_x, new_y, *pixel);
        }
    }

    shifted_img
}

fn apply_homomorphic_filtering(
    gray_img: &GrayImage,
    r_l: f64,
    r_h: f64,
    c: f64,
    d0: f64,
) -> GrayImage {
    let (width, height) = gray_img.dimensions();

    // 1. log
    let mut data: Vec<Vec<Complex<f64>>> =
        vec![vec![Complex::new(0.0, 0.0); width as usize]; height as usize];
    for (x, y, pixel) in gray_img.enumerate_pixels() {
        let log_val = (pixel.0[0] as f64 + 1.0).ln();
        data[y as usize][x as usize] = Complex::new(log_val, 0.0);
    }

    // 2. FFT
    let mut planner = FftPlanner::new();
    let mut fft_data = dft_2d_complex(&data, &mut planner, false);

    // 3. Filtering
    // 3.1. Create Gaussian filter
    let mut filter_shifted = vec![vec![0.0; width as usize]; height as usize];
    let mid_x = width as f64 / 2.0;
    let mid_y = height as f64 / 2.0;
    for i in 0..height {
        for j in 0..width {
            let distance_squared = ((i as f64 - mid_y).powi(2) + (j as f64 - mid_x).powi(2)).sqrt();
            filter_shifted[i as usize][j as usize] =
                (r_h - r_l) * (1.0 - (-c * (distance_squared / d0).powi(2)).exp()) + r_l;
        }
    }
    let filter = shift_to_center_f64(&filter_shifted);
    // 3.2. Apply the filter
    for i in 0..height {
        for j in 0..width {
            fft_data[i as usize][j as usize] *= filter[i as usize][j as usize];
        }
    }

    // 4. IFFT
    let ifft_data = dft_2d_complex(&fft_data, &mut planner, true);

    // 5. exp
    let mut exp_data = vec![vec![0.0; width as usize]; height as usize];
    for i in 0..height {
        for j in 0..width {
            exp_data[i as usize][j as usize] = ifft_data[i as usize][j as usize].exp().re - 1.0;
        }
    }

    // 6. scale to [0, 255]
    let min = exp_data
        .iter()
        .flatten()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let max = exp_data
        .iter()
        .flatten()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let range = max - min;

    let mut output_img = GrayImage::new(width, height);
    for (y, row) in exp_data.iter().enumerate() {
        for (x, &val) in row.iter().enumerate() {
            let pixel_value = ((val - min) / range * 255.0).round().clamp(0.0, 255.0) as u8;
            output_img.put_pixel(x as u32, y as u32, Luma([pixel_value]));
        }
    }

    output_img
}

pub fn dft(img: &DynamicImage) -> DynamicImage {
    let gray_img = img.to_luma8();
    let output_img = apply_shift_to_center(&apply_dft_2d(&gray_img));
    DynamicImage::ImageLuma8(output_img)
}

pub fn dft_non_shifted(img: &DynamicImage) -> DynamicImage {
    let gray_img = img.to_luma8();
    let output_img = apply_dft_2d(&gray_img);
    DynamicImage::ImageLuma8(output_img)
}

pub fn dft_no_log(img: &DynamicImage) -> DynamicImage {
    let gray_img = img.to_luma8();
    let output_img = apply_shift_to_center(&&apply_dft_2d_no_log(&gray_img));
    DynamicImage::ImageLuma8(output_img)
}

pub fn idft(img: &DynamicImage) -> DynamicImage {
    let gray_img = img.to_luma8();
    let output_img = apply_shift_to_center(&apply_idft_2d(&gray_img));
    DynamicImage::ImageLuma8(output_img)
}

pub fn idft_non_shifted(img: &DynamicImage) -> DynamicImage {
    let gray_img = img.to_luma8();
    let output_img = apply_idft_2d(&gray_img);
    DynamicImage::ImageLuma8(output_img)
}

pub fn shift_to_center(img: &DynamicImage) -> DynamicImage {
    let gray_img = img.to_luma8();
    let shifted_img = apply_shift_to_center(&gray_img);
    DynamicImage::ImageLuma8(shifted_img)
}

pub fn homomorphic(
    img: &DynamicImage,
    r_l: Option<f32>,
    r_h: Option<f32>,
    c: Option<f32>,
    d0: Option<f32>,
) -> DynamicImage {
    let gray_img = img.to_luma8();
    let output_img = apply_homomorphic_filtering(
        &gray_img,
        r_l.unwrap_or(0.3) as f64,
        r_h.unwrap_or(2.0) as f64,
        c.unwrap_or(2.0) as f64,
        d0.unwrap_or(10.0) as f64,
    );
    DynamicImage::ImageLuma8(output_img)
}

pub fn dft_idft(img: &DynamicImage) -> DynamicImage {
    let gray_img = img.to_luma8();
    let restored_img = apply_dft_idft_2d(&gray_img);
    DynamicImage::ImageLuma8(restored_img)
}
