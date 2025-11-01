CREATE TABLE IF NOT EXISTS word (
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
);

