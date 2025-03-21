use crate::Error;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::db::*;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    pub name: String,
}

#[tauri::command]
pub async fn get_tags(state: tauri::State<'_, DatabaseState>) -> Result<Vec<Tag>, Error> {
    let tags = sqlx::query_as::<_, Tag>(
        "
        SELECT name FROM Tag;
        ",
    )
    .fetch_all(&state.0)
    .await?;

    Ok(tags)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn add_tag(state: tauri::State<'_, DatabaseState>, new_tag: String) -> Result<(), Error> {
    sqlx::query(
        "
        INSERT INTO tag (name) VALUES(?)
        ",
    )
    .bind(new_tag)
    .execute(&state.0)
    .await?;

    Ok(())
}
