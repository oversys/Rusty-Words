use rusqlite::{params, Connection, Result};
use tauri::Manager;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Word {
    id: Option<i32>,
    dutch_word: String,
    r#type: String,
    definite_article: Option<String>,
    preposition: Option<String>,
    source: Option<String>,
    translations: Vec<Translation>,
    conjugation: Option<Conjugation>,
    sentences: Vec<Sentence>,
    notes: Vec<String>,
    tags: Vec<Tag>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Translation {
    translation: String,
    language: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Conjugation {
    present_ik: String,
    present_jij: String,
    present_u: String,
    present_hij_zij_het: String,
    present_plural: String,
    imperfectum_singular: String,
    imperfectum_plural: String,
    perfectum: Option<String>,
    perfectum_auxiliary_verb: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Sentence {
    sentence: String,
    meaning: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Tag {
    id: i32,
    name: String,
}

#[tauri::command]
fn get_all_words(app_handle: tauri::AppHandle) -> Result<Vec<Word>, String> {
    get_all_words_inner(app_handle).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_tags(app_handle: tauri::AppHandle) -> Result<Vec<Tag>, String> {
    get_tags_inner(app_handle).map_err(|e| e.to_string())
}

fn get_tags_inner(app_handle: tauri::AppHandle) -> Result<Vec<Tag>> {
    let db_path = app_handle.state::<std::path::PathBuf>();
    let conn = Connection::open(db_path.inner())?;

    // Get all tags
    let mut stmt = conn.prepare("SELECT id, name FROM tag")?;

    let tag_iter = stmt.query_map([], |row| {
        Ok(Tag {
            id: row.get(0)?,
            name: row.get(1)?,
        })
    })?;

    Ok(tag_iter.filter_map(Result::ok).collect())
}

fn get_all_words_inner(app_handle: tauri::AppHandle) -> Result<Vec<Word>> {
    let db_path = app_handle.state::<std::path::PathBuf>();
    let conn = Connection::open(db_path.inner())?;

    let mut stmt = conn.prepare("SELECT id, dutch_word, type, definite_article, preposition, source FROM word")?;

    let word_iter = stmt.query_map([], |row| {
        Ok(Word {
            id: row.get(0)?,
            dutch_word: row.get(1)?,
            r#type: row.get(2)?,
            definite_article: row.get(3)?,
            preposition: row.get(4)?,
            source: row.get(5)?,
            translations: vec![],
            conjugation: None,
            sentences: vec![],
            notes: vec![],
            tags: vec![],
        })
    })?;

    let mut words = Vec::new();

    for word_result in word_iter {
        let mut word = word_result?;

        // Get translations
        let mut stmt = conn.prepare("SELECT translation, language FROM translation WHERE word_id = ? ORDER BY language DESC")?;
        let translation_iter = stmt.query_map([word.id], |row| {
            Ok(Translation {
                translation: row.get(0)?,
                language: row.get(1)?,
            })
        })?;
        word.translations = translation_iter.filter_map(Result::ok).collect();

        // Get conjugation
        if word.r#type == "verb" || word.r#type == "separable verb" {
            let mut stmt = conn.prepare("SELECT present_ik, present_jij, present_u, present_hij_zij_het, present_plural, imperfectum_singular, imperfectum_plural, perfectum, perfectum_auxiliary_verb FROM conjugation WHERE word_id = ?")?;

            let conjugation_row = stmt.query_row([word.id], |row| {
                Ok(Conjugation {
                    present_ik: row.get(0)?,
                    present_jij: row.get(1)?,
                    present_u: row.get(2)?,
                    present_hij_zij_het: row.get(3)?,
                    present_plural: row.get(4)?,
                    imperfectum_singular: row.get(5)?,
                    imperfectum_plural: row.get(6)?,
                    perfectum: row.get(7)?,
                    perfectum_auxiliary_verb: row.get(8)?,
                })
            })?;

            word.conjugation = Some(conjugation_row);
        }

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
        let note_iter = stmt.query_map([word.id], |row| row.get::<_, String>(0))?;
        word.notes = note_iter.collect::<Result<Vec<String>, _>>()?;

        // Get tags
        let mut stmt = conn.prepare(
            "SELECT t.name FROM tag t
            INNER JOIN word_tag wt ON wt.tag_id = t.id
            WHERE wt.word_id = ?")?;

        let tag_iter = stmt.query_map([word.id], |row| {
            Ok(Tag {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?;
        word.tags = tag_iter.filter_map(Result::ok).collect();

        words.push(word);
    }

    Ok(words)
}

#[tauri::command]
fn add_word(app_handle: tauri::AppHandle, word: Word) -> Result<(), String> {
    add_word_inner(app_handle, word).map_err(|e| e.to_string())
}

fn add_word_inner(app_handle: tauri::AppHandle, word: Word) -> Result<()> {
    let db_path = app_handle.state::<std::path::PathBuf>();
    let mut conn = Connection::open(db_path.inner())?;
    let tx = conn.transaction()?;

    // Insert word
    tx.execute(
        "INSERT INTO word (dutch_word, type, definite_article, preposition, source)
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            word.dutch_word,
            word.r#type,
            word.definite_article,
            word.preposition,
            word.source,
        ],
    )?;

    let word_id = tx.last_insert_rowid();

    // Insert translations
    for translation in word.translations {
        tx.execute(
            "INSERT INTO translation (word_id, translation, language)
VALUES (?1, ?2, ?3)",
            params![word_id, translation.translation, translation.language],
        )?;
    }

    // Insert conjugation (if present)
    if let Some(conjugation) = word.conjugation {
        tx.execute(
            "INSERT INTO conjugation (
                word_id, present_ik, present_jij, present_u, present_hij_zij_het,
                present_plural, imperfectum_singular, imperfectum_plural,
                perfectum, perfectum_auxiliary_verb
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                word_id,
                conjugation.present_ik,
                conjugation.present_jij,
                conjugation.present_u,
                conjugation.present_hij_zij_het,
                conjugation.present_plural,
                conjugation.imperfectum_singular,
                conjugation.imperfectum_plural,
                conjugation.perfectum,
                conjugation.perfectum_auxiliary_verb,
            ],
        )?;
    }

    // Insert sentences
    for sentence in word.sentences {
        tx.execute(
            "INSERT INTO sentence (word_id, sentence, meaning) VALUES (?1, ?2, ?3)",
            params![word_id, sentence.sentence, sentence.meaning],
        )?;
    }

    // Insert notes
    for note in word.notes {
        tx.execute(
            "INSERT INTO note (word_id, description) VALUES (?1, ?2)",
            params![word_id, note],
        )?;
    }

    // Insert tags
    for tag in word.tags {
        let tag_id = if tag.id != -1 {
            // Tag already exists
            tag.id as i32
        } else {
            // Create new tag
            tx.execute("INSERT INTO tag (name) VALUES (?1)", params![tag.name])?;

            tx.last_insert_rowid() as i32
        };

        // Associate tag with word
        tx.execute(
            "INSERT INTO word_tag (word_id, tag_id) VALUES (?1, ?2)",
            params![word_id, tag_id],
        )?;
    }

    tx.commit()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_all_words, get_tags, add_word])
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
                    dutch_word TEXT UNIQUE NOT NULL CHECK (dutch_word <> ''),
                    type TEXT NOT NULL CHECK (type <> '' AND type IN ('noun', 'verb', 'separable verb', 'adjective', 'adverb', 'pronoun', 'preposition', 'conjunction', 'interjection', 'not given')),
                    definite_article TEXT,
                    preposition TEXT,
                    source TEXT
                );

                CREATE TABLE IF NOT EXISTS translation (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    word_id INTEGER NOT NULL,
                    translation TEXT NOT NULL CHECK (translation <> ''),
                    language TEXT NOT NULL CHECK (language <> ''),
                    FOREIGN KEY (word_id) REFERENCES word(id)
                );

                CREATE TABLE IF NOT EXISTS conjugation (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    word_id INTEGER NOT NULL,
                    present_ik TEXT NOT NULL CHECK (present_ik <> ''),
                    present_jij TEXT NOT NULL CHECK (present_jij <> ''),
                    present_u TEXT NOT NULL CHECK (present_u <> ''),
                    present_hij_zij_het TEXT NOT NULL CHECK (present_hij_zij_het <> ''),
                    present_plural TEXT NOT NULL CHECK (present_plural <> ''),
                    imperfectum_singular TEXT NOT NULL CHECK (imperfectum_singular <> ''),
                    imperfectum_plural TEXT NOT NULL CHECK (imperfectum_plural <> ''),
                    perfectum TEXT,
                    perfectum_auxiliary_verb TEXT CHECK (perfectum_auxiliary_verb IN ('hebben', 'zijn')),
                    FOREIGN KEY (word_id) REFERENCES word(id)
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
        .expect("Error while running tauri application");
}
