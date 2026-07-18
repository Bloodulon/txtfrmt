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

    pub fn copy_selection(&mut self, delay_ms: u64) -> Result<String, Box<dyn std::error::Error>> {
        info!("copy_selection: sending Ctrl+C (up to 5 attempts)");
        for attempt in 1..=5 {
            let before = self.get_text().unwrap_or_default();
            self.send_ctrl_c()?;
            thread::sleep(Duration::from_millis(delay_ms));

            let after = self.get_text().unwrap_or_default();
            info!("copy_selection: attempt {attempt}: clip='{after}'");

            if !after.is_empty() && after != before {
                info!("Copied text: '{}'", after);
                return Ok(after);
            }
        }
        let last = self.get_text().unwrap_or_default();
        warn!("Nothing selected after 5 attempts (last clip='{last}')");
        Ok(String::new())
    }

    fn paste_key(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.enigo.key(Key::Control, Direction::Press)?;
        thread::sleep(Duration::from_millis(30));
        self.enigo.key(Key::V, Direction::Click)?;
        thread::sleep(Duration::from_millis(30));
        self.enigo.key(Key::Control, Direction::Release)?;
        Ok(())
    }

    pub fn paste_text(&mut self, text: &str, delay_ms: u64) -> Result<(), Box<dyn std::error::Error>> {
        info!("paste_text: setting clipboard to '{}'", text);
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
        info!("transform_selection: starting, old_clipboard='{}'", old_clipboard);

        let selected = self.copy_selection(delay_ms)?;
        if selected.is_empty() {
            warn!("No text selected");
            return Ok(());
        }

        info!("transform_selection: applying transform to '{}'", selected);
        let transformed = transform(&selected);
        info!("transform_selection: transformed='{}'", transformed);

        if transformed == selected {
            warn!("No change after transform");
            return Ok(());
        }

        self.paste_text(&transformed, delay_ms)?;

        if restore {
            thread::sleep(Duration::from_millis(delay_ms));
            info!("transform_selection: restoring clipboard to '{}'", old_clipboard);
            let _ = self.clipboard.set_text(&old_clipboard);
        }

        info!("transform_selection: done");
        Ok(())
    }
}
