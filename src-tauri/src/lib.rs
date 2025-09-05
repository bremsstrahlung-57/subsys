use std::process::Command;
use encoding_rs::UTF_16LE;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_distros() -> Vec<String> {
    let output = Command::new("wsl")
        .arg("--list")
        .arg("--all")
        .arg("--quiet")
        .output()
        .expect("failed to execute process");

    let (cow, _, _) = UTF_16LE.decode(&output.stdout);
    let stdout = cow.to_string();

    let distros: Vec<String> = stdout
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();
    
    distros
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_distros])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
