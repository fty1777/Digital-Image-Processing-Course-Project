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
            println!("menu event: {:?}", event.menu_item_id())
        })
        .invoke_handler(tauri::generate_handler![
            folder::read_folder,
            image::open_image,
            image::transform_image,
            image::save_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
