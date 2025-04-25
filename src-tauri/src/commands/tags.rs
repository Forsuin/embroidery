use crate::commands::{Command, CommandOutput, Tag};
use crate::db::*;
use crate::history::History;
use crate::Error;
use pollster::FutureExt;
use std::sync::Mutex;
use tauri::State;

pub struct AddTagCommand {
    tag_name: String,
}

impl AddTagCommand {
    pub fn new(tag_name: String) -> Self {
        Self { tag_name }
    }
}

impl Command for AddTagCommand {
    fn execute(&mut self, db: State<'_, DatabaseState>) -> Result<CommandOutput, Error> {
        sqlx::query!("INSERT INTO tag (name) VALUES ($1)", self.tag_name)
            .execute(&db.pool)
            .block_on()?;

        Ok(CommandOutput::None)
    }

    fn undo(&mut self, db: State<'_, DatabaseState>) -> Result<CommandOutput, Error> {
        sqlx::query!("DELETE FROM tag WHERE name = $1", self.tag_name)
            .execute(&db.pool)
            .block_on()?;

        Ok(CommandOutput::None)
    }
}

#[tauri::command]
pub async fn get_tags(state: tauri::State<'_, DatabaseState>) -> Result<Vec<Tag>, Error> {
    let tags = sqlx::query_as::<_, Tag>(
        "
        SELECT name FROM Tag;
        ",
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(tags)
}

#[tauri::command(rename_all = "snake_case")]
pub fn add_tag(
    db: tauri::State<'_, DatabaseState>,
    history: tauri::State<'_, Mutex<History>>,
    new_tag: String,
) -> Result<CommandOutput, Error> {
    let command = AddTagCommand::new(new_tag);

    let mut history_state = history.lock().unwrap();

    history_state.add_command(db, command)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_pattern_tags(
    state: tauri::State<'_, DatabaseState>,
    pattern_id: i32,
) -> Result<Vec<Tag>, Error> {
    let pool = &state.pool;

    let stmt = "
    SELECT t.name
    FROM patterns p
    JOIN tag_map tm ON p.id = tm.pattern_id
    JOIN tag t ON tm.tag_id = t.id
    WHERE p.id = ($1);
    ";

    let tags = sqlx::query_as::<_, Tag>(stmt)
        .bind(pattern_id)
        .fetch_all(pool)
        .await?;

    // println!("Get tags: {:#?}", tags);

    Ok(tags)
}
