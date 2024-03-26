// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    Manager,
};

#[tauri::command(rename_all = "snake_case")]
fn update_system_tray(handle: tauri::AppHandle, update_count: i32) {
    println!("Updating system tray count to: {}", update_count);

    let tray = handle.tray_by_id("main-tray").unwrap();
    let toggle = MenuItemBuilder::with_id("update_count", update_count.to_string())
        .build(handle.app_handle())
        .unwrap();
    let menu = MenuBuilder::new(handle.app_handle())
        .items(&[&toggle])
        .build()
        .unwrap();
    tray.set_menu(Some(menu)).unwrap();
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![update_system_tray])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
