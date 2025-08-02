-- SQLite

CREATE TABLE IF NOT EXISTS word (
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
);

