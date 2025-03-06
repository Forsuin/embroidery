use tauri::{
    menu::{MenuBuilder, MenuItem, MenuItemBuilder, SubmenuBuilder},
    Listener,
};

#[tauri::command]
fn format_search_query(args: Vec<String>) -> String {
    args.join(", ")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_os::init())
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
                println!("Event: {:?}", event);
            });

            app.listen("import", |event| {
                println!("Frontend event: {:?}", event);
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default())
        .invoke_handler(tauri::generate_handler![format_search_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(debug_assertions)]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    use tauri_plugin_prevent_default::Flags;

    tauri_plugin_prevent_default::Builder::new()
        .with_flags(Flags::all().difference(Flags::DEV_TOOLS | Flags::RELOAD))
        .build()
}

#[cfg(not(debug_assertions))]
fn prevent_default() -> tauri::plugin::TauriPlugin<tauri::Wry> {
    tauri_plugin_prevent_default::init()
}
