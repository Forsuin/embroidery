use tauri::Manager;

mod db;

#[tauri::command]
fn format_search_query(args: Vec<String>) -> String {
    args.join(", ")
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(prevent_default())
        .setup(|app| {
            let handle = app.handle();

            // init db
            tauri::async_runtime::block_on(async {
                let database = db::Database::new(&handle)
                    .await
                    .expect("Failed to initialize database");

                // store db pool in app state
                app.manage(db::DatabaseState(database.pool));
            });

            Ok(())
        })
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
