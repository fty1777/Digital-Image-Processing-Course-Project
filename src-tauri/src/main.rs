// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod folder;
mod image;
mod menu;
mod transform;

#[derive(Clone, serde::Serialize)]
struct MenuEventPayload {
    menu_item: String,
}

fn main() {
    tauri::Builder::default()
        .menu(menu::create_menu().unwrap())
        .on_menu_event(|event| {
            event
                .window()
                .emit(
                    "menu_event",
                    MenuEventPayload {
                        menu_item: event.menu_item_id().to_string(),
                    },
                )
                .unwrap();
            match event.menu_item_id() {
                "color/to_binary" => {
                    println!("color/to_binary");
                }
                "color/to_gray" => {
                    println!("color/to_gray");
                }
                "color/invert" => {
                    println!("color/invert");
                }
                "color/exponential" => {
                    println!("color/exponential");
                }
                "color/hist_equalize" => {
                    println!("color/hist_equalize");
                }
                "geometric/translate" => {
                    println!("geometric/translate");
                }
                "geometric/rotate" => {
                    println!("geometric/rotate");
                }
                "geometric/resize" => {
                    println!("geometric/resize");
                }
                "geometric/mirror" => {
                    println!("geometric/mirror");
                }
                "geometric/stretch" => {
                    println!("geometric/stretch");
                }
                "binary_op/add" => {
                    println!("binary_op/add");
                }
                "binary_op/sub" => {
                    println!("binary_op/sub");
                }
                "binary_op/mul" => {
                    println!("binary_op/mul");
                }
                "binary_op/div" => {
                    println!("binary_op/div");
                }
                "interpolate/nearest" => {
                    println!("interpolate/nearest");
                }
                "interpolate/bilinear" => {
                    println!("interpolate/bilinear");
                }
                "interpolate/bicubic" => {
                    println!("interpolate/bicubic");
                }
                "filter/mean" => {
                    println!("filter/mean");
                }
                "filter/median" => {
                    println!("filter/median");
                }
                "filter/gaussian" => {
                    println!("filter/gaussian");
                }
                "filter/homomorphic" => {
                    println!("filter/homomorphic");
                }
                "filter/sobel_sharpen" => {
                    println!("filter/sobel_sharpen");
                }
                "filter/laplacian_sharpen" => {
                    println!("filter/laplacian_sharpen");
                }
                "filter/prewitt_sharpen" => {
                    println!("filter/prewitt_sharpen");
                }
                "filter/roberts_sharpen" => {
                    println!("filter/roberts_sharpen");
                }
                "border/sobel" => {
                    println!("border/sobel");
                }
                "border/laplacian" => {
                    println!("border/laplacian");
                }
                "border/prewitt" => {
                    println!("border/prewitt");
                }
                "border/roberts" => {
                    println!("border/roberts");
                }
                "fft/dft" => {
                    println!("fft/dft");
                }
                "fft/idft" => {
                    println!("fft/idft");
                }
                "fft/dft_non_shifted" => {
                    println!("fft/dft_non_shifted");
                }
                "fft/idft_non_shifted" => {
                    println!("fft/idft_non_shifted");
                }
                "fft/shift_to_center" => {
                    println!("fft/shift_to_center");
                }
                "fft/log_enhance" => {
                    println!("fft/log_enhance");
                }
                "fourier_desc" => {
                    println!("fourier_desc");
                }
                _ => {}
            }
        })
        .invoke_handler(tauri::generate_handler![
            folder::read_folder,
            image::open_image,
            image::transform_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
