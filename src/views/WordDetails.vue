<script>
import { invoke } from "@tauri-apps/api/core";
import { openUrl } from "@tauri-apps/plugin-opener";
import { ask, message } from "@tauri-apps/plugin-dialog";

export default {
	props: ["wordId"],

	data() {
		return {
			showSkeleton: false,
			word: null,
			conjugationLabels: {
				presentIk: "Present, ik",
				presentJij: "Present, jij",
				presentU: "Present, u",
				presentHijZijHet: "Present, hij/zij/het",
				presentPlural: "Present, plural",
				imperfectumSingular: "Imperfectum, singular",
				imperfectumPlural: "Imperfectum, plural",
				perfectum: "Perfectum"
			}
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
		},

		async copyReference() {
			try {
				await navigator.clipboard.writeText(`/word/${this.wordId}`);
				await message("Reference copied to clipboard.", { title: "Copy Reference", kind: "info" });
			} catch (err) {
				await message("Failed to copy reference.", { title: "Copy Reference", kind: "error" });
			}
		},

		async removeWord() {
			const confirm = await ask("Are you sure you want to delete this word?", {
				title: "Delete word",
				kind: "warning",
			});

			if (confirm) {
				await invoke("delete_word", { wordId: parseInt(this.wordId) });
				await message("Word deleted successfully.", { title: "Delete word", kind: "info" });

				localStorage.removeItem("lastViewedWordId");
				this.$router.push("/");
			}
		}
	},

	async mounted() {
		setTimeout(() => {
			if (!this.word) {
				this.showSkeleton = true;
			}
		}, 300);

		this.word = await invoke("get_word", { wordId: parseInt(this.wordId) });
		localStorage.setItem("lastViewedWordId", parseInt(this.wordId));
	},
}
</script>

<template>
	<div class="main-container" @click="handleClick">
		<div v-if="word" class="word-container">
			<!-- Main word details -->
			<div class="word-header">
				<div class="word-info">
					<h1>{{ word.dutchWord }}{{ word.definiteArticle ? ", " : "" }} {{ word.definiteArticle }}</h1>
					<h4>({{ word.type }})</h4>
				</div>

				<div class="word-actions">
					<button class="remove-btn" @click="removeWord">Remove</button>
					<button class="normal-btn" @click="copyReference">Copy Reference</button>
					<RouterLink :to="`/edit/${word.id}`" class="normal-btn">Edit</RouterLink>
				</div>
			</div>

			<br>

			<template v-for="translation in word.translations">
				<p v-if="translation.language == 'English'" class="translation">→ <i>{{ translation.translation }}</i></p>
				<p v-if="translation.language == 'Arabic'" class="translation arabic-translation">{{ translation.translation }} ←</p>
			</template>

			<hr />

			<!-- Details -->
			<h2>Details</h2>

			<div v-if="word.plural || word.preposition || word.source" class="details-container">
				<p v-if="word.plural">
					<span class="label">Plural:</span> {{ word.plural }}
				</p>

				<p v-if="word.preposition">
					<span class="label">Separable prefix:</span> {{ word.preposition }}
				</p>

				<p v-if="word.source">
					<span class="label">Source:</span> <span v-html="formatText(word.source)"></span>
				</p>
			</div>
			<div v-else class="empty-section">No details for this word.</div>

			<hr />

			<!-- Sentences -->
			<h2>Sentences</h2>

			<div v-if="word.sentences && word.sentences.length" class="sentences-container">
				<div v-for="sentence in word.sentences" class="box-container">
					<p v-html="formatText(sentence.sentence)"></p>
					<p>→ <span v-html="formatText(sentence.meaning)"></span></p>
				</div>
			</div>
			<div v-else class="empty-section">No example sentences.</div>

			<hr />

			<!-- Notes -->
			<h2>Notes</h2>

			<ul v-if="word.notes && word.notes.length">
				<li v-for="note in word.notes" v-html="formatText(note)"></li>
			</ul>
			<div v-else class="empty-section">No notes available.</div>

			<hr />

			<!-- Tags -->
			<h2>Tags</h2>

			<div class="tags-container" v-if="word.tags && word.tags.length">
				<div v-for="tag in word.tags" class="tag">
					<RouterLink :to="{ path: '/', query: { tag: tag.id } }">{{ tag.name }}</RouterLink>
				</div>
			</div>
			<div v-else class="empty-section">No tags for this word.</div>

			<hr v-if="word.conjugation" />

			<!-- Conjugation -->
			<div v-if="word.conjugation" class="conjugation-container">
				<h2>Conjugation</h2>

				<div class="conjugation-grid">
					<div class="grid-row" v-for="(label, key) in conjugationLabels" :key="key">
						<label>{{ label }}</label>
						<span>{{ word.conjugation[key] }}</span>
					</div>

					<div class="grid-row">
						<label>Perfectum Auxiliary Verb</label>
						<span>{{ word.conjugation.perfectumAuxiliaryVerb }}</span>
					</div>
				</div>
			</div>
		</div>

		<div v-else-if="showSkeleton" class="word-container">
			<div class="word-header">
				<div class="word-info" style="width: 100%;">
					<div class="skeleton-title"></div>
					<div class="skeleton-subtitle" style="width: 30%;"></div>
				</div>
			</div>

			<div class="skeleton-line" style="width: 50%; margin-top: 2rem;"></div>
			<div class="skeleton-line" style="width: 35%; margin-top: 1rem; margin-left: 65%;"></div>

			<hr />
			<div class="skeleton-heading"></div>
			<div class="skeleton-line" style="width: 100%; height: 3.5rem;"></div>

			<hr />
			<div class="skeleton-heading"></div>
			<div class="skeleton-line" style="width: 100%; height: 10rem;"></div>

			<hr />
			<div class="skeleton-heading"></div>
			<div class="skeleton-line" style="width: 100%; height: 3.5rem;"></div>

			<hr />
			<div class="skeleton-heading"></div>
			<div class="skeleton-line" style="width: 100%; height: 3.5rem;"></div>
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

