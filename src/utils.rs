use std::{
    env,
    fs::{self, File},
    path::Path,
};

use sqlx::{Sqlite, SqlitePool};

use crate::{
    error::AppError,
    types::{UpsertPayload, Verb, VerbExerciseOptions, Word},
};
pub fn build_db_path() -> Result<String, AppError> {
    let home_var = env::var("HOME");
    let dir: String;

    match home_var {
        Ok(home) => dir = format!("{home}/.local/state/abfrag"),
        Err(e) => return Err(e.into()),
    }

    if !Path::new(&dir).exists() {
        fs::create_dir(&dir)?;
    }

    Ok(format!("{dir}/db.sqlite"))
}

pub fn split_opt_string(opt_str: Option<String>) -> Option<Vec<String>> {
    opt_str.map(|opt| {
        opt.split(";")
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
    })
}

pub async fn create_sql_pool() -> Result<SqlitePool, AppError> {
    let pool = SqlitePool::connect(&build_db_path()?).await?;

    Ok(pool)
}

pub async fn init_database() -> Result<(), AppError> {
    if let Err(AppError::SqlError(e)) = create_sql_pool().await {
        if e.to_string().contains("unable to open database file") {
            File::create(build_db_path()?)?;
        }
    };

    Ok(())
}

pub async fn run_migrations() -> Result<(), AppError> {
    let pool = create_sql_pool().await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(())
}

pub fn get_upsert_payload_from_prompt() -> Result<UpsertPayload, AppError> {
    Ok(UpsertPayload::Verb(vec![Verb::empty()]))
}

pub fn get_upsert_payload_from_json(file: String) -> Result<UpsertPayload, AppError> {
    let file_content = fs::read_to_string(file.clone())?;
    let mut split = file.split(".");
    split.next();

    let mut payload: UpsertPayload = UpsertPayload::Verb(vec![]);
    match split.next() {
        Some("json") => payload = serde_json::from_str::<UpsertPayload>(&file_content)?,
        Some(_) | None => Err(AppError::CmdError("unrecognized filetype".to_string()))?,
    }

    Ok(payload)
}

pub fn parse_raw_upsert_payload(raw_payload: String) -> Result<UpsertPayload, AppError> {
    Ok(UpsertPayload::Verb(vec![Verb::empty()]))
}

pub async fn get_verbs(
    pool: &SqlitePool,
    options: VerbExerciseOptions,
) -> Result<Vec<Verb>, AppError> {
    let res = sqlx::query_as::<_, Verb>(
        "SELECT 
            de, 
            de_expanded, 
            de_examples, 
            de_forms, 
            en, 
            en_expanded, 
            en_examples, 
            irregular, 
            freq_percentile
        FROM verbs 
        WHERE irregular = ?1
        ORDER BY RANDOM()
        LIMIT ?2;",
    )
    .bind(options.irregular)
    .bind(options.word_amount)
    .fetch_all(pool)
    .await?;

    Ok(res)
}

pub async fn upsert_verb(v: Verb, pool: &SqlitePool) -> Result<(), AppError> {
    sqlx::query::<Sqlite>(
        "REPLACE INTO verbs (
            de, 
            de_expanded, 
            de_examples, 
            de_forms, 
            en, 
            en_expanded, 
            en_examples, 
            irregular, 
            freq_percentile
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
    )
    .bind(v.de)
    .bind(v.de_expanded)
    .bind(v.de_examples.map(|e| e.join(";")))
    .bind(v.de_forms.join(";"))
    .bind(v.en)
    .bind(v.en_expanded)
    .bind(v.en_examples.map(|e| e.join(";")))
    .bind(v.irregular)
    .bind(v.freq_percentile)
    .execute(pool)
    .await
    .map_err(|e| {
        println!("{:?}", e);
        e
    })?;

    Ok(())
}
