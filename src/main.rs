use clap::Parser;

mod error;
mod handlers;
mod prompt;
mod types;
mod utils;

use error::*;
use handlers::{handle_upsert_payload, start_verb_exercise};
use types::{Cli, ExerciseType, Operation};
use utils::{init_database, run_migrations};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let Cli { operation, .. } = Cli::parse();

    init_database().await?;
    run_migrations().await?;

    match operation.unwrap_or(Operation::default()) {
        Operation::Exercise(ExerciseType::Verb { options }) => {
            start_verb_exercise(options).await?;
        }
        Operation::Upsert { payload } => {
            handle_upsert_payload(payload).await?;
        }
    }

    Ok(())
}
