// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("running");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

                // Using separate modifiers and combining them
                let modifiers = Modifiers::ALT.union(Modifiers::CONTROL);
                let shortcut = Shortcut::new(Some(modifiers), Code::KeyX);
                println!("Registering shortcut: {:?}", shortcut);

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |app, shortcut_triggered, _event| {
                            println!("Received shortcut: {:?}", shortcut_triggered);
                            if shortcut_triggered == &shortcut {
                                println!("shortcut triggered");
                                if let Some(window) = app.get_webview_window("main") {
                                    println!("window found");
                                    window.show().unwrap();
                                    window.set_focus().unwrap();
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(shortcut)?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
