<script>
import { message } from "@tauri-apps/plugin-dialog";

export default {
	data() {
		return {
			listHover: false,
			addHover: false,
			detailsHover: false
		}
	},
	methods: {
		async goToLastWord() {
			const lastViewedWordId = localStorage.getItem("lastViewedWordId");

			if (lastViewedWordId) {
				this.$router.push(`/word/${lastViewedWordId}`);
			} else {
				await message("No word viewed yet.", { title: "Word Details", kind: "warning" });
			}
		}
	}
}
</script>

<template>
	<nav class="nav">
		<RouterLink to="/" @mouseover="listHover = true" @mouseleave="listHover = false">
			<img v-show="listHover !== true && $route.path !== '/'" src="../assets/icons/list.svg" alt="List" draggable="false" />
			<img v-show="listHover || $route.path === '/'" src="../assets/icons/list_selected.svg" alt="List" draggable="false" />
		</RouterLink>

		<RouterLink to="/add" @mouseover="addHover = true" @mouseleave="addHover = false">
			<img v-show="addHover !== true && $route.path !== '/add'" src="../assets/icons/add.svg" alt="Add" draggable="false" />
			<img v-show="addHover || $route.path === '/add'" src="../assets/icons/add_selected.svg" alt="Add" draggable="false" />
		</RouterLink>

		<button @click="goToLastWord" @mouseover="detailsHover = true" @mouseleave="detailsHover = false">
			<img v-show="detailsHover !== true && !$route.path.startsWith('/word') && !$route.path.startsWith('/edit')" src="../assets/icons/book.svg" alt="Details" draggable="false" />
			<img v-show="detailsHover || $route.path.startsWith('/word') || $route.path.startsWith('/edit')" src="../assets/icons/book_selected.svg" alt="Details" draggable="false" />
		</button>
	</nav>
</template>

<style scoped>
.nav {
	position: fixed;
	bottom: 0;
	padding-top: 0.7rem;
	padding-bottom: calc(0.5rem + env(safe-area-inset-top));
	padding-left: 0;
	padding-right: 0;

	width: 100%;
	display: flex;
	justify-content: space-around;
	background-color: #FFFFFC;
	border-top: 1.75px solid #D4CDC3;
}

button {
	all: unset;
	cursor: pointer;

	-webkit-tap-highlight-color: transparent;
	-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

button:focus {
	outline: 0;
}

.nav a {
	position: relative;
	color: inherit;
	text-decoration: none;
}

.nav a:focus {
	outline: 0;
}

.nav img {
	height: 2rem;
	width: auto;
	display: block;
}
</style>

