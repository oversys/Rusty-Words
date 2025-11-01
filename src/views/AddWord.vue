<script>
import { invoke } from "@tauri-apps/api/core";

export default {
	data() {
		return {
			word: {
				dutchWord: "",
				type: "",
				definiteArticle: "",
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
			availableTags: [],
			availableWordTypes: ["noun", "verb", "separable verb", "adjective", "adverb", "pronoun", "preposition", "conjunction", "interjection", "not given"],
			languageOptions: ["English", "Arabic"]
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
		async submitWord() {
			console.log(this.word);

			try {
				if (this.word.type !== 'verb' && this.word.type !== 'separable verb') {
					delete this.word.conjugation;
				}

				const cleanedWord = {
					...this.word,
					translations: this.word.translations.filter(t => t.translation.trim() && t.language.trim()),
					sentences: this.word.sentences.filter(s => s.sentence.trim() && s.meaning.trim()),
					notes: this.word.notes.filter(n => n.trim()),
					tags: this.word.tags.filter(t => t.name.trim())
				};

				await invoke("add_word", { word: cleanedWord });
				alert("Word added successfully!");
				this.$router.push("/");
			} catch (e) {
				console.error(e);
				alert("Failed to add word.");
			}
		}
	},
	async mounted() {
		this.availableTags = await invoke("get_tags");
		console.log(this.availableTags);
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
		</div>

		<!-- Word type -->
		<div class="field-container">
			<p>Word Type</p>
			<div class="box-container">
				<select v-model="word.type">
					<option disabled value="">Select Word Type</option>
					<option v-for="(type, index) in availableWordTypes" :key="index" :value="type">{{ type }}</option>
				</select>
			</div>
		</div>

		<!-- Definite article (for nouns) -->
		<div v-if="word.type == 'noun'" class="field-container">
			<p>Definite Article</p>
			<div class="box-container">
				<input v-model="word.definiteArticle" />
			</div>
		</div>

		<!-- Preposition (for separable verbs) -->
		<div v-if="word.type == 'separable verb'" class="field-container">
			<p>Preposition</p>
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
					<input v-model="translation.translation" placeholder="Translation" />
				</div>
				<div class="box-container" style="margin: 0;">
					<select v-model="translation.language">
						<option disabled value="">Select Language</option>
						<option v-for="(lang, idx) in languageOptions" :key="idx" :value="lang">{{ lang }}</option>
					</select>
				</div>
			</div>
			<button @click="addTranslation" style="margin-top: 1.5rem;">+ Add Translation</button>
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

			<button @click="addSentence" style="margin-top: 1.5rem;">+ Add Sentence</button>
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
			<div class="box-container">
				<select >
					<option disabled value="">Select Tag</option>
					<option v-for="tag in availableTags" :key="tag.id" :value="tag.name">{{ tag.name }}</option>
				</select>
			</div>

			<div v-for="(tag, index) in word.tags" :key="index">
				<div v-if="tag.id === -1" class="box-container">
					<input v-model="tag.name" placeholder="New Tag" />
				</div>
			</div>
			<button @click="addTag">+ Add Tag</button>
		</div>

		<!-- Conjugation (for verbs and separable verbs) -->
		<div v-if="word.type == 'verb' || word.type == 'separable verb'" class="field-container">
			<p>Conjugation</p>
			<div class="box-container">
				<label>Present (Ik)</label>
				<input v-model="word.conjugation.presentIk" />
			</div>
			<div class="box-container">
				<label>Present (Jij)</label>
				<input v-model="word.conjugation.presentJij" />
			</div>
			<div class="box-container">
				<label>Present (U)</label>
				<input v-model="word.conjugation.presentU" />
			</div>
			<div class="box-container">
				<label>Present (Hij/Zij/Het)</label>
				<input v-model="word.conjugation.presentHijZijHet" />
			</div>
			<div class="box-container">
				<label>Present (Plural)</label>
				<input v-model="word.conjugation.presentPlural" />
			</div>
			<div class="box-container">
				<label>Imperfectum (Singular)</label>
				<input v-model="word.conjugation.imperfectumSingular" />
			</div>
			<div class="box-container">
				<label>Imperfectum (Plural)</label>
				<input v-model="word.conjugation.imperfectumPlural" />
			</div>
			<div class="box-container">
				<label>Perfectum</label>
				<input v-model="word.conjugation.perfectum" />
			</div>

			<select v-model="word.conjugation.perfectumAuxiliaryVerb">
				<option disabled value="">Select Perfectum Auxiliary Verb</option>
				<option value="hebben">hebben</option>
				<option value="zijn">zijn</option>
			</select>
		</div>

		<!-- Buttons -->
		<div class="buttons-container">
			<RouterLink to="/" class="cancel-btn">Cancel</RouterLink>
			<button @click="submitWord" class="save-btn">Save</button>
		</div>
	</div>
</template>

<style scoped>
.main-container {
	padding: 2rem 1rem;
	margin-bottom: 3rem;
	display: flex;
	flex-direction: column;
}

.box-container {
	background: #FEFEFA;
	border-radius: 0.75rem;
	border: 1.75px solid #D4CDC3;
	margin: 0 0 1.5rem 0;
}

.box-container input {
	all: unset;
	padding: 1rem;
	width: 100%;
	font-size: 1.35rem;
	font-family: Helvetica Neue;
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

.translation-group {
	background: #F2EEE6;
	border-radius: 0.75rem;
	border: 1.75px solid #E0DBCE;
	padding: 0.75rem;
	margin-top: 1rem;
}

.sentence-group {
	background: #F2EEE6;
	border-radius: 0.75rem;
	border: 1.75px solid #E0DBCE;
	padding: 0.75rem;
	margin-top: 1rem;
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
}

select {
	background: #FEFEFA;
	border-radius: 0.75rem;
	border: 1.75px solid #D4CDC3;
	padding: 0.75rem 1rem;
	font-size: 1.35rem;
	font-family: Helvetica Neue;
	font-weight: 400;
	width: 100%;
}

select:focus {
	outline: none;
	border-color: #263543;
	box-shadow: 0 0 0 2px rgba(38, 53, 67, 0.2);
}
</style>

