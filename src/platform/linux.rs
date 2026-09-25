use crate::transform::Transform;
use std::{
    io::Write,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

fn run(program: &str, args: &[&str]) -> Result<std::process::Output, String> {
    Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("cannot run {program}: {e}"))
}

fn clipboard_text() -> Result<String, String> {
    let out = run("wl-paste", &["--no-newline", "--type", "text"])?;
    if !out.status.success() {
        return Err("wl-paste failed; is a Wayland session active?".into());
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

fn key(chord: &[&str]) -> Result<(), String> {
    let status = Command::new("wtype")
        .args(chord)
        .status()
        .map_err(|e| format!("cannot run wtype: {e}"))?;
    if !status.success() {
        return Err("wtype failed to send keyboard shortcut".into());
    }
    Ok(())
}

fn copy() -> Result<String, String> {
    // Hyprland launches the bound command while its modifiers are still held.
    // Let Ctrl/Shift from the binding release before synthesizing Ctrl+C.
    thread::sleep(Duration::from_millis(350));
    let status = Command::new("wl-copy")
        .arg("--clear")
        .status()
        .map_err(|e| format!("cannot clear clipboard with wl-copy: {e}"))?;
    if !status.success() {
        return Err("wl-copy could not clear clipboard; selection was not copied".into());
    }
    let clear_deadline = Instant::now() + Duration::from_millis(500);
    while Instant::now() < clear_deadline {
        if clipboard_text().map(|s| s.is_empty()).unwrap_or(true) {
            break;
        }
        thread::sleep(Duration::from_millis(25));
    }
    if clipboard_text().map(|s| !s.is_empty()).unwrap_or(false) {
        return Err(
            "clipboard still contains old text after clearing; refusing to paste it".into(),
        );
    }
    key(&["-M", "ctrl", "-k", "c", "-m", "ctrl"])?;
    let deadline = Instant::now() + Duration::from_millis(900);
    while Instant::now() < deadline {
        thread::sleep(Duration::from_millis(40));
        let after = clipboard_text().unwrap_or_default();
        if !after.is_empty() {
            return Ok(after);
        }
    }
    Err("no selected text detected (Ctrl+C did not populate the clipboard)".into())
}

fn set_clipboard(text: &str) -> Result<(), String> {
    let mut child = Command::new("wl-copy")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("cannot run wl-copy: {e}"))?;
    child
        .stdin
        .take()
        .unwrap()
        .write_all(text.as_bytes())
        .map_err(|e| format!("wl-copy stdin: {e}"))?;
    let status = child.wait().map_err(|e| format!("wl-copy: {e}"))?;
    if !status.success() {
        return Err("wl-copy failed".into());
    }
    Ok(())
}

pub fn transform_selection(
    transform: Transform,
    delay_ms: u64,
    restore: bool,
) -> Result<(), String> {
    let old_clipboard = clipboard_text().unwrap_or_default();
    let selected = match copy() {
        Ok(text) => text,
        Err(error) => {
            // The copy path clears the clipboard; recover it even when
            // restore_clipboard is disabled because no transform happened.
            let _ = set_clipboard(&old_clipboard);
            return Err(error);
        }
    };
    let changed = transform.apply(&selected);
    if changed == selected {
        if restore {
            set_clipboard(&old_clipboard)?;
        }
        return Ok(());
    }
    set_clipboard(&changed)?;
    thread::sleep(Duration::from_millis(delay_ms.max(80)));
    key(&["-M", "ctrl", "-k", "v", "-m", "ctrl"])?;
    thread::sleep(Duration::from_millis(delay_ms.max(100)));
    if restore {
        set_clipboard(&old_clipboard)?;
    }
    Ok(())
}
