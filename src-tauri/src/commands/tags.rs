use crate::Error;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::db::*;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Tag {
    #[sqlx(default)]
    pub id: Option<i32>,
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
        INSERT INTO tag (name) VALUES($1)
        ",
    )
    .bind(new_tag)
    .execute(&state.0)
    .await?;

    Ok(())
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_pattern_tags(
    state: tauri::State<'_, DatabaseState>,
    pattern_id: i32,
) -> Result<Vec<Tag>, Error> {
    let pool = &state.0;

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

    println!("Get tags: {:#?}", tags);

    Ok(tags)
}
