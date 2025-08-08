use rusqlite::{Connection, Result, params};
use tauri::Manager;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Word {
    id: Option<i32>,
    dutch_word: String,
    definite_article: Option<String>,
    english_translation: String,
    arabic_translation: Option<String>,
    source: Option<String>,
    sentences: Vec<Sentence>,
    notes: Vec<String>,
    tags: Vec<String>
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Sentence {
    sentence: String,
    meaning: String
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_all_words(app_handle: tauri::AppHandle) -> Result<Vec<Word>, String> {
    get_all_words_inner(app_handle).map_err(|e| e.to_string())
}

fn get_all_words_inner(app_handle: tauri::AppHandle) -> Result<Vec<Word>> {
    let db_path = app_handle.state::<std::path::PathBuf>();
    let conn = Connection::open(db_path.inner())?;

    let mut stmt = conn.prepare("SELECT id, dutch_word, definite_article, english_translation, arabic_translation, source FROM word")?;

    let word_iter = stmt.query_map([], |row| {
        Ok(Word {
            id: row.get(0)?,
            dutch_word: row.get(1)?,
            definite_article: row.get(2)?,
            english_translation: row.get(3)?,
            arabic_translation: row.get(4)?,
            source: row.get(5)?,
            sentences: vec![],
            notes: vec![],
            tags: vec![],
        })
    })?;

    let mut words = Vec::new();

    for word_result in word_iter {
        let mut word = word_result?;

        // Get sentences
        let mut stmt = conn.prepare("SELECT sentence, meaning FROM sentence WHERE word_id = ?")?;
        let sentence_iter = stmt.query_map([word.id], |row| {
            Ok(Sentence {
                sentence: row.get(0)?,
                meaning: row.get(1)?,
            })
        })?;
        word.sentences = sentence_iter.filter_map(Result::ok).collect();

        // Get notes
        let mut stmt = conn.prepare("SELECT description FROM note WHERE word_id = ?")?;
        let note_iter = stmt.query_map([word.id], |row| {
            row.get::<_, String>(0)
        })?;
        word.notes = note_iter.collect::<Result<Vec<String>, _>>()?;

        // Get tags
        let mut stmt = conn.prepare("
            SELECT t.name FROM tag t
            INNER JOIN word_tag wt ON wt.tag_id = t.id
            WHERE wt.word_id = ?
        ")?;

        let tag_iter = stmt.query_map([word.id], |row| {
            row.get::<_, String>(0)
        })?;
        word.tags = tag_iter.collect::<Result<Vec<String>, _>>()?;

        words.push(word);
    }

    Ok(words)
}

#[tauri::command]
fn add_word(app_handle: tauri::AppHandle, word: Word) -> Result<(), String> {
    let db_path = app_handle.state::<std::path::PathBuf>();
    let mut conn = Connection::open(db_path.inner()).map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    tx.execute(
        "INSERT INTO word (dutch_word, definite_article, english_translation, arabic_translation, source)
        VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            word.dutch_word,
            word.definite_article,
            word.english_translation,
            word.arabic_translation,
            word.source
        ],
    ).map_err(|e| e.to_string())?;

    let word_id = tx.last_insert_rowid();

    for sentence in word.sentences {
        tx.execute(
            "INSERT INTO sentence (word_id, sentence, meaning) VALUES (?1, ?2, ?3)",
            params![word_id, sentence.sentence, sentence.meaning],
        ).map_err(|e| e.to_string())?;
    }

    for note in word.notes {
        tx.execute(
            "INSERT INTO note (word_id, description) VALUES (?1, ?2)",
            params![word_id, note],
        ).map_err(|e| e.to_string())?;
    }

    for tag in word.tags {
        tx.execute(
            "INSERT OR IGNORE INTO tag (name) VALUES (?1)",
            params![tag],
        ).map_err(|e| e.to_string())?;

        let tag_id: i64 = tx.query_row(
            "SELECT id FROM tag WHERE name = ?1",
            params![tag],
            |row| row.get(0),
        ).map_err(|e| e.to_string())?;

        tx.execute(
            "INSERT INTO word_tag (word_id, tag_id) VALUES (?1, ?2)",
            params![word_id, tag_id],
        ).map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_all_words, add_word])
        .setup(|app| {
            let db_path = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir")
                .join("rusty_words.db");

            let conn = Connection::open(&db_path).expect("Failed to open DB");

            conn.execute_batch(
                "CREATE TABLE IF NOT EXISTS word (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    dutch_word TEXT NOT NULL CHECK (dutch_word <> ''),
                    definite_article TEXT,
                    english_translation TEXT NOT NULL CHECK (english_translation <> ''),
                    arabic_translation TEXT,
                    source TEXT
                );

                CREATE TABLE IF NOT EXISTS sentence (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    word_id INTEGER NOT NULL,
                    sentence TEXT NOT NULL CHECK (sentence <> ''),
                    meaning TEXT NOT NULL CHECK (meaning <> ''),
                    FOREIGN KEY (word_id) REFERENCES word(id)
                );

                CREATE TABLE IF NOT EXISTS note (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    word_id INTEGER NOT NULL,
                    description TEXT NOT NULL CHECK (description <> ''),
                    FOREIGN KEY (word_id) REFERENCES word(id)
                );

                CREATE TABLE IF NOT EXISTS tag (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    name TEXT UNIQUE NOT NULL CHECK (name <> '')
                );

                CREATE TABLE IF NOT EXISTS word_tag (
                    word_id INTEGER NOT NULL,
                    tag_id INTEGER NOT NULL,
                    PRIMARY KEY (word_id, tag_id),
                    FOREIGN KEY (word_id) REFERENCES word(id) ON DELETE CASCADE,
                    FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE
                );"
            )?;

            // Store path globally so other commands can use it
            app.manage(db_path);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
