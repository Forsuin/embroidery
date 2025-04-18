use std::path::Path;

use serde::{Deserialize, Serialize};
use tauri::Emitter;
use tauri_plugin_dialog::DialogExt;

use crate::db::DatabaseState;
use crate::Error;

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

            app.emit("file-import-populate", &files).unwrap();
        }
    });
}

#[tauri::command]
pub fn drag_drop_file_dialog(app: tauri::AppHandle, payload: DragDropPayload) {
    // println!("{:?}", payload);

    let mut files: Vec<FileImport> = vec![];

    for path in payload.paths {
        let file_path = path.clone();
        let name = Path::new(&path)
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

    app.emit("file-import-populate", &files).unwrap();
}

#[tauri::command]
pub async fn import_files(
    state: tauri::State<'_, DatabaseState>,
    files: Vec<FileImport>,
) -> Result<(), Error> {
    let pool = &state.0;

    for import in files {
        let name = import.name;
        let tags = import.tags;
        let _path = import.path;

        let pattern_insert = r#"INSERT INTO patterns (name) VALUES ($1)"#;

        sqlx::query(pattern_insert)
            .bind(name.clone())
            .execute(pool)
            .await?;

        for tag in tags {
            // println!("pattern name: {}, tag name: {}", name, tag);

            sqlx::query!(
                "
                INSERT INTO tag_map (pattern_id, tag_id)
                SELECT 
                (SELECT id FROM patterns WHERE name = $1),
                (SELECT id FROM tag WHERE name = $2);
                ",
                name,
                tag
            )
            .execute(pool)
            .await?;

            // sqlx::query(tagmap_insert)
            //     .bind(name.clone())
            //     .bind(tag)
            //     .execute(pool)
            //     .await?;
        }
    }

    // println!("Insertions finished");

    
    
    Ok(())
}
