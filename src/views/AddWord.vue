<script>
import { invoke } from "@tauri-apps/api/core";
import { message } from "@tauri-apps/plugin-dialog";

export default {
	data() {
		return {
			word: {
				dutchWord: "",
				type: "",
				definiteArticle: "",
				plural: "",
				preposition: "",
				source: "",
				translations: [{ translation: "", language: "English" }],
				conjugation: {
					presentIk: "",
					presentJij: "",
					presentU: "",
					presentHijZijHet: "",
					presentPlural: "",
					imperfectumSingular: "",
					imperfectumPlural: "",
					perfectum: "",
					perfectumAuxiliaryVerb: ""
				},
				sentences: [{ sentence: "", meaning: "" }],
				notes: [""],
				tags: []
			},

			availableWordTypes: ["noun", "verb", "separable verb", "adjective", "adverb", "suffix", "pronoun", "preposition", "conjunction", "interjection", "not given"],
			definiteArticles: ["de", "het", "de/het"],
			languageOptions: ["English", "Arabic"],
			availableTags: [],
			selectedTagIds: [""],

			conjugationLabels: {
				presentIk: "Present, ik",
				presentJij: "Present, jij",
				presentU: "Present, u",
				presentHijZijHet: "Present, hij/zij/het",
				presentPlural: "Present, plural",
				imperfectumSingular: "Imperfectum, singular",
				imperfectumPlural: "Imperfectum, plural",
				perfectum: "Perfectum"
			},

			wordTimeout: null,
			wordExists: false,
			words: null
		}
	},
	methods: {
		addTranslation() {
			this.word.translations.push({ translation: "", language: "" });
		},

		addSentence() {
			this.word.sentences.push({ sentence: "", meaning: "" });
		},

		addNote() {
			this.word.notes.push("");
		},

		addTag() {
			this.word.tags.push({id: -1, name: ""});
		},

		onTagSelect() {
			const newTags = this.word.tags.filter(t => t && t.id === -1);

			// Get full tags (id and name)
			const existingSelected = this.selectedTagIds
			.map(id => this.availableTags.find(tag => tag.id === id))
			.filter(tag => tag) // remove null/undefined
			.map(tag => ({ id: tag.id, name: tag.name }));

			this.word.tags = [...existingSelected, ...newTags];

			// Find index of the last non-empty dropdown
			const lastSelectedIndex = this.selectedTagIds.map(Boolean).lastIndexOf(true);

			// Nothing selected
			if (lastSelectedIndex === -1) {
				this.selectedTagIds = [""];
				return;
			}

			// Keep selections up to lastSelectedIndex
			const kept = this.selectedTagIds.slice(0, lastSelectedIndex + 1);

			// Find remaining unchosen tags
			const remainingTags = this.availableTags.filter(
				t => !existingSelected.some(sel => sel.id === t.id)
			);

			// Add one empty dropdown if there are still unchosen tags and the last kept isn't already empty
			if (remainingTags.length > 0 && kept.at(-1) !== "") {
				kept.push("");
			} else if (kept.at(-1) === "") {
				// If there are no remaining tags, remove unnecessary trailing empty
				kept.pop();
			}

			this.selectedTagIds = kept;
		},

		async addWord() {
			const wordCopy = {...this.word};

			try {
				if (wordCopy.type !== 'verb' && wordCopy.type !== 'separable verb') {
					delete wordCopy.conjugation;
				}

				const cleanedWord = {
					...wordCopy,
					translations: wordCopy.translations.filter(t => t.translation.trim() && t.language.trim()),
					sentences: wordCopy.sentences.filter(s => s.sentence.trim() && s.meaning.trim()),
					notes: wordCopy.notes.filter(n => n.trim()),
					tags: wordCopy.tags.filter(t => t.name.trim())
				};

				await invoke("add_word", { word: cleanedWord });
				await message("Word added successfully!", { title: "Add word", kind: "info" });

				this.$router.push("/");
			} catch (e) {
				console.error(e);
				await message("Failed to add word.", { title: "Add word", kind: "error" });
			}
		}
	},

	watch: {
		"word.dutchWord"(newValue) {
			clearTimeout(this.wordTimeout);

			if (!newValue) {
				this.wordExists = false;
				return
			}

			this.wordTimeout = setTimeout(() => {
				this.wordExists = this.words.some(w => w.dutchWord === this.word.dutchWord);
			}, 2000);
		}
	},

	async mounted() {
		this.availableTags = await invoke("get_tags");
		this.words = await invoke("get_all_words");
	}
}
</script>

