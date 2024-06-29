use crate::transform::{
    binary_op_transform, border_transform, color_transform, fft_transform, filter_transform,
    geometic_transform,
};

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
pub fn transform_image(
    img: String, // base64
    transform: String,
    transform_arg: String,
) -> Result<String, String> {
    let img = image::load_from_memory(&STANDARD.decode(img).unwrap())
        .map_err(|e| format!("Failed to load image from base64: {}", e))?;
    let transformed_img = match transform.as_str() {
        "color/invert" => color_transform::invert(img),
        "color/exponential" => color_transform::exponential(img, transform_arg.parse::<f32>().ok()),
        "color/hist_equalize" => color_transform::hist_equalize(img),
        "color/to_gray" => color_transform::to_gray(img),
        "color/to_binary" => color_transform::to_binary(img, transform_arg.parse::<f32>().ok()),
        "geometric/translate" => {
            let args: Vec<&str> = transform_arg.split(',').map(|s| s.trim()).collect();
            if args.len() != 2 {
                return Err("Invalid arguments for translate".to_string());
            }
            geometic_transform::translate(
                img,
                args[0].parse::<i32>().ok(),
                args[1].parse::<i32>().ok(),
            )
        }
        "geometric/rotate" => geometic_transform::rotate(img, transform_arg.parse::<f32>().ok()),
        "geometric/resize" => {
            let args: Vec<&str> = transform_arg.split(',').map(|s| s.trim()).collect();
            if args.len() != 2 {
                return Err("Invalid arguments for resize".to_string());
            }
            geometic_transform::resize(
                img,
                args[0].parse::<u32>().ok(),
                args[1].parse::<u32>().ok(),
            )
        }
        "geometric/mirror" => geometic_transform::mirror(img, transform_arg.as_str()),
        "geometric/stretch" => {
            let args: Vec<&str> = transform_arg.split(',').map(|s| s.trim()).collect();
            if args.len() != 2 {
                return Err("Invalid arguments for stretch".to_string());
            }
            geometic_transform::stretch(
                img,
                args[0].parse::<f32>().ok(),
                args[1].parse::<f32>().ok(),
            )
        }
        _ => return Err("Invalid transform".to_string()),
    };

    let base64_img = encode_image_to_base64(transformed_img)?;
    Ok(base64_img)
}
