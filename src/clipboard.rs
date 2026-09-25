use arboard::Clipboard;
use enigo::{Direction, Enigo, Key, Keyboard, Settings as EnigoSettings};
use log::{info, warn};
use std::thread;
use std::time::Duration;
use std::time::SystemTime;

pub struct ClipboardManager {
    clipboard: Clipboard,
    enigo: Enigo,
}

impl ClipboardManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let clipboard = Clipboard::new()?;
        let enigo = Enigo::new(&EnigoSettings::default())?;
        info!("ClipboardManager initialized");
        Ok(Self { clipboard, enigo })
    }

    pub fn get_text(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        let text = self.clipboard.get_text()?;
        Ok(text.trim_start_matches('\u{FEFF}').to_string())
    }

    fn send_ctrl_c(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.enigo.key(Key::Control, Direction::Press)?;
        thread::sleep(Duration::from_millis(30));
        self.enigo.key(Key::C, Direction::Click)?;
        thread::sleep(Duration::from_millis(30));
        self.enigo.key(Key::Control, Direction::Release)?;
        Ok(())
    }

    pub fn copy_selection(
        &mut self,
        delay_ms: u64,
        previous_clipboard: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        info!("copy_selection: sending Ctrl+C (up to 5 attempts)");
        let marker = format!(
            "__txtfrmt_probe_{}_{}__",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)?
                .as_nanos()
        );
        self.clipboard.set_text(marker.clone())?;
        let copy_result = (|| -> Result<String, Box<dyn std::error::Error>> {
            for attempt in 1..=5 {
                self.send_ctrl_c()?;
                thread::sleep(Duration::from_millis(delay_ms));

                let after = self.get_text().unwrap_or_default();
                info!(
                    "copy_selection: attempt {attempt}: clipboard has {} bytes",
                    after.len()
                );

                if !after.is_empty() && after != marker {
                    info!("Copied selected text ({} bytes)", after.len());
                    return Ok(after);
                }
            }
            let last = self.get_text().unwrap_or_default();
            warn!(
                "Nothing selected after 5 attempts (clipboard has {} bytes)",
                last.len()
            );
            Ok(String::new())
        })();

        match copy_result {
            Ok(selected) if !selected.is_empty() => Ok(selected),
            Ok(_) => {
                self.clipboard.set_text(previous_clipboard)?;
                Ok(String::new())
            }
            Err(error) => {
                self.clipboard.set_text(previous_clipboard)?;
                Err(error)
            }
        }
    }

    fn paste_key(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.enigo.key(Key::Control, Direction::Press)?;
        thread::sleep(Duration::from_millis(30));
        self.enigo.key(Key::V, Direction::Click)?;
        thread::sleep(Duration::from_millis(30));
        self.enigo.key(Key::Control, Direction::Release)?;
        Ok(())
    }

    pub fn paste_text(
        &mut self,
        text: &str,
        delay_ms: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        info!("paste_text: setting clipboard ({} bytes)", text.len());
        self.clipboard.set_text(text)?;
        thread::sleep(Duration::from_millis(delay_ms / 2));

        info!("paste_text: sending Ctrl+V");
        self.paste_key()?;

        info!("Paste complete");
        Ok(())
    }

    pub fn transform_selection(
        &mut self,
        transform: impl FnOnce(&str) -> String,
        restore: bool,
        delay_ms: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let old_clipboard = self.clipboard.get_text().unwrap_or_default();
        info!(
            "transform_selection: starting, clipboard has {} bytes",
            old_clipboard.len()
        );

        let selected = self.copy_selection(delay_ms, &old_clipboard)?;
        if selected.is_empty() {
            warn!("No text selected");
            return Ok(());
        }

        info!(
            "transform_selection: applying transform to {} selected bytes",
            selected.len()
        );
        let transformed = transform(&selected);
        info!(
            "transform_selection: result has {} bytes",
            transformed.len()
        );

        if transformed == selected {
            warn!("No change after transform");
            if restore {
                let _ = self.clipboard.set_text(&old_clipboard);
            }
            return Ok(());
        }

        let paste_result = self.paste_text(&transformed, delay_ms);
        if restore {
            thread::sleep(Duration::from_millis(delay_ms));
            info!("transform_selection: restoring previous clipboard");
            let _ = self.clipboard.set_text(&old_clipboard);
        }
        paste_result?;

        info!("transform_selection: done");
        Ok(())
    }
}
