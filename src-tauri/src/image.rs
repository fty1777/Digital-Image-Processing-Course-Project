use crate::transform::{binary_op, color, fft, filter, fourier_desc, geometric};

use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat};
use std::io::{BufWriter, Cursor};

pub fn read_image(path: String) -> Result<DynamicImage, String> {
    let img = ImageReader::open(&path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .with_guessed_format()
        .map_err(|e| format!("Failed to guess image format: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;
    Ok(img)
}

pub fn encode_image_to_base64(img: DynamicImage) -> Result<String, String> {
    let mut buf = Vec::new();
    {
        let mut writer = BufWriter::new(Cursor::new(&mut buf));
        img.write_to(&mut writer, ImageFormat::Bmp)
            .map_err(|e| format!("Failed to write image to buffer: {}", e))?;
    }
    let base64_img = STANDARD.encode(&buf);
    Ok(base64_img)
}

#[tauri::command]
pub fn open_image(path: String) -> Result<String, String> {
    // Always open in RGB8 format
    let img = DynamicImage::ImageRgb8(read_image(path)?.to_rgb8());
    let base64_img = encode_image_to_base64(img)?;
    Ok(base64_img)
}

#[tauri::command]
pub fn save_image(path: String, img: String) -> Result<(), String> {
    let img = image::load_from_memory(&STANDARD.decode(img).unwrap())
        .map_err(|e| format!("Failed to load image from base64: {}", e))?;
    if path.ends_with(".jpg") || path.ends_with(".jpeg") {
        img.save_with_format(&path, ImageFormat::Jpeg)
            .map_err(|e| format!("Failed to save image: {}", e))?;
    } else if path.ends_with(".png") {
        img.save_with_format(&path, ImageFormat::Png)
            .map_err(|e| format!("Failed to save image: {}", e))?;
    } else if path.ends_with(".bmp") {
        img.save_with_format(&path, ImageFormat::Bmp)
            .map_err(|e| format!("Failed to save image: {}", e))?;
    } else {
        return Err("Unsupported image format".to_string());
    }
    Ok(())
}

#[tauri::command]
pub fn transform_image(
    img: String,  // base64
    img2: String, // base64
    transform: String,
    transform_arg: String,
) -> Result<String, String> {
    let img = image::load_from_memory(&STANDARD.decode(img).unwrap())
        .map_err(|e| format!("Failed to load image from base64: {}", e))?;
    // load to img2 but use Option to handle the case where img2 is empty
    let img2 = if img2.is_empty() {
        None
    } else {
        Some(
            image::load_from_memory(&STANDARD.decode(img2).unwrap())
                .map_err(|e| format!("Failed to load image from base64: {}", e))?,
        )
    };
    let transformed_img = match transform.as_str() {
        "color/invert" => color::invert(img),
        "color/exponential" => color::exponential(img, transform_arg.parse::<f32>().ok()),
        "color/hist_equalize" => color::hist_equalize(img),
        "color/to_gray" => color::to_gray(img),
        "color/to_binary" => color::to_binary(img, transform_arg.parse::<f32>().ok()),
        "geometric/translate" => {
            let args: Vec<&str> = transform_arg.split(',').map(|s| s.trim()).collect();
            if args.len() != 2 {
                return Err("Invalid arguments for translate".to_string());
            }
            geometric::translate(
                img,
                args[0].parse::<i32>().ok(),
                args[1].parse::<i32>().ok(),
            )
        }
        "geometric/rotate" => geometric::rotate(img, transform_arg.parse::<f32>().ok()),
        "geometric/resize" => {
            let args: Vec<&str> = transform_arg.split(',').map(|s| s.trim()).collect();
            if args.len() != 2 {
                return Err("Invalid arguments for resize".to_string());
            }
            geometric::resize(
                img,
                args[0].parse::<u32>().ok(),
                args[1].parse::<u32>().ok(),
            )
        }
        "geometric/mirror" => geometric::mirror(img, transform_arg.as_str()),
        "geometric/stretch" => {
            let args: Vec<&str> = transform_arg.split(',').map(|s| s.trim()).collect();
            if args.len() != 2 {
                return Err("Invalid arguments for stretch".to_string());
            }
            geometric::stretch(
                img,
                args[0].parse::<f32>().ok(),
                args[1].parse::<f32>().ok(),
            )
        }
        "binary_op/add" => binary_op::binary_op(&img, &img2.unwrap(), "add"),
        "binary_op/sub" => binary_op::binary_op(&img, &img2.unwrap(), "sub"),
        "binary_op/mul" => binary_op::binary_op(&img, &img2.unwrap(), "mul"),
        "binary_op/div" => binary_op::binary_op(&img, &img2.unwrap(), "div"),
        "filter/mean" => filter::mean(&img, transform_arg.parse::<u32>().unwrap_or(3)),
        "filter/median" => filter::median(&img, transform_arg.parse::<u32>().unwrap_or(3)),
        "filter/gaussian" => {
            let args: Vec<&str> = transform_arg.split(',').map(|s| s.trim()).collect();
            if args.len() != 2 {
                return Err("Invalid arguments for gaussian".to_string());
            }
            filter::gaussian(
                &img,
                args[0].parse::<usize>().unwrap_or(3),
                args[1].parse::<f32>().unwrap_or(1.),
            )
        }
        "filter/sobel" => filter::sobel(&img, transform_arg.as_str()),
        "filter/laplacian" => filter::laplacian(&img, transform_arg.parse::<u8>().unwrap_or(8)),
        "filter/prewitt" => filter::prewitt(&img, transform_arg.as_str()),
        "filter/roberts" => filter::roberts(&img, transform_arg.as_str()),
        "filter/sobel_sharpen" => filter::sobel_sharpen(&img, transform_arg.as_str()),
        "filter/laplacian_sharpen" => {
            filter::laplacian_sharpen(&img, transform_arg.parse::<u8>().unwrap_or(8))
        }
        "filter/prewitt_sharpen" => filter::prewitt_sharpen(&img, transform_arg.as_str()),
        "filter/roberts_sharpen" => filter::roberts_sharpen(&img, transform_arg.as_str()),
        "fft/dft" => fft::dft(&img),
        "fft/dft_non_shifted_no_log" => fft::dft_non_shifted_no_log(&img),
        "fft/dft_non_shifted" => fft::dft_non_shifted(&img),
        "fft/dft_no_log" => fft::dft_no_log(&img),
        "fft/idft" => fft::idft(&img),
        "fft/idft_non_shifted" => fft::idft_non_shifted(&img),
        "fft/shift_to_center" => fft::shift_to_center(&img),
        "fft/homomorphic" => {
            let args: Vec<&str> = transform_arg.split(',').map(|s| s.trim()).collect();
            if args.len() != 4 {
                return Err("Invalid arguments for homomorphic filtering".to_string());
            }
            fft::homomorphic(
                &img,
                args[0].parse::<f32>().ok(),
                args[1].parse::<f32>().ok(),
                args[2].parse::<f32>().ok(),
                args[3].parse::<f32>().ok(),
            )
        }
        "fft/dft_idft" => fft::dft_idft(&img),
        "fourier_desc" => {
            fourier_desc::reconstruct(&img, transform_arg.parse::<usize>().unwrap_or(64))
        }

        _ => return Err("Invalid transform".to_string()),
    };

    let base64_img = encode_image_to_base64(transformed_img)?;
    Ok(base64_img)
}
