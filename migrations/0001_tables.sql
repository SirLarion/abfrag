CREATE TABLE IF NOT EXISTS verbs (
    id INTEGER PRIMARY KEY,
    de TEXT NOT NULL UNIQUE,
    de_expanded TEXT,
    de_examples BLOB,
    de_forms BLOB NOT NULL,
    en TEXT NOT NULL UNIQUE,
    en_expanded TEXT,
    en_examples BLOB,
    irregular BOOLEAN NOT NULL,
    freq_percentile REAL NOT NULL
);

CREATE TABLE IF NOT EXISTS nouns (
    id INTEGER PRIMARY KEY,
    de TEXT NOT NULL UNIQUE,
    de_expanded TEXT,
    de_examples BLOB,
    de_forms BLOB NOT NULL,
    en TEXT NOT NULL UNIQUE,
    en_expanded TEXT,
    en_examples BLOB,
    gender TEXT NOT NULL,
    freq_percentile REAL NOT NULL
);
