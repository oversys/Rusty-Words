<script>
import { invoke } from "@tauri-apps/api/core";

export default {
	data() {
		return {
			words: null,
			searchQuery: ''
		}
	},
	async mounted() {
		this.words = await invoke("get_all_words");
		console.log(this.words);
	},
	methods: {
		viewWordDetails(word) {
			localStorage.setItem("word", JSON.stringify(word));
			this.$router.push('/word');
		}
	},
	computed: {
		filteredWords() {
			if (!this.searchQuery) return this.words;

			const query = this.searchQuery.toLowerCase();
			return this.words.filter(word =>
				word.dutchWord.toLowerCase().includes(query) ||
					word.englishTranslation.toLowerCase().includes(query) ||
					(word.definiteArticle && word.definiteArticle.toLowerCase().includes(query))
			);
		}
	}
}
</script>

<template>
	<div class="main-container">
		<div class="box-container search">
			<img src="../assets/icons/search.svg" alt="Search" />
			<input v-model="searchQuery" placeholder="Search words" />
		</div>

		<div v-for="word in filteredWords"
			:key="word.id"
			class="box-container"
			@click="viewWordDetails(word)"
		>
			<h2>{{ word.dutchWord }}{{ word.definiteArticle ? ', ' : '' }} {{ word.definiteArticle }}</h2>

			<div v-for="translation in word.translations">
				<p v-if="translation.language == 'English'" class="translation">→ <i>{{ translation.translation }}</i></p>
				<p v-if="translation.language == 'Arabic'" class="translation arabic-translation">{{ translation.translation }} ←</p>
			</div>
		</div>
	</div>
</template>

<style scoped>
.main-container {
	padding: 2rem 1rem;
	display: flex;
	flex-direction: column;
}

.box-container {
	background: #FEFEFA;
	border-radius: 0.75rem;
	border: 1.75px solid #D4CDC3;
	padding: 1.5rem;
	margin-bottom: 1rem;
}

.box-container:not(.search) {
	cursor: pointer;
}

.search {
	padding: 1rem;
	margin: 1.5rem 0;
	display: flex;
	flex-direction: row;
}

.search img {
	height: 2rem;
}

.search input {
	all: unset;
	padding-top: 3px;
	width: 100%;
	font-size: 1.35rem;
	font-family: Helvetica Neue;
	margin-left: 1rem;
}

.translation {
	margin-top: 0.8rem;
}

.arabic-translation {
	text-align: right;
	font-family: RB;
}
</style>

