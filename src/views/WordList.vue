<script>
import { invoke } from "@tauri-apps/api/core";

export default {
	data() {
		return {
			words: null
		}
	},
	async mounted() {
		this.words = await invoke("get_all_words");
		console.log(this.words);
	},
	methods: {
	}
}
</script>

<template>
	<div class="main-container">
		<h2>My Words</h2>

		<div class="box-container search">
			<img src="../assets/icons/search.svg" alt="Search" />
			<input placeholder="Search words" />
		</div>

		<div v-for="word in words" class="box-container">
			<h2>{{ word.dutch_word }}{{ word.definite_article ? ', ' : '' }} {{ word.definite_article }}</h2>
			<p class="translation">→ <i>{{ word.english_translation }}</i></p>
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
	border: 1.75px solid #E6E2DE;
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
</style>

