use std::path::Path;

use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tauri_plugin_dialog::DialogExt;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileImport {
    pub name: String,
    pub path: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DragDropPayload {
    pub paths: Vec<String>,
}

#[tauri::command]
pub fn select_file_dialog(app: tauri::AppHandle) {
    app.dialog().file().pick_files(move |paths| {
        if let Some(paths) = paths {
            let mut files: Vec<FileImport> = vec![];

            for path in paths {
                let file_path = path.clone();
                let name = Path::new(&path.into_path().unwrap())
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string();

                files.push(FileImport {
                    name,
                    path: file_path.to_string(),
                    tags: vec![],
                });
            }

            app.emit("file-import-finished", &files).unwrap();
        }
    });
}

#[tauri::command]
pub fn drag_drop_file_dialog(app: tauri::AppHandle, payload: DragDropPayload) {
    // println!("{:?}", payload);

    let mut files: Vec<FileImport> = vec![];

    for path in payload.paths {
        let file_path = path.clone();
        let name = path;

        files.push(FileImport {
            name,
            path: file_path.to_string(),
            tags: vec![],
        });
    }

    app.emit("file-import-finished", &files).unwrap();
}
