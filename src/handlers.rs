use crate::prompt::run_verb_prompt;

use super::{
    error::AppError,
    types::{UpsertPayload, VerbExerciseOptions},
    utils::{
        create_sql_pool, get_upsert_payload_from_json, get_upsert_payload_from_prompt, get_verbs,
        parse_raw_upsert_payload, upsert_verb,
    },
};

pub async fn start_verb_exercise(options: VerbExerciseOptions) -> Result<(), AppError> {
    let pool = create_sql_pool().await?;
    let verbs = get_verbs(&pool, options).await?;

    run_verb_prompt(verbs)?;
    Ok(())
}

pub async fn handle_upsert_payload(payload: Option<String>) -> Result<(), AppError> {
    let parsed_payload: UpsertPayload;
    if let Some(raw_payload) = payload {
        let mut split = raw_payload.split(".");
        split.next();

        match split.next() {
            Some(_) => parsed_payload = get_upsert_payload_from_json(raw_payload)?,

            None => {
                parsed_payload = parse_raw_upsert_payload(raw_payload)?;
            }
        }
    } else {
        parsed_payload = get_upsert_payload_from_prompt()?;
    }

    let pool = create_sql_pool().await?;
    match parsed_payload {
        UpsertPayload::Verb(verbs) => {
            for v in verbs {
                upsert_verb(v, &pool).await?;
            }
        }
        UpsertPayload::Noun(_nouns) => {}
    }

    Ok(())
}
