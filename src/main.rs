use slint::ComponentHandle;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use include_dir::{include_dir, Dir};

static LANG_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/lang");

slint::include_modules!();

fn main() {
    if std::env::var("SLINT_BACKEND").is_err() {
        unsafe {
            std::env::set_var("SLINT_BACKEND", "software");
        }
    }

    let default_language = "en";

    let ui = MainWindow::new().unwrap();
    let translations = Arc::new(Mutex::new(HashMap::<String, String>::new()));

    let tr_clone = translations.clone();
    ui.global::<L10n>().on_get(move |key| {
        let map = tr_clone.lock().unwrap();
        map.get(key.as_str())
            .map(|s| s.to_string().into())
            .unwrap_or(key)
    });

    load_embedded_locale(&translations, default_language);
    load_click_events(&ui);

    ui.run().unwrap();
}

fn load_embedded_locale(map_arc: &Arc<Mutex<HashMap<String, String>>>, lang: &str) {
    let file_name = format!("{}.json", lang);
    if let Some(file) = LANG_DIR.get_file(file_name) {
        if let Ok(json) = serde_json::from_str::<HashMap<String, String>>(file.contents_utf8().unwrap()) {
            let mut map = map_arc.lock().unwrap();
            *map = json;
        }
    }
}

fn load_click_events(ui: &MainWindow) {
    let ui_weak = ui.as_weak();

    ui.on_launch_clicked({
        let ui_handle = ui_weak.clone();
        move || {
            let _ui = ui_handle.unwrap();
            println!("Launch pressed");
        }
    });

    ui.on_account_clicked({
        let ui_handle = ui_weak.clone();
        move || {
            let _ui = ui_handle.unwrap();
            _ui.set_active_screen(1);
        }
    });

    ui.on_settings_clicked({
        let ui_handle = ui_weak.clone();
        move || {
            let _ui = ui_handle.unwrap();
            _ui.set_active_screen(2);
        }
    });

    ui.on_versions_clicked({
        let ui_handle = ui_weak.clone();
        move || {
            let _ui = ui_handle.unwrap();
            _ui.set_active_screen(3);
        }
    });
}