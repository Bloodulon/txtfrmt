use crate::{
    clipboard::ClipboardManager,
    daemon::{Daemon, DaemonEvent},
    settings::Settings,
    transform::Transform,
    tui,
};
use log::{error, info};
use tokio::sync::mpsc;

fn hide_console() {
    unsafe {
        let hwnd = winapi::um::wincon::GetConsoleWindow();
        if !hwnd.is_null() {
            winapi::um::winuser::ShowWindow(hwnd, winapi::um::winuser::SW_HIDE);
        }
    }
}

pub async fn run_daemon(settings: Settings) {
    hide_console();
    info!("Starting textfmt daemon...");
    for hk in &settings.hotkeys {
        println!(
            "  {}+{} {}",
            hk.modifiers.join("+"),
            hk.key,
            hk.transform.name()
        );
    }

    let (tx, mut rx) = mpsc::channel::<DaemonEvent>(32);
    let settings_clone = settings.clone();

    let daemon_handle = tokio::spawn(async move {
        Daemon::run(&settings_clone, tx).await;
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
