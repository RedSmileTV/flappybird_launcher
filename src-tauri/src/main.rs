// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use tauri::Window;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn launch(window: Window, version: String) {

    if !check_java() {
        Command::new("cmd")
            .arg("/c")
            .arg("start")
            .arg("https://www.java.com/download/")
            .output()
            .expect("Failed to open Java download page");
        return;
    }

    show_window(window.clone(), false);
    let launching = Command::new("javaw")
        .arg("-jar")
        .arg("FlappyBird-".to_owned() + &version + ".jar")
        .output();

    match launching {
        Ok(launching) => {
            // Check the exit status
            if launching.status.success() {
                println!("FlappyBird.jar launched successfully");

            } else {
                println!("Failed to launch FlappyBird.jar");
                download(version);
            }
            show_window(window, true);
        }
        Err(e) => {
            eprintln!("Failed to execute command: {}", e);
        }
    }
}

fn show_window(window: Window, show: bool) {
    if show {
        window.show().expect("Failed to show window");
    }
    else {
        window.hide().expect("Failed to hide window");
    }
}

fn check_java() -> bool {
    let output = Command::new("java")
        .arg("-version")
        .output();

    return match output {
        Ok(_) => {
            println!("Java is installed and available in the system.");
            true
        }
        Err(_) => {
            println!("Java is not installed or not available in the system.");
            false
        }
    }
}

fn download(version: String) {

    let url: String = "https://github.com/MCmoderSD/FlappyBird/releases/download/".to_owned() + &version + "/FlappyBird.jar";

    Command::new("cmd")
        .arg("/c")
        .arg("curl")
        .arg("-L")
        .arg(url)
        .arg("-o")
        .arg("FlappyBird-".to_owned() + &version +".jar")
        .output()
        .expect("Failed to download FlappyBird.jar");

    println!("FlappyBird.jar downloaded successfully");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![launch])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
