-- PostgreSQL

CREATE TABLE word (
	id SERIAL PRIMARY KEY,
	dutch_word TEXT NOT NULL,
	definite_article TEXT NOT NULL,
	english_translation TEXT NOT NULL,
	arabic_translation TEXT,
	source TEXT
);

CREATE TABLE sentence (
	id SERIAL PRIMARY KEY,
	word_id INTEGER NOT NULL,
	sentence TEXT NOT NULL,
	meaning TEXT NOT NULL,
	FOREIGN KEY (word_id) REFERENCES word(id)
);

CREATE TABLE note (
	id SERIAL PRIMARY KEY,
	word_id INTEGER NOT NULL,
	description TEXT NOT NULL,
	FOREIGN KEY (word_id) REFERENCES word(id)
);

CREATE TABLE tag (
	id SERIAL PRIMARY KEY,
	name TEXT UNIQUE NOT NULL
);

CREATE TABLE word_tag (
	word_id INTEGER NOT NULL,
	tag_id INTEGER NOT NULL,
	PRIMARY KEY (word_id, tag_id),
	FOREIGN KEY (word_id) REFERENCES word(id) ON DELETE CASCADE,
	FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE
);

