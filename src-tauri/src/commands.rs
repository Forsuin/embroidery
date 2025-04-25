mod file_import;
mod patterns;
mod tags;

use crate::db::DatabaseState;
use crate::history::History;
use crate::Error;
pub use file_import::*;
pub use patterns::*;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::sync::Mutex;
pub use tags::*;
use tauri::State;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pattern {
    pub id: u32,
    pub name: String,
    pub pattern_num: Option<u32>,
    pub thread_count: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    #[sqlx(default)]
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Serialize)]
pub enum CommandOutput {
    None,
    Pattern(Vec<Pattern>),
    Tag(Vec<Tag>),
}

pub trait Command: Send + Sync {
    fn execute(&mut self, db: State<'_, DatabaseState>) -> Result<CommandOutput, Error>;
    fn undo(&mut self, db: State<'_, DatabaseState>) -> Result<CommandOutput, Error>;
}

/// Tries to undo the most recent command
/// Returns `Some<Result<CommandOutput, Error>>` if a command can be undone
/// otherwise returns `None`
#[tauri::command]
pub fn undo(
    db: tauri::State<'_, DatabaseState>,
    history: tauri::State<'_, Mutex<History>>,
) -> Option<Result<CommandOutput, Error>> {
    let mut history_state = history.lock().unwrap();

    history_state.undo(db)
}

/// Tries to execute a more recent command 
/// Returns `Some<Result<CommandOutput, Error>>` if a redo command can be run
/// otherwise returns `None`
#[tauri::command]
pub fn redo(
    db: tauri::State<'_, DatabaseState>,
    history: tauri::State<'_, Mutex<History>>,
) -> Option<Result<CommandOutput, Error>> {
    let mut history_state = history.lock().unwrap();

    history_state.redo(db)
}
