use crate::transform::color::invert;
use image::{DynamicImage, GrayImage};
use imageproc::contours::find_contours;
use imageproc::drawing::draw_filled_rect_mut;
use imageproc::rect::Rect;
use num_complex::Complex;
use rustfft::FftPlanner;

fn fft(points: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let len = points.len();
    let mut fft_planner = FftPlanner::<f64>::new();
    let fft = fft_planner.plan_fft_forward(len);
    let mut fd = points.to_owned();
    fft.process(&mut fd);
    fd
}

fn ifft(fd: &Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let len = fd.len();
    let mut ifft_planner = FftPlanner::<f64>::new();
    let ifft = ifft_planner.plan_fft_inverse(len);
    let mut points = fd.to_owned();
    ifft.process(&mut points);
    points
}

fn truncate_fft(fft: &Vec<Complex<f64>>, n: usize) -> Vec<Complex<f64>> {
    // Truncate directly without explicit shifting (directly fill without shifting)
    let mut truncated_fft = fft.to_owned();
    let l = n / 2;
    let r = l + fft.len() - n;
    truncated_fft[l..r].fill(Complex::new(0.0, 0.0));

    truncated_fft
}

pub fn reconstruct_impl(img: &GrayImage, nterms: usize) -> GrayImage {
    let binary_img = imageproc::contrast::threshold(&img, 128);

    // Find contours
    let contours = find_contours::<i32>(&binary_img);
    // get the contour with the most points
    let contour = contours.iter().max_by_key(|c| c.points.len()).unwrap();
    // generate a Complex<f64> vector with real parts as xs and imag parts as ys of points
    let points: Vec<Complex<f64>> = contour
        .points
        .iter()
        .map(|p| num_complex::Complex::new(p.x as f64, p.y as f64))
        .collect();
    // get the max x and y
    let max_x = points
        .iter()
        .map(|p| p.re)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_y = points
        .iter()
        .map(|p| p.im)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_x = points
        .iter()
        .map(|p| p.re)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_y = points
        .iter()
        .map(|p| p.im)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let x_scale = max_x - min_x;
    let y_scale = max_y - min_y;
    let max_scale = x_scale.max(y_scale);

    // fft
    let fd = fft(&points);

    let truncated_fd = truncate_fft(&fd, nterms);

    let points_reconstructed = ifft(&truncated_fd);

    // scale the reconstructed points from (p_min, p_max) to 0-max(max_x, max_y)
    let min_x_reconstructed = points_reconstructed
        .iter()
        .map(|p| p.re)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_y_reconstructed = points_reconstructed
        .iter()
        .map(|p| p.im)
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_x_reconstructed = points_reconstructed
        .iter()
        .map(|p| p.re)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_y_reconstructed = points_reconstructed
        .iter()
        .map(|p| p.im)
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let x_scale_reconstructed = max_x_reconstructed - min_x_reconstructed;
    let y_scale_reconstructed = max_y_reconstructed - min_y_reconstructed;
    let max_scale_reconstructed = x_scale_reconstructed.max(y_scale_reconstructed);

    let points_reconstructed: Vec<Complex<f64>> = points_reconstructed
        .iter()
        .map(|p| {
            Complex::new(
                (p.re - min_x_reconstructed) / max_scale_reconstructed * max_scale + min_x,
                (p.im - min_y_reconstructed) / max_scale_reconstructed * max_scale + min_y,
            )
        })
        .collect();

    // draw the reconstructed contour
    let mut img_reconstructed = GrayImage::new(img.width(), img.height());
    for p in &points_reconstructed {
        let x = p.re.round() as i32;
        let y = p.im.round() as i32;
        draw_filled_rect_mut(
            &mut img_reconstructed,
            Rect::at(x - 1, y - 1).of_size(3, 3),
            image::Luma([255]),
        );
    }

    img_reconstructed
}

pub fn reconstruct(img: &DynamicImage, nterms: usize) -> DynamicImage {
    let img = img.to_luma8();
    let reconstructed = DynamicImage::ImageLuma8(reconstruct_impl(&img, nterms));
    invert(reconstructed)
}
