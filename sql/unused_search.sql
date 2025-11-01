SELECT DISTINCT w.*
FROM word w
JOIN sentence s ON s.word_id = w.id
JOIN note n ON n.word_id = w.id
JOIN word_tag wt ON wt.word_id = w.id
JOIN tag t ON t.id = wt.tag_id
WHERE
(
setweight(to_tsvector(coalesce(w.dutch_word, '')), 'A') ||
setweight(to_tsvector(coalesce(w.english_translation, '')), 'A') ||
setweight(to_tsvector(coalesce(w.arabic_translation, '')), 'B') ||
setweight(to_tsvector(coalesce(s.sentence, '')), 'C') ||
setweight(to_tsvector(coalesce(s.meaning, '')), 'C') ||
setweight(to_tsvector(coalesce(n.description, '')), 'D') ||
setweight(to_tsvector(coalesce(t.name, '')), 'D')
) @@ plainto_tsquery('your_keyword');

