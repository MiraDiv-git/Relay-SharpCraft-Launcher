use slint::ComponentHandle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

slint::include_modules!();

fn main() {
    let ui = MainWindow::new().unwrap();
    let default_lang = "en";

    let mut locales = HashMap::new();
    locales.insert("ua", include_str!("../lang/ua.json"));
    locales.insert("en", include_str!("../lang/en.json"));

    let initial_json = locales.get(default_lang).unwrap_or(&"{}");
    let current_map: HashMap<String, String> = serde_json::from_str(initial_json).unwrap_or_default();

    let translations = Arc::new(Mutex::new(current_map));

    let tr_clone = translations.clone();
    ui.global::<L10n>().on_get(move |key| {
        let map = tr_clone.lock().unwrap();
        map.get(key.as_str())
            .map(|s| s.to_string().into())
            .unwrap_or(key)
    });

    ui.run().unwrap();
}