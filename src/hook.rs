use crate::daemon::DaemonEvent;
use crate::settings::Settings;
use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, mpsc};

struct HookState {
    ctrl: AtomicBool,
    shift: AtomicBool,
    alt: AtomicBool,
    meta: AtomicBool,
    pressed_keys: Mutex<HashSet<u32>>,
    settings: Settings,
    tx: mpsc::Sender<DaemonEvent>,
}

static mut HOOK_STATE: Option<HookState> = None;

unsafe extern "system" fn hook_callback(code: i32, wparam: usize, lparam: isize) -> isize {
    use winapi::um::winuser::{
        CallNextHookEx, HC_ACTION, KBDLLHOOKSTRUCT, WM_KEYDOWN, WM_KEYUP, WM_SYSKEYDOWN,
        WM_SYSKEYUP,
    };

    if code == HC_ACTION {
        let wparam = wparam as u32;
        let is_down = wparam == WM_KEYDOWN || wparam == WM_SYSKEYDOWN;
        let is_up = wparam == WM_KEYUP || wparam == WM_SYSKEYUP;

        if is_down || is_up {
            let kb = unsafe { *(lparam as *const KBDLLHOOKSTRUCT) };
            let vk = kb.vkCode;

            let state_ptr: *const Option<HookState> = &raw const HOOK_STATE;
            let state = unsafe { &*state_ptr };
            if let Some(ref state) = state.as_ref() {
                let repeated = if is_down {
                    state
                        .pressed_keys
                        .lock()
                        .map(|mut keys| !keys.insert(vk))
                        .unwrap_or(false)
                } else {
                    if let Ok(mut keys) = state.pressed_keys.lock() {
                        keys.remove(&vk);
                    }
                    false
                };
                match vk {
                    0x11 | 0xA2 | 0xA3 => state.ctrl.store(is_down, Ordering::SeqCst),
                    0x10 | 0xA0 | 0xA1 => state.shift.store(is_down, Ordering::SeqCst),
                    0x12 | 0xA4 | 0xA5 => state.alt.store(is_down, Ordering::SeqCst),
                    0x5B | 0x5C => state.meta.store(is_down, Ordering::SeqCst),
                    _ if is_down => {
                        let c = state.ctrl.load(Ordering::SeqCst);
                        let s = state.shift.load(Ordering::SeqCst);
                        let a = state.alt.load(Ordering::SeqCst);
                        let m = state.meta.load(Ordering::SeqCst);

                        for hk in &state.settings.hotkeys {
                            if let Some((vk_mods, vk_key)) = parse_vk(&hk.modifiers, &hk.key) {
                                if mods_match(vk_mods, c, s, a, m) && vk as u32 == vk_key {
                                    if !repeated {
                                        let _ = state.tx.send(DaemonEvent::Transform(hk.transform));
                                    }
                                    return 1;
                                }
                            }
                        }

                        if let Some((vk_mods, vk_key)) = parse_vk(
                            &state.settings.settings_hotkey.modifiers,
                            &state.settings.settings_hotkey.key,
                        ) {
                            if mods_match(vk_mods, c, s, a, m) && vk as u32 == vk_key {
                                if !repeated {
                                    let _ = state.tx.send(DaemonEvent::OpenSettings);
                                }
                                return 1;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    unsafe { CallNextHookEx(std::ptr::null_mut(), code, wparam as _, lparam as _) }
}

fn mods_match(vk_mods: u32, ctrl: bool, shift: bool, alt: bool, meta: bool) -> bool {
    let actual = (ctrl as u32) | ((shift as u32) << 1) | ((alt as u32) << 2) | ((meta as u32) << 3);
    actual == vk_mods
}

pub fn run_keyboard_hook(settings: &Settings, tx: mpsc::Sender<DaemonEvent>) {
    unsafe {
        HOOK_STATE = Some(HookState {
            ctrl: AtomicBool::new(false),
            shift: AtomicBool::new(false),
            alt: AtomicBool::new(false),
            meta: AtomicBool::new(false),
            pressed_keys: Mutex::new(HashSet::new()),
            settings: settings.clone(),
            tx,
        });

        use winapi::um::winuser::{
            GetMessageW, MSG, SetWindowsHookExW, UnhookWindowsHookEx, WH_KEYBOARD_LL,
        };

        let hook = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(hook_callback as unsafe extern "system" fn(i32, usize, isize) -> isize),
            std::ptr::null_mut(),
            0,
        );

        if hook.is_null() {
            eprintln!("SetWindowsHookExW failed");
            return;
        }

        let mut msg = std::mem::zeroed::<MSG>();
        while GetMessageW(&mut msg, std::ptr::null_mut(), 0, 0) > 0 {}

        UnhookWindowsHookEx(hook);
        HOOK_STATE = None;
    }
}

fn parse_vk(mod_strings: &[String], key: &str) -> Option<(u32, u32)> {
    let mut mods = 0u32;
    for m in mod_strings {
        match m.to_uppercase().as_str() {
            "CONTROL" | "CTRL" => mods |= 1,
            "SHIFT" => mods |= 2,
            "ALT" => mods |= 4,
            "META" | "SUPER" | "WIN" | "CMD" => mods |= 8,
            _ => return None,
        }
    }
    let vk = key_to_vk(key)?;
    Some((mods, vk))
}

fn key_to_vk(key: &str) -> Option<u32> {
    match key {
        "KeyA" => Some(0x41),
        "KeyB" => Some(0x42),
        "KeyC" => Some(0x43),
        "KeyD" => Some(0x44),
        "KeyE" => Some(0x45),
        "KeyF" => Some(0x46),
        "KeyG" => Some(0x47),
        "KeyH" => Some(0x48),
        "KeyI" => Some(0x49),
        "KeyJ" => Some(0x4A),
        "KeyK" => Some(0x4B),
        "KeyL" => Some(0x4C),
        "KeyM" => Some(0x4D),
        "KeyN" => Some(0x4E),
        "KeyO" => Some(0x4F),
        "KeyP" => Some(0x50),
        "KeyQ" => Some(0x51),
        "KeyR" => Some(0x52),
        "KeyS" => Some(0x53),
        "KeyT" => Some(0x54),
        "KeyU" => Some(0x55),
        "KeyV" => Some(0x56),
        "KeyW" => Some(0x57),
        "KeyX" => Some(0x58),
        "KeyY" => Some(0x59),
        "KeyZ" => Some(0x5A),
        "Digit0" => Some(0x30),
        "Digit1" => Some(0x31),
        "Digit2" => Some(0x32),
        "Digit3" => Some(0x33),
        "Digit4" => Some(0x34),
        "Digit5" => Some(0x35),
        "Digit6" => Some(0x36),
        "Digit7" => Some(0x37),
        "Digit8" => Some(0x38),
        "Digit9" => Some(0x39),
        "F1" => Some(0x70),
        "F2" => Some(0x71),
        "F3" => Some(0x72),
        "F4" => Some(0x73),
        "F5" => Some(0x74),
        "F6" => Some(0x75),
        "F7" => Some(0x76),
        "F8" => Some(0x77),
        "F9" => Some(0x78),
        "F10" => Some(0x79),
        "F11" => Some(0x7A),
        "F12" => Some(0x7B),
        "Space" => Some(0x20),
        "Enter" => Some(0x0D),
        "Escape" => Some(0x1B),
        "Tab" => Some(0x09),
        "Backspace" => Some(0x08),
        "Delete" => Some(0x2E),
        "ArrowUp" => Some(0x26),
        "ArrowDown" => Some(0x28),
        "ArrowLeft" => Some(0x25),
        "ArrowRight" => Some(0x27),
        _ => None,
    }
}
