// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused)]

use rand::Rng;
use serde::Serialize;
use serde_json::to_string;
use serialport::{available_ports, Error, SerialPort, SerialPortInfo, SerialPortType, UsbPortInfo};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tauri::{
    api::path::local_data_dir,
    async_runtime::{Receiver, Sender},
    AppHandle, Manager,
};
use tokio::sync::mpsc;

// Arduino for testing
const TARGET_VENDOR: u16 = 9025;
const BAUD_RATE: u32 = 9600;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![])
        .setup(|app| {
            let app_handle = app.handle();

            let port_info = Arc::new(Mutex::new(None));
            let reader_port = port_info.clone();

            let (tx, mut rx) = mpsc::channel::<Option<SerialPortInfo>>(100);
            tauri::async_runtime::spawn(async move { watch_ports(&app_handle, port_info) });

            let read_port_app_handle = app.handle().clone();

            tauri::async_runtime::spawn(
                async move { read_port(&read_port_app_handle, reader_port) },
            );

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn watch_ports<R: tauri::Runtime>(
    manager: &impl Manager<R>,
    port_info: Arc<Mutex<Option<SerialPortInfo>>>,
) {
    let mut port_tracker: Option<SerialPortInfo> = None;
    loop {
        match available_ports() {
            Ok(ports) => {
                for ref p in ports {
                    match &p.port_type {
                        SerialPortType::UsbPort(info) => {
                            match info.vid {
                                TARGET_VENDOR => {
                                    let current_port_status = Some(p.clone());

                                    // we will only lock the arc if we need it
                                    if port_tracker != current_port_status {
                                        let mut locked_port_info = port_info.lock().unwrap();
                                        *locked_port_info = current_port_status;
                                    }

                                    port_tracker = Some(p.clone());
                                }
                                _ => {
                                    continue;
                                }
                            };
                        }
                        _ => {
                            if port_tracker != None {
                                let mut locked_port_info = port_info.lock().unwrap();
                                *locked_port_info = None;
                            }

                            port_tracker = None;
                            continue;
                        }
                    }
                }

                manager
                    .emit_all("port_status", to_string(&port_tracker.clone()).unwrap())
                    .unwrap();
            }
            Err(e) => {
                eprintln!("{:?}", e);
                eprintln!("Error listing serial ports");
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

fn read_port<R: tauri::Runtime>(
    manager: &impl Manager<R>,
    port_info: Arc<Mutex<Option<SerialPortInfo>>>,
) {
    let mut target_port: Option<Result<Box<dyn SerialPort>, Error>> = None;

    loop {
        if (target_port.is_none()) {
            println!("reader: we are locking...");
            let mut locked_port_info = port_info.lock().unwrap();
            match &mut *locked_port_info {
                Some(port) => {
                    target_port = Some(
                        serialport::new(port.port_name.clone(), BAUD_RATE)
                            .timeout(Duration::from_millis(10))
                            .open(),
                    );
                }
                _ => {
                    target_port = None;
                    std::thread::sleep(std::time::Duration::from_secs(2));
                    continue;
                }
            }
        }

        match &mut target_port {
            Some(port) => match port {
                Ok(p) => {
                    let fake_reading = rand::thread_rng().gen_range(0..100);
                    println!("Value: {}", fake_reading);

                    let mut serial_buf: Vec<u8> = vec![0; 1000];

                    match p.read(serial_buf.as_mut_slice()) {
                        Ok(t) => println!("reading..."),
                        Err(ref e) if e.kind() == io::ErrorKind::BrokenPipe => {
                            target_port = None;
                            std::thread::sleep(std::time::Duration::from_secs(1));
                        }
                        Err(e) => println!("read error: {}", e),
                    }

                    manager
                        .emit_all("port_reading", to_string(&fake_reading).unwrap())
                        .unwrap();
                }
                Err(e) => {
                    println!("Port connect error: {}", e);
                }
            },
            None => {
                println!("shouldn't really get here?");
            }
        }
    }
}
