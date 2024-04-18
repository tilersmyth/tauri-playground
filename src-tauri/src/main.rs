// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use serde::Serialize;
use serde_json::to_string;
use serialport::{available_ports, SerialPortInfo, SerialPortType, UsbPortInfo};
use std::sync::{Arc, Mutex};
use tauri::Manager;

// Arduino for testing
const TARGET_VENDOR: u16 = 9025;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let app_handle = app.handle();

            let port_info: Arc<Mutex<Option<SerialPortInfo>>> = Arc::new(Mutex::new(None));
            let reader_port = port_info.clone();
            tauri::async_runtime::spawn(async move { watch_ports(port_info) });

            tauri::async_runtime::spawn(async move {
                loop {
                    // let mut test = reader_port.lock().unwrap().clone();
                    app_handle
                        .emit_all(
                            "port_status",
                            to_string(&reader_port.lock().unwrap().clone()).unwrap(),
                        )
                        .unwrap();

                    // match test {
                    //     Some(port) => {
                    //         println!("PORT: {}", to_string(&port).unwrap());
                    //     }
                    //     None => {
                    //         println!("NO DEVICE")
                    //     }
                    // }

                    std::thread::sleep(std::time::Duration::from_secs(1));
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn watch_ports(port_info: Arc<Mutex<Option<SerialPortInfo>>>) {
    loop {
        match available_ports() {
            Ok(ports) => {
                let mut locked_port_info = port_info.lock().unwrap();
                for ref p in ports {
                    match &p.port_type {
                        SerialPortType::UsbPort(info) => {
                            match info.vid {
                                TARGET_VENDOR => {
                                    let port_info = p.clone();
                                    *locked_port_info = Some(port_info);
                                }
                                _ => {
                                    continue;
                                }
                            };
                        }
                        _ => {
                            *locked_port_info = None;
                            continue;
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
                eprintln!("Error listing serial ports");
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
