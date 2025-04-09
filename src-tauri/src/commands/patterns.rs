use crate::Error;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::db::*;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Pattern {
    pub id: u32,
    pub name: String,
    pub pattern_num: Option<u32>,
    pub thread_count: Option<u32>,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_patterns(
    state: tauri::State<'_, DatabaseState>,
    query_tags: Vec<String>,
) -> Result<Vec<Pattern>, Error> {
    let pool = &state.0;

    if query_tags.is_empty() {
        return get_all_patterns(state).await;
    }

    let params = (1..=query_tags.len())
        .map(|i| format!("${}", i))
        .collect::<Vec<String>>()
        .join(", ");

    let stmt = format!(
        "
        SELECT p.*
        FROM tag_map pt, patterns p, tag t
        WHERE pt.tag_id = t.id
        AND (t.name IN ( {} ))
        AND p.id = pt.pattern_id
        GROUP BY p.id;
        ",
        params
    );

    let mut query = sqlx::query_as::<_, Pattern>(&stmt);
    for tag in query_tags {
        query = query.bind(tag);
    }

    let rows = query.fetch_all(pool).await?;

    // println!("{:#?}", rows);

    Ok(rows)
}

async fn get_all_patterns(state: tauri::State<'_, DatabaseState>) -> Result<Vec<Pattern>, Error> {
    let pool = &state.0;

    let stmt = "SELECT * FROM patterns;";

    let patterns = sqlx::query_as::<_, Pattern>(stmt).fetch_all(pool).await?;

    Ok(patterns)
}
