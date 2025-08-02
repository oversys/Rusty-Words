<script>
export default {
	data() {
		return {
			word: JSON.parse(localStorage.getItem('word'))
		};
	},
	methods: {
		formatItalic(text) {
			if (!text) return '';
			return text.replace(/\*(.*?)\*/g, '<i>$1</i>');
		}
	}
}
</script>

<template>
	<div class="main-container">
		<h2>Word Details</h2>

		<div v-if="word" class="details-container">
			<!-- Main word details -->
			<h1>{{ word.dutchWord }}{{ word.definiteArticle ? ', ' : '' }} {{ word.definiteArticle }}</h1>
			<h4>(verb)</h4>
			<p>→ {{ word.englishTranslation }}</p>
			<p v-if="word.arabicTranslation" style="text-align: right;">{{ word.arabicTranslation }} ←</p>
			<p v-if="word.source">Source: {{ word.source }}</p>

			<hr />

			<!-- Sentences -->
			<h2>Sentences</h2>

			<div v-for="sentence in word.sentences" class="box-container">
				<p v-html="formatItalic(sentence.sentence)"></p>
				<p>→ <span v-html="formatItalic(sentence.meaning)"></span></p>
			</div>

			<hr />

			<!-- Notes -->
			<h2>Notes</h2>

			<ul>
				<li v-for="note in word.notes">{{ note }}</li>
			</ul>

			<hr />

			<!-- Tags -->
			<h2>Tags</h2>
			
			<p v-for="tag in word.tags">{{ tag }}</p>

		</div>
		<div v-else>
			<p>This page will show the details of the last word you selected.</p>
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

.details-container {
	margin-top: 1rem;
}

.details-container p {
	font-weight: 500;
	font-size: 1.5rem;
	color: #01030B;
	margin-top: 0.5rem;
}

.box-container {
	background: #FEFEFA;
	border-radius: 0.75rem;
	border: 1.75px solid #D4CDC3;
	padding: 1.5rem;
	margin-bottom: 1rem;
}

hr {
	display: block;
	height: 1px;
	border: 0;
	border-top: 1.75px solid #D4CDC3;
	margin: 1em 0;
	padding: 0;
}
</style>
