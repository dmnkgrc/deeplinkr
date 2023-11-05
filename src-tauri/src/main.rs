// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, process::Command};

use tauri::{CustomMenuItem, Manager, SystemTray, SystemTrayEvent, SystemTrayMenu};
use tauri_plugin_positioner::{Position, WindowExt};

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Device {
    name: String,
    udid: String,
    state: String,
    #[serde(rename(deserialize = "isAvailable"))]
    is_available: bool,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct Simulators {
    devices: HashMap<String, Vec<Device>>,
}

#[tauri::command]
fn get_simulators() -> Result<Simulators, String> {
    let output = Command::new("xcrun")
        .args(["simctl", "list", "devices", "-j"])
        // execute the command, wait for it to complete, then capture the output
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    let simulators: Simulators =
        serde_json::from_str(&stdout).expect("Failed to parse simulators json");
    return Ok(simulators);
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .invoke_handler(tauri::generate_handler![get_simulators])
        .on_system_tray_event(|app, event| {
            tauri_plugin_positioner::on_tray_event(app, &event);
            match event {
                SystemTrayEvent::LeftClick {
                    position: _,
                    size: _,
                    ..
                } => {
                    let window = app.get_window("main").unwrap();
                    // use TrayCenter as initial window position
                    let _ = window.move_window(Position::TrayCenter);
                    if window.is_visible().unwrap() {
                        window.hide().unwrap();
                    } else {
                        window.show().unwrap();
                        window.set_focus().unwrap();
                    }
                }
                SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                    "quit" => {
                        std::process::exit(0);
                    }
                    _ => {}
                },
                _ => {}
            }
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::Focused(is_focused) => {
                if !is_focused {
                    event.window().hide().unwrap();
                }
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
