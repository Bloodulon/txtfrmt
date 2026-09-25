# textfmt

**Background text formatter with global hotkeys for Windows and Hyprland.**

Select text in any application, press a hotkey — the text is transformed in place.

---

## Build

Cargo selects the platform implementation from the build target, so each build
produces one executable for that OS. A normal build targets the current OS:

```sh
cargo build --release
```

To cross-compile, install the Rust target and use `--target`; for example, from
Linux to 64-bit Windows:

```sh
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu
```

Cross-compiling also requires a suitable Windows linker/toolchain. The Windows
binary is written to `target/x86_64-pc-windows-gnu/release/textfmt.exe`.

On Windows, launch the daemon with `textfmt.exe`. On Arch/Hyprland, install
`wl-clipboard` and `wtype`, then bind `textfmt --transform <Transform>` as
described below.

## Usage

Run the daemon — it sits in the system tray (console auto-hides):

```powershell
textfmt
```

Open the settings TUI (text-based user interface):

```powershell
textfmt --settings
```

### Default Hotkeys

| Hotkey | Transform |
|---|---|
| `Ctrl+Shift+R` | Convert to Russian layout |
| `Ctrl+Shift+E` | Convert to English layout |
| `Ctrl+Shift+T` | Auto-detect layout and toggle |
| `Ctrl+Shift+U` | Toggle case (lower ↔ UPPER) |
| `Ctrl+Shift+C` | Clean extra whitespace |
| `Ctrl+Shift+S` | Open settings UI |

### All Transforms

- **ToRussian** — fixes English-looking text typed with Russian layout active (e.g., `ghbdtn` → `привет`)
- **ToEnglish** — fixes Russian-looking text typed with English layout active (e.g., `руддщ` → `hello`)
- **ToggleLayout** — detects the dominant layout and toggles to the opposite
- **ToggleCase** — if more uppercase letters → to lowercase, and vice versa
- **ToUpper** — transforms to UPPERCASE
- **ToLower** — transforms to lowercase
- **TitleCase** — capitalises each word
- **CleanWhitespace** — trims and collapses multiple spaces into one
- **Reverse** — reverses grapheme clusters
- **ToCamelCase** — converts `snake_case` or `kebab-case` to camelCase
- **ToSnakeCase** — converts camelCase to `snake_case`

### Configuration

The config file is created automatically at:

```
%APPDATA%\textfmt\config.toml
```

Example:

```toml
restore_clipboard = false
clipboard_delay_ms = 150

[[hotkeys]]
modifiers = ["CONTROL", "SHIFT"]
key = "KeyE"
transform = "ToEnglish"

[[hotkeys]]
modifiers = ["CONTROL", "SHIFT"]
key = "KeyT"
transform = "ToggleLayout"

[settings_hotkey]
modifiers = ["CONTROL", "SHIFT"]
key = "KeyS"
transform = "ToggleLayout"
```

### Logs

Stderr logs use `env_logger`. To capture:

```powershell
textfmt 2> textfmt.log
```

---

## Requirements

- Windows, or Linux/Wayland with Hyprland, `wl-clipboard` and `wtype`
- Rust 2024 edition

### Arch Linux / Hyprland

Install the Wayland helpers and build the program:

```sh
sudo pacman -S wl-clipboard wtype
cargo build --release
```

Copy `target/release/textfmt` to a directory on `PATH`, or install it with
`cargo install --path .`. Bind transformations in `~/.config/hypr/bindings.lua`:

```lua
o.bind("CTRL + SHIFT + R", nil, "textfmt --transform ToRussian")
o.bind("CTRL + SHIFT + E", nil, "textfmt --transform ToEnglish")
o.bind("CTRL + SHIFT + U", nil, "textfmt --transform ToggleCase")
```

Reload Hyprland, select text, then use a binding. Linux mode copies the
selection with `wtype`, transforms it, and pastes the result with `wl-clipboard`.
The `restore_clipboard` setting controls whether the prior text clipboard is
restored after pasting. Clipboard formats other than text are not preserved.

---

## License

MIT

---

## Русский

**Фоновый демон форматирования текста с глобальными горячими клавишами для Windows.**

Выделите текст в любом приложении, нажмите горячую клавишу — текст изменится прямо на месте.

### Использование

```powershell
textfmt
```

Настройки (текстовый интерфейс):

```powershell
textfmt --settings
```

### Горячие клавиши по умолчанию

| Клавиши | Действие |
|---|---|
| `Ctrl+Shift+R` | Переключить на русскую раскладку |
| `Ctrl+Shift+E` | Переключить на английскую раскладку |
| `Ctrl+Shift+T` | Автоопределение раскладки и переключение |
| `Ctrl+Shift+U` | Смена регистра (нижний ↔ ВЕРХНИЙ) |
| `Ctrl+Shift+C` | Очистить лишние пробелы |
| `Ctrl+Shift+S` | Открыть настройки |

### Все преобразования

- **На русскую раскладку** — исправляет текст, набранный в английской раскладке, когда думали, что включена русская (`ghbdtn` → `привет`)
- **На английскую раскладку** — исправляет текст, набранный в русской раскладке, когда думали, что включена английская (`руддщ` → `hello`)
- **Сменить раскладку** — определяет доминирующий язык и переключает на противоположный
- **Сменить регистр** — если больше заглавных → в нижний, иначе в ВЕРХНИЙ
- **ВЕРХНИЙ регистр** — все буквы заглавные
- **нижний регистр** — все буквы строчные
- **Заглавные Слова** — каждое слово с заглавной буквы
- **Очистить пробелы** — удаляет лишние пробелы, оставляя по одному
- **Развернуть** — переворачивает текст задом наперёд
- **camelCase** — преобразует `snake_case` или `kebab-case` в camelCase
- **snake_case** — преобразует camelCase в `snake_case`

### Конфигурация

Файл создаётся автоматически:

```
%APPDATA%\textfmt\config.toml
```

Пример:
```toml
restore_clipboard = false
clipboard_delay_ms = 150

[[hotkeys]]
modifiers = ["CONTROL", "SHIFT"]
key = "KeyE"
transform = "ToEnglish"
```

### Логи

Логи пишутся в stderr через `env_logger`. Сохранить в файл:

```powershell
textfmt 2> textfmt.log
```

### Требования

- Windows (используется Win32 API для низкоуровневого хука клавиатуры)
- Rust 2024 edition

### Лицензия

MIT
