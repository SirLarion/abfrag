use clap::{Args, Parser, Subcommand};
use core::fmt::Debug;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqliteRow, Row};

use crate::utils::split_opt_string;

pub trait Word: Debug {
    fn empty() -> Self
    where
        Self: Sized;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Gender {
    Feminine,
    Masculine,
    Neutral,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, FromRow)]
pub struct Noun {
    pub de: String,
    pub de_expanded: Option<String>,
    pub de_examples: Option<Vec<String>>,
    pub en: String,
    pub en_expanded: Option<String>,
    pub en_examples: Option<Vec<String>>,
    pub gender: Gender,
    pub freq_percentile: f32,
}

impl Word for Noun {
    fn empty() -> Self {
        Self {
            de: String::new(),
            de_expanded: None,
            de_examples: None,
            en: String::new(),
            en_expanded: None,
            en_examples: None,
            gender: Gender::Neutral,
            freq_percentile: 0.0,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, sqlx::Type, sqlx::Encode)]
pub struct Verb {
    pub de: String,
    pub de_expanded: Option<String>,
    pub de_examples: Option<Vec<String>>,
    pub de_forms: Vec<String>,
    pub en: String,
    pub en_expanded: Option<String>,
    pub en_examples: Option<Vec<String>>,
    pub irregular: bool,
    pub freq_percentile: f32,
}

impl FromRow<'_, SqliteRow> for Verb {
    fn from_row(row: &SqliteRow) -> sqlx::Result<Self> {
        Ok(Self {
            de: row.try_get("de")?,
            de_expanded: row.try_get("de_expanded")?,
            de_examples: split_opt_string(row.try_get::<Option<String>, _>("de_examples")?),
            de_forms: row
                .try_get::<String, _>("de_forms")?
                .split(";")
                .map(|s| s.to_string())
                .collect::<Vec<String>>(),
            en: row.try_get("en")?,
            en_expanded: row.try_get("en_expanded")?,
            en_examples: split_opt_string(row.try_get::<Option<String>, _>("en_examples")?),
            irregular: row.try_get("irregular")?,
            freq_percentile: row.try_get("freq_percentile")?,
        })
    }
}

impl Word for Verb {
    fn empty() -> Self {
        Self {
            de: String::new(),
            de_expanded: None,
            de_examples: None,
            de_forms: Vec::new(),
            en: String::new(),
            en_expanded: None,
            en_examples: None,
            irregular: false,
            freq_percentile: 0.0,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum UpsertPayload {
    Noun(Vec<Noun>),
    Verb(Vec<Verb>),
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(next_line_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub operation: Option<Operation>,

    /// Run command verbosely
    #[arg(long, default_value_t = false)]
    pub verbose: bool,

    /// Turn debugging information on
    #[arg(short, long, default_value_t = false)]
    pub debug: bool,
}

#[derive(Subcommand)]
pub enum Operation {
    #[command(subcommand)]
    Exercise(ExerciseType),

    Upsert {
        #[arg()]
        payload: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ExerciseType {
    Verb {
        #[command(flatten)]
        options: VerbExerciseOptions,
    },
}

impl Default for Operation {
    fn default() -> Self {
        Operation::Exercise(ExerciseType::Verb {
            options: VerbExerciseOptions {
                irregular: true,
                freq_bias: true,
                word_amount: 10,
            },
        })
    }
}

#[derive(Args)]
pub struct VerbExerciseOptions {
    #[arg(short, long, default_value_t = true)]
    pub irregular: bool,

    #[arg(long, default_value_t = false)]
    pub freq_bias: bool,

    #[arg(short, long, default_value_t = 10)]
    pub word_amount: i32,
}
