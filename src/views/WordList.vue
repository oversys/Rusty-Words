<script>
import { invoke } from "@tauri-apps/api/core";

export default {
	data() {
		return {
			showScrollButton: false,
			isLoading: true,

			words: [],
			searchQuery: "",
			showFilters: false,

			availableWordTypes: ["noun", "verb", "separable verb", "adjective", "adverb", "suffix", "pronoun", "preposition", "conjunction", "interjection", "not given"],
			definiteArticles: ["de", "het", "de/het"],
			availableTags: [],

			selectedWordTypes: [],
			selectedArticles: [],
			selectedTags: []
		}
	},
	methods: {
		clearFilters() {
			this.selectedWordTypes = [];
			this.selectedArticles = [];
			this.selectedTags = [];
		},
		handleScroll() {
			this.showScrollButton = window.scrollY > window.innerHeight * 2;
		},
		scrollToTop() {
			window.scrollTo({
				top: 0,
				behavior: "smooth"
			});
		}
	},
	computed: {
		filteredWords() {
			if (!this.words) return [];

			const query = this.searchQuery.toLowerCase();

			return this.words.filter(word => {
				// Text search
				const fields = [
					word.dutchWord,
					word.plural,
					word.preposition,
					word.source
				].filter(Boolean).join(" ").toLowerCase();

				const translationFields = word.translations
				.map(t => t.translation)
				.join(" ")
				.toLowerCase();

				const sentenceFields = word.sentences
				.map(s => s.sentence + " " + s.meaning)
				.join(" ")
				.toLowerCase();

				const notesFields = word.notes.join(" ").toLowerCase();

				const tagsFields = word.tags
				.map(tag => tag.name)
				.join(" ")
				.toLowerCase();

				const combined = `${fields} ${translationFields} ${sentenceFields} ${notesFields} ${tagsFields}`;

				const matchesSearch = !query || combined.includes(query);

				// Filters
				const matchesType = this.selectedWordTypes.length === 0 || this.selectedWordTypes.includes(word.type);

				const matchesArticle = this.selectedArticles.length === 0 || this.selectedArticles.includes(word.definiteArticle);

				const matchesTag = this.selectedTags.length === 0 || word.tags.some(tag => this.selectedTags.includes(tag.id));

				return matchesSearch && matchesType && matchesArticle && matchesTag;
			});
		}
	},
	async mounted() {
		window.addEventListener("scroll", this.handleScroll);

		this.words = await invoke("get_all_words");
		this.availableTags = await invoke("get_tags");

		const tagFromQuery = this.$route.query.tag;
		if (tagFromQuery) {
			this.selectedTags = [parseInt(tagFromQuery)];
			this.showFilters = true;
		}

		this.isLoading = false;
	},
	unmounted() {
		window.removeEventListener("scroll", this.handleScroll);
	}
}
</script>

<template>
	<div class="main-container">
		<transition name="fade">
			<button
				v-if="showScrollButton"
				class="scroll-top-btn"
				@click="scrollToTop"
			>
				<img src="../assets/icons/arrow_up.svg" alt="Up" />
			</button>
		</transition>

		<!-- Search & Filter -->
		<div class="search-filter-container">
			<div class="box-container search">
				<img src="../assets/icons/search.svg" alt="Search" />
				<input v-model="searchQuery" placeholder="Search words" />
			</div>

			<button class="filter-button" @click="showFilters = !showFilters"><img src="../assets/icons/filter.svg" alt="Filter" /></button>
		</div>

		<p v-if="isLoading"><i>Loading words...</i></p>
		<p v-else-if="filteredWords.length !== words.length" style="margin-bottom: 1.5rem;"><i><b>{{ filteredWords.length }}</b> out of <b>{{ words.length }}</b> words matched search/filter criteria.</i></p>
		<p v-else style="margin-bottom: 1.5rem;"><i><b>{{ words.length }}</b> words found.</i></p>

		<template v-if="isLoading">
			<div
				v-for="n in 6"
				:key="n"
				class="box-container skeleton"
			>
				<div class="skeleton-title"></div>
				<div class="skeleton-subtitle"></div>
				<div class="skeleton-line"></div>
			</div>
		</template>

		<!-- Collapsible filter section -->
		<transition name="slide-fade">
			<div v-if="showFilters" class="box-container filters">
				<h2>Filter Options</h2>

				<!-- Word types -->
				<div class="filter-group">
					<h4>Word Types</h4>
					<div class="filter-options">
						<label v-for="type in availableWordTypes" :key="type">
							<input type="checkbox" v-model="selectedWordTypes" :value="type" />
							{{ type }}
						</label>
					</div>
				</div>

				<!-- Definite articles -->
				<div class="filter-group">
					<h4>Definite Articles</h4>
					<div class="filter-options">
						<label v-for="article in definiteArticles" :key="article">
							<input type="checkbox" v-model="selectedArticles" :value="article" />
							{{ article }}
						</label>
					</div>
				</div>

				<!-- Tags -->
				<div class="filter-group">
					<h4>Tags</h4>
					<div class="filter-options">
						<label v-for="tag in availableTags" :key="tag.id">
							<input type="checkbox" v-model="selectedTags" :value="tag.id" />
							{{ tag.name }}
						</label>
					</div>
				</div>

				<button class="clear-btn" @click="clearFilters">Clear Filters</button>
			</div>
		</transition>

		<!-- Word list -->
		<div v-for="word in filteredWords"
			:key="word.id"
			class="box-container"
			@click="this.$router.push(`/word/${word.id}`);"
		>
			<h2>{{ word.dutchWord }}{{ word.definiteArticle ? ", " : "" }} {{ word.definiteArticle }}</h2>
			<h4>({{ word.type }})</h4>

			<div v-for="translation in word.translations">
				<p v-if="translation.language == 'English'" class="translation">→ <i>{{ translation.translation }}</i></p>
				<p v-else-if="translation.language == 'Arabic'" class="translation arabic-translation">{{ translation.translation }} ←</p>
			</div>
		</div>
	</div>