.word-container {
	margin-top: 1rem;
}

.word-container p {
	font-weight: 500;
	font-size: 1.5rem;
	color: #01030B;
	margin-top: 0.5rem;
}

.word-header {
	display: flex;
	justify-content: space-between;
	align-items: center;

	overflow-wrap: break-word;
	word-wrap: break-word;
	word-break: break-word;
	-ms-hyphens: auto;
	-moz-hyphens: auto;
	-webkit-hyphens: auto;
	hyphens: auto;
}

.word-actions {
	display: flex;
	gap: 0.75rem;
	white-space: nowrap;
}

.normal-btn,
.remove-btn {
	all: unset;
	padding: 0.4rem 0.8rem;
	font-size: 1.2rem;
	border-radius: 0.5rem;
	cursor: pointer;
	border: 1.75px solid #D4CDC3;
	background-color: #EFEBE0;
	color: #000;
	transition: all 0.2s ease;
	text-decoration: none;
	text-align: center;

	-webkit-tap-highlight-color: transparent;
	-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

@media (hover: hover) {
	.normal-btn:hover {
		background-color: #E3DCCD;
	}
}

.remove-btn {
	background-color: #F8DADA;
	border-color: #E6B4B4;
}

@media (hover: hover) {
	.remove-btn:hover {
		background-color: #F2BDBD;
	}
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

.details-container {
	display: flex;
	flex-direction: column;
	gap: 0.75rem;
	margin-top: 1rem;
}

.details-container p {
	font-size: 1.5rem;
	color: #01030B;
	margin: 0;
	padding: 0.4rem 0.6rem;
	background: #FEFEFA;
	border: 1.75px solid #D4CDC3;
	border-radius: 0.5rem;
}

.label {
	color: #7A6F5C;
	margin-right: 0.5rem;
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
	border-radius: 0.5rem;
	padding: 0.3rem 0.6rem;
	text-align: center;
	white-space: nowrap;
}

.tag a {
	text-decoration: none;
	font-weight: 400;
	font-size: 1.4rem;
	color: #000;
	cursor: pointer;
}

hr {
	display: block;
	height: 1px;
	border: 0;
	border-top: 1.75px solid #D4CDC3;
	margin: 1rem 0;
	padding: 0;
}

.conjugation-container {
	margin-top: 1rem;
}

.conjugation-grid {
	display: grid;
	grid-template-columns: 1fr 2fr;
	gap: 0.75rem 3rem;
	background: #F2EEE6;
	border-radius: 0.75rem;
	border: 1.75px solid #E0DBCE;
	padding: 1rem;
	margin-top: 1rem;
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

.grid-row span {
	font-size: 1.35rem;
	padding: 0.75rem 1rem;
	background: #FEFEFA;
	border: 1.75px solid #D4CDC3;
	border-radius: 0.75rem;
	display: block;
}

.empty-section {
	font-size: 1.5rem;
	font-style: italic;
	color: #A0A0A0;
	background: #EDEAE1;
	border: 1.5px dashed #D4CDC3;
	border-radius: 0.75rem;
	padding: 1rem;
	margin: 1rem 0;
	text-align: center;
}

.skeleton-title,
.skeleton-subtitle,
.skeleton-heading,
.skeleton-line {
	background: linear-gradient(
		90deg,
		#ebebeb 25%,
		#f5f5f5 37%,
		#ebebeb 63%
		);
	background-size: 400% 100%;
	animation: shimmer 1.4s ease infinite;
	border-radius: 4px;
}

.skeleton-title {
	height: 3.5rem;
	width: 90%;
	margin-bottom: 0.75rem;
}

.skeleton-subtitle {
	height: 1.1rem;
	width: 30%;
	margin-bottom: 1rem;
}

.skeleton-heading {
	height: 1.5rem;
	width: 40%;
	margin-bottom: 1rem;
}

.skeleton-line {
	height: 1.3rem;
	width: 80%;
}

@keyframes shimmer {
	0% {
		background-position: 100% 0;
	}

	100% {
		background-position: 0 0;
	}
}

/* Move buttons below word and type on narrow screens */
@media (max-width: 1200px) {
	.word-header {
		flex-direction: column;
		align-items: flex-start;
		gap: 1rem;
	}

	.word-actions {
		width: 100%;
		justify-content: center;
		gap: 0.5rem;
	}

	.normal-btn,
	.remove-btn {
		width: 50%;
	}
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

	.word-container {
		margin-top: 0.5rem;
	}

	.word-info h1 {
		font-size: 2.5rem;
	}

	.word-info h4 {
		font-size: 1.1rem;
	}

	.word-container p,
	.details-container p,
	ul li,
	.empty-section {
		font-size: 1.1rem;
	}

	.empty-section {
		padding: 0.8rem;
	}

	.normal-btn,
	.remove-btn {
		font-size: 1rem;
	}

	hr {
		margin: 0.9rem 0;
	}

	h2 {
		font-size: 1.57rem;
	}

	.translation:first-of-type {
		margin-top: 0.2rem !important;
	}

	.box-container {
		padding: 0.85rem;
		margin-bottom: 0.6rem;
		border-radius: 0.6rem;
	}

	.details-container {
		gap: 0.6rem;
		margin-top: 0.5rem;
	}

	.details-container p {
		padding: 0.3rem 0.6rem;
	}

	ul {
		margin-top: 0.5rem;
	}

	ul li {
		padding: 0.6rem 0.8rem;
		margin-bottom: 0.6rem;
		border-radius: 0.6rem;
	}

	.tags-container {
		gap: 0.4rem;
		margin-top: 0.5rem;
	}

	.tag a {
		font-size: 1.1rem;
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
}
</style>

