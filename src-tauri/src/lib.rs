use portable_pty::{CommandBuilder, NativePtySystem, PtySize, PtySystem};
use std::io::{Read, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{Emitter, State};

/// **EXPERIMENTAL**: WSL session management
/// 
/// This is an experimental implementation for managing WSL terminal sessions.
/// The API and behavior may change in future versions.
/// 
/// # Warning
/// This feature is unstable and may not work as expected in all environments.
#[doc(alias = "experimental")]
struct WslSession {
    writer: Arc<Mutex<Box<dyn Write + Send>>>,
}

/// **EXPERIMENTAL**: Start a WSL instance
/// 
/// # Warning
/// This command is experimental and may change or be removed in future versions.
#[tauri::command]
fn start_wsl(
    app: tauri::AppHandle,
    distro: String,
    state: State<'_, Arc<Mutex<Option<WslSession>>>>,
) -> Result<(), String> {
    let pty_system = NativePtySystem::default();

    let pair = pty_system
        .openpty(PtySize {
            rows: 30,
            cols: 100,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| e.to_string())?;

    let mut cmd = CommandBuilder::new("wsl");
    cmd.arg("-d");
    cmd.arg(distro);

    let mut child = pair.slave.spawn_command(cmd).map_err(|e| e.to_string())?;

    let writer = Arc::new(Mutex::new(pair.master.take_writer().unwrap()));

    let session = WslSession { writer };
    *state.lock().unwrap() = Some(session);

    let mut reader = pair.master.try_clone_reader().unwrap();

    thread::spawn(move || {
        let mut buf = [0u8; 4096];
        while let Ok(n) = reader.read(&mut buf) {
            if n == 0 {
                break;
            }
            let output = String::from_utf8_lossy(&buf[..n]).to_string();
            let _ = app.emit("wsl-output", output);
        }
    });

    // process to background
    thread::spawn(move || {
        let _ = child.wait();
    });

    Ok(())
}

/// **EXPERIMENTAL**: Write data to WSL terminal
/// 
/// # Warning
/// This command is experimental and may change or be removed in future versions.
#[tauri::command]
fn write_to_wsl(
    data: String,
    state: State<'_, Arc<Mutex<Option<WslSession>>>>,
) -> Result<(), String> {
    if let Some(session) = &*state.lock().unwrap() {
        let mut writer = session.writer.lock().unwrap();
        writer
            .write_all(data.as_bytes())
            .map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(Mutex::new(None::<WslSession>)))
        .invoke_handler(tauri::generate_handler![start_wsl, write_to_wsl])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