</template>

<style scoped>
.main-container {
	padding: 1rem;
	padding-bottom: calc(3rem + env(safe-area-inset-bottom));
	display: flex;
	flex-direction: column;
}

.scroll-top-btn {
	position: fixed;
	top: calc(4.5rem + env(safe-area-inset-top));
	left: 0;
	right: 0;
	margin: 0 auto;
	width: 3.5rem;
	height: 3.5rem;
	border-radius: 50%;
	background: #FEFEFA;
	border: 1.75px solid #D4CDC3;
	display: flex;
	align-items: center;
	justify-content: center;
	cursor: pointer;
	box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
	z-index: 100;
	transition: transform 0.2s ease;

	-webkit-tap-highlight-color: transparent;
	-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

.scroll-top-btn:focus {
	outline: 0;
}

.scroll-top-btn img {
	height: 1.8rem;
	width: 1.8rem;
}

@media (hover: hover) {
	.scroll-top-btn:hover {
		background: #F8F6F1;
		transform: translateY(-2px);
	}
}

.fade-enter-active,
.fade-leave-active {
	transition: opacity 0.3s ease, transform 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
	opacity: 0;
	transform: translateY(-10px);
}

.search-filter-container {
	display: flex;
	align-items: center;
	gap: 0.5rem;
}

.filters {
	box-shadow: 0 3px 10px rgba(0, 0, 0, 0.05);
}

@media (hover: hover) {
	.filters:hover {
		box-shadow: 0 5px 15px rgba(0, 0, 0, 0.08);
	}
}

.filter-button {
	all: unset;
	display: flex;
	align-items: center;
	justify-content: center;
	background: #FEFEFA;
	border-radius: 0.75rem;
	border: 1.75px solid #D4CDC3;
	height: 3.5rem;
	width: 3.5rem;
	padding: 0.25rem;
	cursor: pointer;
	margin-top: 0.5rem;

	-webkit-tap-highlight-color: transparent;
	-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

.filter-button img {
	height: 2.2rem;
	width: 2.2rem;
}

.slide-fade-enter-active {
	transition: all 0.35s ease-out;
}

.slide-fade-leave-active {
	transition: all 0.35s ease-in;
}

.slide-fade-enter-from,
.slide-fade-leave-to {
	opacity: 0;
	transform: translateY(-10px);
}

.filters h2 {
	margin: 0 0 1.5rem;
}

.filter-group {
	margin-bottom: 1.5rem;
	padding-bottom: 1rem;
	border-bottom: 1px dashed #E2DBD4;
}

.filter-group:last-of-type {
	border-bottom: none;
	margin-bottom: 1.5rem;
}

.filter-group h4 {
	margin: 0 0 0.7rem;
	color: #000;
}

.filter-options {
	display: flex;
	flex-wrap: wrap;
	gap: 0.4rem 0.9rem;
}

.filter-options input {
	display: none;
}

.filter-options label {
	background: #F8F6F1;
	border: 1px solid #DCD3C6;
	padding: 0.35rem 0.75rem;
	border-radius: 1rem;
	cursor: pointer;
	transition: all 0.2s ease;

	-webkit-tap-highlight-color: transparent;
	-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

@media (hover: hover) {
	.filter-options label:hover {
		background: #EAE4DA;
	}
}

.filter-options label:has(input:checked) {
	background: #EAE4DA;
}

.clear-btn {
	all: unset;
	padding: 0.4rem 0.8rem;
	font-size: 1.2rem;
	border-radius: 0.5rem;
	cursor: pointer;
	border: 1.75px solid #E6B4B4;
	background-color: #F8DADA;
	color: #000;
	transition: all 0.2s ease;
	text-decoration: none;
	text-align: center;

	-webkit-tap-highlight-color: transparent;
	-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

@media (hover: hover) {
	.clear-btn:hover {
		background: #F2BDBD;
	}
}

.box-container {
	background: #FEFEFA;
	border-radius: 0.75rem;
	border: 1.75px solid #D4CDC3;
	padding: 1.5rem;
	margin-bottom: 1rem;

	overflow-wrap: break-word;
	word-wrap: break-word;
	word-break: break-word;
	-ms-hyphens: auto;
	-moz-hyphens: auto;
	-webkit-hyphens: auto;
	hyphens: auto;

	-webkit-tap-highlight-color: transparent;
	-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

.box-container:not(.search, .filters) {
	cursor: pointer;
}

.search {
	flex: 1;
	padding: 1rem;
	margin: 1rem 0 0.5rem 0;
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

.skeleton {
	pointer-events: none;
}

.skeleton-title,
.skeleton-subtitle,
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
	height: 1.6rem;
	width: 60%;
	margin-bottom: 0.75rem;
}

.skeleton-subtitle {
	height: 1.1rem;
	width: 30%;
	margin-bottom: 1rem;
}

.skeleton-line {
	height: 1.1rem;
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

.translation {
	margin-top: 0.8rem;
}

.arabic-translation {
	text-align: right;
	font-family: RB;
}

/* Compact mode / mobile optimization */
@media (max-height: 850px) {
	.main-container {
		padding: 0.5rem;
		padding-bottom: calc(3rem + env(safe-area-inset-bottom));
	}

	.search {
		margin: 0.5rem 0 1rem 0;
		padding: 0.6rem;
	}

	.search img {
		height: 1.4rem;
	}

	.search input {
		font-size: 1rem;
		margin-left: 0.6rem;
	}

	.filter-button {
		height: 2.7rem;
		width: 2.7rem;
		border-radius: 0.6rem;
		margin-top: 0;
	}

	.filter-button img {
		height: 1.7rem;
		width: 1.7rem;
	}

	.box-container {
		padding: 0.85rem;
		margin-bottom: 0.6rem;
		border-radius: 0.6rem;
	}

	.box-container h2 {
		font-size: 1.57rem;
		margin: 0;
		line-height: 1.2;
	}

	.box-container h4 {
		font-size: 1.1rem;
		margin: 0.2rem 0 0.5rem 0;
		color: #666;
	}

	.translation {
		margin-top: 0.4rem;
		margin-bottom: 0.2rem;
		font-size: 1.1rem;
		line-height: 1.3;
	}

	.filters h2 {
		font-size: 1.57rem;
		margin: 0 0 0.8rem;
	}

	.filter-group {
		margin-bottom: 0.8rem;
		padding-bottom: 0.6rem;
	}

	.filter-group h4 {
		font-size: 1.1rem;
		margin: 0 0 0.4rem;
	}

	.filter-options label {
		padding: 0.3rem 0.6rem;
		font-size: 0.9rem;
	}

	.scroll-top-btn {
		width: 2.75rem;
		height: 2.75rem;
	}

	.scroll-top-btn img {
		height: 1.2rem;
		width: 1.2rem;
	}

	p {
		font-size: 1.1rem;
		margin-bottom: 0.8rem;
	}

	.clear-btn {
		font-size: 0.93rem;
	}

	.filter-group:last-of-type {
		margin-bottom: 1.2rem;
	}

	.skeleton-title,
	.skeleton-subtitle,
	.skeleton-line {
		border-radius: 3px;
	}

	.skeleton-title {
		height: 1.26rem;
	}

	.skeleton-subtitle,
	.skeleton-line {
		height: 0.86rem;
	}
}
</style>

