use slint::ComponentHandle;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex};

slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();

    let translations = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    // Связываем колбэк Slint с нашим HashMap в Rust
    let tr_clone = translations.clone();
    ui.global::<L10n>().on_get(move |key| {
        let map = tr_clone.lock().unwrap();
        map.get(key.as_str())
            .map(|s| s.as_str().into())
            .unwrap_or(key)
    });

    load_locale_to_map(&translations, "en");

    ui.run().unwrap();
}

fn load_locale_to_map(map_arc: &Arc<Mutex<HashMap<String, String>>>, lang: &str) {
    let path = format!("lang/{}.json", lang);
    if let Ok(content) = fs::read_to_string(path) {
        if let Ok(json) = serde_json::from_str::<HashMap<String, String>>(&content) {
            let mut map = map_arc.lock().unwrap();
            *map = json;
        }
    }
}