<template>
	<div class="main-container">
		<!-- Dutch word -->
		<div class="field-container">
			<p>Dutch Word</p>
			<div class="box-container">
				<input v-model="word.dutchWord" />
			</div>

			<!-- Word exists warning -->
			<div v-if="wordExists" class="word-exists-warning">
				Note: Word already exists in dictionary.
			</div>
		</div>

		<!-- Word type -->
		<div class="field-container">
			<p>Word Type</p>

			<select v-model="word.type" class="spacing-bottom">
				<option disabled value="">Select word type</option>
				<option v-for="(type, index) in availableWordTypes" :key="index" :value="type">{{ type }}</option>
			</select>
		</div>

		<!-- Definite article (for nouns) -->
		<div v-if="word.type == 'noun'" class="field-container spacing-bottom">
			<p>Definite Article</p>

			<select v-model="word.definiteArticle">
				<option disabled value="">Select definite article</option>
				<option v-for="(article, index) in definiteArticles" :key="index" :value="article">{{ article }}</option>
			</select>
		</div>

		<!-- Plural (for most nouns) -->
		<div v-if="word.type == 'noun'" class="field-container">
			<p>Plural</p>

			<div class="box-container">
				<input v-model="word.plural" placeholder="Leave empty for no plural" />
			</div>
		</div>

		<!-- Separable prefix (for separable verbs) -->
		<div v-if="word.type == 'separable verb'" class="field-container">
			<p>Separable prefix</p>

			<div class="box-container">
				<input v-model="word.preposition" />
			</div>
		</div>

		<!-- Source -->
		<div class="field-container">
			<p>Source</p>

			<div class="box-container">
				<input v-model="word.source" />
			</div>
		</div>

		<!-- Translations -->
		<div class="field-container">
			<p>Translations</p>

			<div v-for="(translation, index) in word.translations" :key="index" class="translation-group">
				<div class="box-container" style="margin-bottom: 1rem;">
					<input
						v-model="translation.translation"
						:disabled="!translation.language"
						:dir="translation.language === 'Arabic' ? 'rtl' : 'ltr'"
						:class="{
							rtl: translation.language === 'Arabic',
							'disabled-input': !translation.language
						}"
						:placeholder="translation.language === 'Arabic' ? 'الترجمة' : 'Translation'"
					/>
				</div>

				<select v-model="translation.language">
					<option disabled value="">Select language</option>
					<option v-for="(lang, idx) in languageOptions" :key="idx" :value="lang">{{ lang }}</option>
				</select>
			</div>
			<button @click="addTranslation" class="spacing-top">+ Add Translation</button>
		</div>

		<!-- Sentences -->
		<div class="field-container">
			<p>Sentences</p>

			<div v-for="(sentence, index) in word.sentences" :key="index" class="sentence-group">
				<div class="box-container" style="margin-bottom: 1rem;">
					<input v-model="sentence.sentence" placeholder="Sentence" />
				</div>

				<div class="box-container" style="margin: 0;">
					<input v-model="sentence.meaning" placeholder="Meaning" />
				</div>
			</div>

			<button @click="addSentence" class="spacing-top">+ Add Sentence</button>
		</div>

		<!-- Notes -->
		<div class="field-container">
			<p>Notes</p>

			<div v-for="(note, index) in word.notes" :key="index" class="box-container">
				<input v-model="word.notes[index]" />
			</div>

			<button @click="addNote">+ Add Note</button>
		</div>

		<!-- Tags -->
		<div class="field-container">
			<p>Tags</p>

			<!-- Dropdowns for existing tags -->
			<select
				v-for="(selectedId, index) in selectedTagIds"
				:key="'tag-select-' + index"
				class="spacing-bottom"
				v-model="selectedTagIds[index]"
				@change="onTagSelect"
			>
				<option value="">Select tag</option>
				<option
					v-for="tag in availableTags.filter(availableTag => !word.tags.some(selectedTag => selectedTag.id === availableTag.id) || availableTag.id == selectedId)"
					:key="tag.id"
					:value="tag.id"
				>
					{{ tag.name }}
				</option>
			</select>

			<!-- New tag inputs (id === -1) -->
			<div v-for="(tag, index) in word.tags" :key="'new-tag-' + index">
				<div v-if="tag.id === -1" class="box-container">
					<input v-model="tag.name" placeholder="New Tag" />
				</div>
			</div>

			<button @click="addTag">+ Add Tag</button>
		</div>

		<!-- Conjugation (for verbs and separable verbs) -->
		<div v-if="word.type == 'verb' || word.type == 'separable verb'" class="field-container">
			<p>Conjugation</p>

			<div class="conjugation-grid">
				<div class="grid-row" v-for="(label, key) in conjugationLabels" :key="key">
					<label>{{ label }}</label>
					<input v-model="word.conjugation[key]" />
				</div>

				<div class="grid-row">
					<label>Perfectum Auxiliary Verb</label>
					<select v-model="word.conjugation.perfectumAuxiliaryVerb">
						<option disabled value="">Select</option>
						<option value="hebben">hebben</option>
						<option value="zijn">zijn</option>
					</select>
				</div>
			</div>
		</div>

		<!-- Buttons -->
		<div class="buttons-container">
			<RouterLink to="/" class="cancel-btn">Cancel</RouterLink>
			<button @click="addWord" class="save-btn">Save</button>
		</div>
	</div>
</template>

<style scoped>
.main-container {
	padding: 1rem;
	margin-bottom: 3rem;
	display: flex;
	flex-direction: column;
}

