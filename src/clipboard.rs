use arboard::Clipboard;
use enigo::{Enigo, Keyboard, Settings as EnigoSettings, Key, Direction};
use std::thread;
use std::time::Duration;
use log::{info, warn};

pub struct ClipboardManager {
    clipboard: Clipboard,
    enigo: Enigo,
}

impl ClipboardManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let clipboard = Clipboard::new()?;
        let enigo = Enigo::new(&EnigoSettings::default())?;
        Ok(Self { clipboard, enigo })
    }

    fn safe_get_text(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        for _ in 0..5 {
            match self.clipboard.get_text() {
                Ok(text) => return Ok(text.trim_start_matches('\u{FEFF}').to_string()),
                Err(_) => {
                    thread::sleep(Duration::from_millis(20));
                    continue;
                }
            }
        }
        Err("Clipboard occupied for too long".into())
    }

    fn safe_set_text(&mut self, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        for _ in 0..5 {
            match self.clipboard.set_text(text) {
                Ok(_) => return Ok(()),
                Err(_) => {
                    thread::sleep(Duration::from_millis(20));
                    continue;
                }
            }
        }
        Err("Clipboard occupied for too long".into())
    }

    fn send_ctrl_c(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.enigo.key(Key::Control, Direction::Press)?;
        thread::sleep(Duration::from_millis(20));
        self.enigo.key(Key::C, Direction::Click)?;
        thread::sleep(Duration::from_millis(20));
        self.enigo.key(Key::Control, Direction::Release)?;
        Ok(())
    }

    pub fn copy_selection(&mut self, delay_ms: u64) -> Result<String, Box<dyn std::error::Error>> {
        info!("copy_selection: sending Ctrl+C (up to 5 attempts)");
        for attempt in 1..=5 {
            let before = self.safe_get_text().unwrap_or_default();
            self.send_ctrl_c()?;
            thread::sleep(Duration::from_millis(delay_ms));

            let after = self.safe_get_text().unwrap_or_default();
            if !after.is_empty() && after != before {
                return Ok(after);
            }
        }
        warn!("Nothing selected after 5 attempts");
        Ok(String::new())
    }

    fn send_ctrl_v(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.enigo.key(Key::Control, Direction::Press)?;
        thread::sleep(Duration::from_millis(20));
        self.enigo.key(Key::V, Direction::Click)?;
        thread::sleep(Duration::from_millis(20));
        self.enigo.key(Key::Control, Direction::Release)?;
        Ok(())
    }

    pub fn paste_text(&mut self, text: &str, delay_ms: u64) -> Result<(), Box<dyn std::error::Error>> {
        self.safe_set_text(text)?;
        thread::sleep(Duration::from_millis(delay_ms / 2));
        self.send_ctrl_v()?;
        Ok(())
    }

    pub fn transform_selection(
        &mut self,
        transform: impl FnOnce(&str) -> String,
        restore: bool,
        delay_ms: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let old_clipboard = self.safe_get_text().unwrap_or_default();
        let selected = self.copy_selection(delay_ms)?;
        if selected.is_empty() {
            return Ok(());
        }

        let transformed = transform(&selected);
        if transformed == selected {
            return Ok(());
        }

        self.paste_text(&transformed, delay_ms)?;

        if restore {
            thread::sleep(Duration::from_millis(delay_ms));
            let _ = self.safe_set_text(&old_clipboard);
        }
        Ok(())
    }
}
