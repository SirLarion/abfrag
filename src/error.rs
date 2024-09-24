use std::{env, io, num};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("executing command failed: {0}")]
    CmdError(String),

    #[error(transparent)]
    IoError(#[from] io::Error),

    #[error(transparent)]
    EnvError(#[from] env::VarError),

    #[error(transparent)]
    SerdeError(#[from] serde_json::Error),

    #[error(transparent)]
    PromptError(#[from] inquire::InquireError),

    #[error(transparent)]
    ParseFloatError(#[from] num::ParseFloatError),

    #[error(transparent)]
    SqlError(#[from] sqlx::error::Error),

    #[error(transparent)]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error(transparent)]
    CsvError(#[from] csv::Error),
}
