use crate::commands::{Command, CommandOutput};
use crate::db::DatabaseState;
use crate::history::History;
use crate::Error;
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite};
use std::path::Path;
use std::sync::Mutex;
use tauri::async_runtime::block_on;
use tauri::{Emitter, State};
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

#[derive(Default)]
pub struct ImportCommand {
    files_import: Vec<FileImport>,
}

impl ImportCommand {
    pub fn new(files: Vec<FileImport>) -> Self {
        Self {
            files_import: files,
        }
    }
}

impl Command for ImportCommand {
    fn execute(&mut self, db: State<'_, DatabaseState>) -> Result<CommandOutput, Error> {
        let pool = &db.pool;

        for import in &self.files_import {
            block_on(
                sqlx::query!("INSERT INTO patterns (name) VALUES ($1)", import.name).execute(pool),
            )?;

            for tag in &import.tags {
                block_on(
                    sqlx::query!(
                        "
                INSERT INTO tag_map (pattern_id, tag_id)
                SELECT 
                (SELECT id FROM patterns WHERE name = $1),
                (SELECT id FROM tag WHERE name = $2);
                ",
                        import.name,
                        tag
                    )
                    .execute(pool),
                )?;
            }
        }

        println!("Imported files: {:?}", self.files_import);

        Ok(CommandOutput::None)
    }

    fn undo(&mut self, db: State<'_, DatabaseState>) -> Result<CommandOutput, Error> {
        let pool = &db.pool;

        for import in &self.files_import {
            block_on(
                sqlx::query!("DELETE FROM patterns WHERE name = $1", import.name).execute(pool),
            )?;

            for tag in &import.tags {
                block_on(
                    sqlx::query!(
                        "DELETE FROM tag_map 
                     WHERE pattern_id = (SELECT id FROM patterns WHERE name = $1)
                     AND tag_id = (SELECT id FROM tag WHERE name = $2)",
                        import.name,
                        tag
                    )
                    .execute(pool),
                )?;

                // if self.delete_tags {
                //     sqlx::query!(
                //         "DELETE FROM tag
                //          WHERE name = $1",
                //         tag
                //     )
                //     .execute(pool)
                //     .block_on()?;
                // }
            }
        }

        println!("Undid import files: {:?}", self.files_import);

        Ok(CommandOutput::None)
    }
}

#[tauri::command]
pub fn import_files(
    db: tauri::State<'_, DatabaseState>,
    history: tauri::State<'_, Mutex<History>>,
    files: Vec<FileImport>,
) -> Result<CommandOutput, Error> {
    let import_command = ImportCommand::new(files);

    let mut history_state = history.lock().unwrap();

    history_state.add_command(db, import_command)
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

// #[tauri::command]
// pub async fn import_files(
//     state: tauri::State<'_, DatabaseState>,
//     files: Vec<FileImport>,
// ) -> Result<(), Error> {
//     let pool = &state.pool;
//
//     for import in files {
//         let name = import.name;
//         let tags = import.tags;
//         let _path = import.path;
//
//         let pattern_insert = r#"INSERT INTO patterns (name) VALUES ($1)"#;
//
//         sqlx::query(pattern_insert)
//             .bind(name.clone())
//             .execute(pool)
//             .await?;
//
//         for tag in tags {
//             // println!("pattern name: {}, tag name: {}", name, tag);
//
//             sqlx::query!(
//                 "
//                 INSERT INTO tag_map (pattern_id, tag_id)
//                 SELECT
//                 (SELECT id FROM patterns WHERE name = $1),
//                 (SELECT id FROM tag WHERE name = $2);
//                 ",
//                 name,
//                 tag
//             )
//             .execute(pool)
//             .await?;
//
//             // sqlx::query(tagmap_insert)
//             //     .bind(name.clone())
//             //     .bind(tag)
//             //     .execute(pool)
//             //     .await?;
//         }
//     }
//
//     // println!("Insertions finished");
//
//     Ok(())
// }
