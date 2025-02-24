use tauri::menu::{MenuBuilder, MenuItem, MenuItemBuilder, SubmenuBuilder};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let _handle = app.handle();

            let file_explorer_prompt = if cfg!(target_os = "windows") {
                "Reveal in File Explorer"
            } else if cfg!(target_os = "macos") {
                "Show in Finder"
            } else {
                "Show in File Manager"
            };

            let file_submenu = SubmenuBuilder::new(app, "File")
                .item(
                    &MenuItemBuilder::new("Import")
                        .id("import")
                        .accelerator("CmdOrCtrl+I")
                        .build(app)?,
                )
                .item(
                    &MenuItemBuilder::new(file_explorer_prompt)
                        .id("reveal")
                        .accelerator("CmdOrCtrl+Alt+R")
                        .build(app)?,
                )
                .separator()
                .item(
                    &MenuItemBuilder::new("Settings")
                        .id("settings")
                        .accelerator("CmdOrCtrl+,")
                        .build(app)?,
                )
                .separator()
                .item(
                    &MenuItemBuilder::new("Quit")
                        .id("quit")
                        .accelerator("CmdOrCtrl+q")
                        .build(app)?,
                )
                .build()?;

            let edit_submenu = SubmenuBuilder::new(app, "Edit")
                .items(&[
                    &MenuItemBuilder::new("Undo".to_string())
                        .id("undo")
                        .accelerator("CmdOrCtrl+Z")
                        .build(app)?,
                    &MenuItemBuilder::new("Redo")
                        .id("redo")
                        .accelerator("CmdOrCtrl+Shift+Z")
                        .build(app)?,
                ])
                .build()?;

            let menu = MenuBuilder::new(app)
                .items(&[&file_submenu, &edit_submenu])
                .build()?;

            app.set_menu(menu)?;

            app.on_menu_event(move |_app_handle: &tauri::AppHandle, event| {
                println!("Event: {:?}", event.id());
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
