#[cfg(windows)]
mod clipboard;
#[cfg(windows)]
mod daemon;
#[cfg(windows)]
mod hook;
mod layout;
mod platform;
mod settings;
mod transform;
mod tui;

use crate::settings::Settings;
use log::info;
use std::env;

#[cfg(target_os = "linux")]
use crate::transform::Transform;

#[tokio::main]
async fn main() {
    env_logger::init();
    let args: Vec<String> = env::args().collect();

    let mut settings = Settings::load();
    info!(
        "Settings loaded from: {}",
        Settings::config_path().display()
    );

    if args.contains(&"--settings".to_string()) || args.contains(&"-s".to_string()) {
        if let Err(e) = tui::run_settings_ui(&mut settings) {
            eprintln!("TUI error: {}", e);
        }
        return;
    }

    #[cfg(windows)]
    platform::run_daemon(settings).await;

    #[cfg(target_os = "linux")]
    if let Some(transform) = args.windows(2).find(|a| a[0] == "--transform") {
        match parse_transform(&transform[1]) {
            Some(transform) => {
                if let Err(e) = platform::transform_selection(
                    transform,
                    settings.clipboard_delay_ms,
                    settings.restore_clipboard,
                ) {
                    eprintln!("txtfrmt: {e}");
                    std::process::exit(1);
                }
            }
            None => {
                eprintln!("Unknown transform: {}", transform[1]);
                std::process::exit(2);
            }
        }
    } else {
        eprintln!(
            "Usage: txtfrmt --transform <ToRussian|ToEnglish|ToggleLayout|ToggleCase|ToUpper|ToLower|TitleCase|CleanWhitespace|Reverse|ToCamelCase|ToSnakeCase>\n       txtfrmt --settings"
        );
        std::process::exit(2);
    }
}

#[cfg(target_os = "linux")]
fn parse_transform(s: &str) -> Option<Transform> {
    Some(match s {
        "ToRussian" => Transform::ToRussian,
        "ToEnglish" => Transform::ToEnglish,
        "ToggleLayout" => Transform::ToggleLayout,
        "ToggleCase" => Transform::ToggleCase,
        "ToUpper" => Transform::ToUpper,
        "ToLower" => Transform::ToLower,
        "TitleCase" => Transform::TitleCase,
        "CleanWhitespace" => Transform::CleanWhitespace,
        "Reverse" => Transform::Reverse,
        "ToCamelCase" => Transform::ToCamelCase,
        "ToSnakeCase" => Transform::ToSnakeCase,
        _ => return None,
    })
}
