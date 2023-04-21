#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use arboard::Clipboard;
use std::sync::Mutex;
use tauri::Manager;

mod initialize;

#[tokio::main]
async fn main() {
    initialize::init();

    tauri::async_runtime::set(tokio::runtime::Handle::current());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_clipboard])
        .setup(|app| {
            let clipboard_manager = ClipboardManager::new()?;
            app.manage(clipboard_manager);

            #[cfg(debug_assertions)]
            app.get_window("main").unwrap().open_devtools();

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
struct ClipboardManager {
    inner: Mutex<Clipboard>,
}
impl ClipboardManager {
    pub fn new() -> anyhow::Result<Self> {
        let mut clipboard = Clipboard::new()?;
        Ok(Self {
            inner: Mutex::new(clipboard),
        })
    }
    pub fn set_clipboard(&self, text: &str) -> anyhow::Result<()> {
        // lock-unlock の間に await してはいけない
        let mut clipboard = self.inner.lock().unwrap();
        clipboard.set_text(text).unwrap();
        Ok(())
    }
}

#[tauri::command]
fn set_clipboard(
    clipboard_manager: tauri::State<'_, ClipboardManager>,
    text: &str,
) -> Result<(), String> {
    match clipboard_manager.set_clipboard(text) {
        Ok(_) => return Ok(()),
        Err(_) => return Err("Err in clipboard".to_string()),
    }
}
