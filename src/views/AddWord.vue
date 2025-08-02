<script>
import { invoke } from "@tauri-apps/api/core";

export default {
	data() {
		return {
			word: {
				dutchWord: '',
				definiteArticle: '',
				englishTranslation: '',
				arabicTranslation: '',
				source: '',
				sentences: [{ sentence: '', meaning: '' }],
				notes: [''],
				tags: ['']
			}
		}
	},
	methods: {
		addSentence() {
			this.word.sentences.push({ sentence: '', meaning: '' });
		},
		addNote() {
			this.word.notes.push('');
		},
		addTag() {
			this.word.tags.push('');
		},
		async submitWord() {
			try {
				const cleanedWord = {
					...this.word,
					sentences: this.word.sentences.filter(s => s.sentence.trim() && s.meaning.trim()),
					notes: this.word.notes.filter(n => n.trim()),
					tags: this.word.tags.filter(t => t.trim())
				};

				await invoke("add_word", { word: cleanedWord });
				alert("Word added successfully!");
				this.$router.push('/');
			} catch (e) {
				console.error(e);
				alert("Failed to add word.");
			}
		}
	}
}
</script>

<template>
	<div class="main-container">
		<h2>Add Word</h2>

		<div class="field-container">
			<p>Dutch Word</p>
			<div class="box-container">
				<input v-model="word.dutchWord" />
			</div>
		</div>

		<div class="field-container">
			<p>Definite Article</p>
			<div class="box-container">
				<input v-model="word.definiteArticle" />
			</div>
		</div>

		<div class="field-container">
			<p>English Translation</p>
			<div class="box-container">
				<input v-model="word.englishTranslation" />
			</div>
		</div>

		<div class="field-container">
			<p>Arabic Translation</p>
			<div class="box-container">
				<input v-model="word.arabicTranslation" />
			</div>
		</div>

		<div class="field-container">
			<p>Source</p>
			<div class="box-container">
				<input v-model="word.source" />
			</div>
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
			<div v-for="(tag, index) in word.tags" :key="index" class="box-container">
				<input v-model="word.tags[index]" />
			</div>
			<button @click="addTag">+ Add Tag</button>
		</div>

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
</style>

