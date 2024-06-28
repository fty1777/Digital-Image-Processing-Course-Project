use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use image::io::Reader as ImageReader;
use image::ImageFormat;
use std::io::{BufWriter, Cursor};
use tauri::command;

#[command]
pub fn open_image(path: String) -> Result<String, String> {
    // Read the BMP image
    let img = ImageReader::open(&path)
        .map_err(|e| format!("Failed to open image: {}", e))?
        .with_guessed_format()
        .map_err(|e| format!("Failed to guess image format: {}", e))?
        .decode()
        .map_err(|e| format!("Failed to decode image: {}", e))?;

    // Convert the image to a vector of bytes using BufWriter for better performance
    let mut buf = Vec::new();
    {
        let mut writer = BufWriter::new(Cursor::new(&mut buf));
        img.write_to(&mut writer, ImageFormat::Bmp)
            .map_err(|e| format!("Failed to write image to buffer: {}", e))?;
    }

    // Encode the bytes to a base64 string using the new method
    let base64_img = STANDARD.encode(&buf);

    // Return the base64 string
    Ok(base64_img)
}
