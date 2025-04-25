use std::sync::Mutex;
use tauri::Manager;
use thiserror::Error;

mod commands;
mod db;
mod history;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
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
                app.manage(db::DatabaseState { pool: database.pool });
                
                // history for session
                app.manage(history::History::new());
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_tags,
            commands::add_tag,
            commands::select_file_dialog,
            commands::drag_drop_file_dialog,
            commands::import_files,
            commands::get_patterns,
            commands::get_pattern_tags,
            commands::search_patterns,
            commands::undo,
            commands::redo,
        ])
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

// Use thiserror::Error to implement serializable errors
// that are returned by commands
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),

    // New variant for database errors
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    // New variant for string-related errors
    #[error("String error: {0}")]
    StringError(String), // Include a String to represent the error message
}

// Errors must implement serde::Serialize to be used in Commands
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}
