use std::{env, fs};

use anyhow::Result;
use sqlx::{Pool, Sqlite, SqlitePool};
use tauri::{AppHandle, Manager};

pub struct Database {
    pub pool: Pool<Sqlite>,
}

impl Database {
    pub async fn new(app_handle: &AppHandle) -> Result<Self> {
        let app_dir = app_handle
            .path()
            .app_data_dir()
            .expect("Failed to get app dir");

        // ensure app directoy exists
        fs::create_dir_all(&app_dir)?;

        let db_path = app_dir.join("embroidery.db");

        // set DATABASE_URL env var to point to this SQLite file
        env::set_var("DATABASE_URL", format!("sqlite://{}", db_path.display()));

        let connection_options = sqlx::sqlite::SqliteConnectOptions::new()
            .filename(&db_path)
            .create_if_missing(true)
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);

        let pool = SqlitePool::connect_with(connection_options).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }
}

pub struct DatabaseState {
    pub pool: Pool<Sqlite>,
}
