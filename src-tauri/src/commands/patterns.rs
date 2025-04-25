use crate::commands::Pattern;
use crate::db::*;
use crate::Error;
use serde::{Deserialize, Serialize};
use sqlx::{QueryBuilder, Sqlite};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub include_tags: Vec<String>,
    pub exclude_tags: Vec<String>,
    pub custom_query: String,
}

#[tauri::command(rename_all = "snake_case")]
pub async fn search_patterns(
    state: tauri::State<'_, DatabaseState>,
    search_query: SearchQuery,
) -> Result<Vec<Pattern>, Error> {
    let mut query_builder: QueryBuilder<Sqlite> = QueryBuilder::new(
        "SELECT p.* FROM patterns p, tag_map tm, tag t WHERE p.id = tm.pattern_id AND tm.tag_id = t.id "
    );
    
    let mut include_length: i32 = 0;
    
    let mut seperated = query_builder.separated(", ");

    if search_query.include_tags.len() > 0 {
        include_length = search_query.include_tags.len() as i32;
        
        seperated.push_unseparated("AND (t.name IN (");

        for tag in search_query.include_tags {
            seperated.push_bind(tag);
        }

        seperated.push_unseparated(")) ");
    }

    if search_query.exclude_tags.len() > 0 {
        seperated.push_unseparated(
            "AND p.id NOT IN (SELECT p.id
                   FROM patterns p,
                        tag_map tm,
                        tag t
                   WHERE p.id = tm.pattern_id
                     AND tm.tag_id = t.id
                     AND (t.name IN (",
        );

        for tag in search_query.exclude_tags {
            seperated.push_bind(tag);
        }

        seperated.push_unseparated(")) ");
    }

    seperated.push_unseparated("GROUP BY p.id ");
    
    if include_length > 0 {
        seperated.push_unseparated("HAVING COUNT(p.id) = ");
        seperated.push_bind_unseparated(include_length);
    }
    
    let query = query_builder.build_query_as::<Pattern>();
    
    let patterns = query.fetch_all(&state.pool).await?;
    
    Ok(patterns)
}

#[tauri::command(rename_all = "snake_case")]
pub async fn get_patterns(
    state: tauri::State<'_, DatabaseState>,
    query_tags: Vec<String>,
) -> Result<Vec<Pattern>, Error> {
    let pool = &state.pool;

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
    let pool = &state.pool;

    let stmt = "SELECT * FROM patterns;";

    let patterns = sqlx::query_as::<_, Pattern>(stmt).fetch_all(pool).await?;

    Ok(patterns)
}
