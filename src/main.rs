mod clipboard;
mod daemon;
mod hook;
mod layout;
mod settings;
mod transform;
mod tui;

use crate::clipboard::ClipboardManager;
use crate::daemon::DaemonEvent;
use crate::settings::Settings;
use log::{error, info};
use std::env;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let mut settings = Settings::load();
    info!("Settings loaded from: {}", Settings::config_path().display());

    if args.contains(&"--settings".to_string()) || args.contains(&"-s".to_string()) {
        if let Err(e) = tui::run_settings_ui(&mut settings) {
            eprintln!("TUI error: {}", e);
        }
        return;
    }

    run_daemon(settings).await;
}

fn hide_console() {
    unsafe {
        let hwnd = winapi::um::wincon::GetConsoleWindow();
        if !hwnd.is_null() {
            winapi::um::winuser::ShowWindow(hwnd, winapi::um::winuser::SW_HIDE);
        }
    }
}

async fn run_daemon(settings: Settings) {
    hide_console();
    info!("Starting textfmt daemon...");
    for hk in &settings.hotkeys {
        println!("  {}+{} {}", hk.modifiers.join("+"), hk.key, hk.transform.name());
    }

    let (tx, mut rx) = mpsc::channel::<DaemonEvent>(32);
    let settings_clone = settings.clone();

    let daemon_handle = tokio::spawn(async move {
        daemon::Daemon::run(&settings_clone, tx).await;
    });

    let mut clipboard_manager = match ClipboardManager::new() {
        Ok(cm) => cm,
        Err(e) => {
            error!("Failed to initialize clipboard: {}", e);
            return;
        }
    };

    while let Some(event) = rx.recv().await {
        match event {
            DaemonEvent::Transform(transform) => {
                info!("Applying transform: {:?}", transform);
                if let Err(e) = clipboard_manager.transform_selection(
                    |text| transform.apply(text),
                    settings.restore_clipboard,
                    settings.clipboard_delay_ms,
                ) {
                    error!("Transform failed: {}", e);
                }
            }
            DaemonEvent::OpenSettings => {
                info!("Opening settings UI...");
                let mut s = settings.clone();
                if let Err(e) = tui::run_settings_ui(&mut s) {
                    error!("Settings UI error: {}", e);
                }
            }
        }
    }

    let _ = daemon_handle.await;
    info!("textfmt daemon stopped");
}
