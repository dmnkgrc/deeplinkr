// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{collections::HashMap, process::Command};
use std::{thread, time};

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
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct SimulatorDeepLink {
    udid: String,
    deep_link: String,
    state: String,
}

#[tauri::command]
fn open_deeplink_in_simulator(link: SimulatorDeepLink) -> Result<(), String> {
    if link.state != "Booted" {
        Command::new("xcrun")
            .args(["simctl", "boot", &link.udid])
            .output()
            .unwrap();
        loop {
            let output = Command::new("xcrun")
                .args(["simctl", "list", "devices", &link.udid, "-j"])
                // execute the command, wait for it to complete, then capture the output
                .output()
                .unwrap();
            let stdout = String::from_utf8(output.stdout).unwrap();

            let simulators: Simulators =
                serde_json::from_str(&stdout).expect("Failed to parse simulators json");
            let mut booted = false;
            for val in simulators.devices.values() {
                for sim in val {
                    if sim.state == "Booted" {
                        booted = true;
                        break;
                    }
                }
                if booted {
                    break;
                }
            }
            if booted {
                break;
            }

            thread::sleep(time::Duration::from_millis(500));
        }
    }
    Command::new("xcrun")
        .args(["simctl", "openurl", "booted", &link.deep_link])
        .output()
        .unwrap();
    Command::new("open")
        .args(["/Applications/Xcode.app/Contents/Developer/Applications/Simulator.app/"])
        .output()
        .unwrap();
    return Ok(());
}

fn main() {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let system_tray_menu = SystemTrayMenu::new().add_item(quit);
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .system_tray(SystemTray::new().with_menu(system_tray_menu))
        .invoke_handler(tauri::generate_handler![
            get_simulators,
            open_deeplink_in_simulator
        ])
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