.field-container {
	margin-top: 1rem;
}

.field-container p {
	font-size: 1.5rem;
	margin: 0 0 0.5rem 0.1rem;
}

.field-container button {
	all: unset;
	color: #77654D;
	font-size: 1.35rem;
	cursor: pointer;
	margin-bottom: 1.5rem;
}

.spacing-bottom {
	margin-bottom: 1.5rem;
}

.spacing-top {
	margin-top: 1.5rem !important;
}

.word-exists-warning {
	font-size: 1.35rem;
	font-style: italic;
	color: #A0A0A0;
	background: #EDEAE1;
	border: 1.5px dashed #D4CDC3;
	border-radius: 0.75rem;
	padding: 0.75rem 0;
	margin-bottom: 1.5rem;
	text-align: center;
}

input,
select {
	all: unset;
	box-sizing: border-box;
	width: 100%;
	font-size: 1.35rem;
	font-family: "Helvetica Neue", RB;
	border-radius: 0.75rem;
	border: 1.75px solid #D4CDC3;
	background: #FEFEFA;
	padding: 1rem;
}

input:focus,
select:focus {
	outline: none;
	border-color: #263543;
	box-shadow: 0 0 0 2px rgba(38, 53, 67, 0.2);
}

select {
	background: #FEFEFA url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='10' height='6' viewBox='0 0 10 6'%3E%3Cpath fill='%2366654D' d='M0 0l5 6 5-6z'/%3E%3C/svg%3E") no-repeat right 1rem center;
}

.disabled-input {
	background: #EDEAE1;
	color: #A0A0A0;
	cursor: not-allowed;
}

.rtl {
	direction: rtl;
	font-family: RB;
	padding: 0.48rem 1rem;
	cursor: text;
}

.box-container {
	margin: 0 0 1.5rem 0;
}

.translation-group,
.sentence-group,
.conjugation-grid {
	background: #F2EEE6;
	border-radius: 0.75rem;
	border: 1.75px solid #E0DBCE;
	padding: 0.75rem;
	margin-top: 1rem;
}

.conjugation-grid {
	display: grid;
	grid-template-columns: 1fr 2fr;
	gap: 0.75rem 1rem;
	padding: 1rem;
}

.grid-row {
	display: contents;
}

.grid-row label {
	font-size: 1.35rem;
	padding: 0.75rem 0;
	font-style: italic;
	align-self: center;
}

.buttons-container {
	display: flex;
	flex-direction: row;
	margin-top: 1.5rem;
	gap: 1rem;
}

.cancel-btn {
	all: unset;
	width: 50%;
	background: #FEFEFA;
	border-radius: 0.75rem;
	border: 1.75px solid #D4CDC3;
	padding: 1rem;
	font-size: 1.5rem;
	text-align: center;
}

.save-btn {
	all: unset;
	width: 50%;
	background: #263543;
	color: #FFFFFF;
	border-radius: 0.75rem;
	padding: 1rem;
	font-size: 1.5rem;
	text-align: center;
	cursor: pointer;
}

/* Show conjugation in 1 column on narrow screens as opposed to the label and conjugation being next to each other */
@media (max-width: 875px) {
	.conjugation-grid {
		grid-template-columns: 1fr;
	}

	.grid-row {
		display: flex;
		flex-direction: column;
	}
}

/* Compact mode / mobile optimization */
@media (max-height: 850px) {
	.main-container {
		padding: 0.5rem;
	}

	.field-container p {
		font-size: 1.25rem;
		margin-bottom: 0.3rem;
	}

	.field-container button {
		font-size: 1.1rem;
		margin-bottom: 0.8rem;
	}

	.word-exists-warning {
		font-size: 1.1rem;
		margin-bottom: 0.6rem;
	}

	input,
	select {
		font-size: 1.1rem;
		padding: 0.8rem;
		border-radius: 0.6rem;
	}

	select {
		background-position: right 0.8rem center;
	}

	.box-container {
		margin-bottom: 0.6rem;
	}

	.spacing-bottom {
		margin-bottom: 0.6rem;
	}

	.spacing-top {
		margin-top: 0.6rem !important;
	}

	.rtl {
		padding: 0.36rem 1rem;
	}

	.translation-group,
	.sentence-group,
	.conjugation-grid {
		padding: 0.6rem;
		margin-top: 0.5rem;
		border-radius: 0.6rem;
	}

	.conjugation-grid {
		padding: 0.75rem;
	}

	.grid-row {
		gap: 0.1rem;
	}

	.grid-row label {
		font-size: 1rem;
		padding: 0 0 0.2rem 0.2rem;
	}

	.grid-row span {
		font-size: 1rem;
		padding: 0.6rem 0.8rem;
	}

	.buttons-container {
		gap: 0.75rem;
	}

	.cancel-btn,
	.save-btn {
		padding: 0.75rem;
		font-size: 1.2rem;
		border-radius: 0.6rem;
	}
}
</style>

