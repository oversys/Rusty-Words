<script>
import { openUrl } from "@tauri-apps/plugin-opener";

export default {
	data() {
		return {
			word: JSON.parse(localStorage.getItem("word"))
		};
	},
	methods: {
		formatItalic(text) {
			if (!text) return "";

			// Convert *text* to <i>text</i>
			return text.replace(/\*(.*?)\*/g, "<i>$1</i>");
		},

		formatLinks(text) {
			if (!text) return "";

			// Convert [text](url) to <a href="url">text</a>
			return text.replace(/\[(.*?)\]\((.*?)\)/g, (match, text, url) => {
				if (url.startsWith("https://") || url.startsWith("http://")) {
					return `<a style="color: #AB9C84; cursor: pointer; text-decoration: underline;" data-external-url="${url}">${text}</a>`;
				} else {
					return `<a style="color: #AB9C84;" href="${url}">${text}</a>`;
				}
			});
		},

		formatText(text) {
			// Apply all formatting
			let formatted = this.formatItalic(text);
			formatted = this.formatLinks(formatted);

			return formatted;
		},

		handleClick(event) {
			const targetData = event.target.dataset;

			if (targetData.externalUrl)
				openUrl(event.target.dataset.externalUrl);
		}
	}
}
</script>

<template>
	<div class="main-container" @click="handleClick">
		<div v-if="word" class="details-container">
			<!-- Main word details -->
			<h1>{{ word.dutchWord }}{{ word.definiteArticle ? ", " : "" }} {{ word.definiteArticle }}</h1>
			<h4>({{ word.type }})</h4>

			<br>

			<div v-for="translation in word.translations">
				<p v-if="translation.language == 'English'" class="translation">→ <i>{{ translation.translation }}</i></p>
				<p v-if="translation.language == 'Arabic'" class="translation arabic-translation">{{ translation.translation }} ←</p>
			</div>

			<hr />

			<p v-if="word.plural">Plural: {{ word.plural }}</p>
			<p v-if="word.preposition">Preposition: {{ word.preposition }}</p>
			<p v-if="word.source">Source: <span v-html="formatText(word.source)"></span></p>

			<hr />

			<!-- Sentences -->
			<h2>Sentences</h2>

			<div class="sentences-container">
				<div v-for="sentence in word.sentences" class="box-container">
					<p v-html="formatText(sentence.sentence)"></p>
					<p>→ <span v-html="formatText(sentence.meaning)"></span></p>
				</div>
			</div>

			<hr />

			<!-- Notes -->
			<h2>Notes</h2>

			<ul>
				<li v-for="note in word.notes" v-html="formatText(note)"></li>
			</ul>

			<hr />

			<!-- Tags -->
			<h2>Tags</h2>

			<div class="tags-container">
				<div v-for="tag in word.tags" class="tag">
					<a href="/">{{ tag.name }}</a>
				</div>
			</div>
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

.arabic-translation {
	text-align: right;
	font-family: RB;
}

.box-container {
	background: #FEFEFA;
	border-radius: 0.75rem;
	border: 1.75px solid #D4CDC3;
	padding: 1.5rem;
	margin-bottom: 1rem;
}

.sentences-container {
	margin-top: 0.5rem;
}

ul {
	list-style: none;
	padding-left: 0;
	margin-top: 1rem;
}

ul li {
	background: #FEFEFA;
	border: 1.75px solid #D4CDC3;
	border-radius: 0.75rem;
	padding: 1rem 1.25rem;
	margin-bottom: 0.75rem;
	font-size: 1.5rem;
}

.tags-container {
	display: flex;
	gap: 0.5rem;
	flex-wrap: wrap;
	margin-top: 1rem;
}

.tag {
	background-color: #EFEBE0;
	border: 1.75px solid #D5CEC3;
	border-radius: 0.75rem;
	padding: 0.3rem 0.6rem;
	text-align: center;
	white-space: nowrap;
}

.tag a {
	text-decoration: none;
	font-weight: 400;
	font-size: 1.4rem;
	color: #000;
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
