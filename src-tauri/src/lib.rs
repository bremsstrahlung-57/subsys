use std::process::Command;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_distros() {
    let output = Command::new("wsl")
        .arg("--list")
        .arg("--all")
        .arg("--quiet")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8(output.stdout);
       
    match stdout {
        Ok(s) => {
            for line in s.lines() {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("Failed to decode wsl output as UTF-8: {}", e);
        }
    }
    
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_distros])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
