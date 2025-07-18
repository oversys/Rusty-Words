use rusqlite::{Connection, Result};

#[derive(serde::Serialize)]
struct Word {
    id: i32,
    dutch_word: String,
    definite_article: Option<String>,
    english_translation: String,
    arabic_translation: Option<String>,
    source: Option<String>,
    sentences: Vec<Sentence>,
    notes: Vec<String>,
    tags: Vec<String>
}

#[derive(serde::Serialize)]
struct Sentence {
    sentence: String,
    meaning: String
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_all_words() -> Result<Vec<Word>, String> {
    get_all_words_inner().map_err(|e| e.to_string())
}

fn get_all_words_inner() -> Result<Vec<Word>> {
    let conn = Connection::open("rusty_words.db")?;

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

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_all_words])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
