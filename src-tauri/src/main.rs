// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{Connection, Result};

fn main() -> Result<()> {
    let conn = Connection::open("rusty_words.db")?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS word (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            dutch_word TEXT NOT NULL,
            definite_article TEXT,
            english_translation TEXT NOT NULL,
            arabic_translation TEXT,
            source TEXT
        );
            
        CREATE TABLE IF NOT EXISTS sentence (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word_id INTEGER NOT NULL,
            sentence TEXT NOT NULL,
            meaning TEXT NOT NULL,
            FOREIGN KEY (word_id) REFERENCES word(id)
        );
            
        CREATE TABLE IF NOT EXISTS note (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            word_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            FOREIGN KEY (word_id) REFERENCES word(id)
        );
            
        CREATE TABLE IF NOT EXISTS tag (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL
        );
            
        CREATE TABLE IF NOT EXISTS word_tag (
            word_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (word_id, tag_id),
            FOREIGN KEY (word_id) REFERENCES word(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE
        );"
    )?;

    rusty_words_lib::run();

    Ok(())
}
