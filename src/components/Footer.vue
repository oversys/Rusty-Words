<script>
export default {
	data() {
		return {
			listHover: false,
			addHover: false,
			detailsHover: false
		}
	},
	mounted() {
	},
	methods: {
	}
}
</script>

<template>
	<nav class="nav">
		<RouterLink to="/" @mouseover="listHover = true" @mouseleave="listHover = false">
			<img v-show="listHover !== true && $route.path !== '/'" src="../assets/icons/list.svg" alt="List" />
			<img v-show="listHover || $route.path === '/'" src="../assets/icons/list_selected.svg" alt="List" />
		</RouterLink>

		<RouterLink to="/add" @mouseover="addHover = true" @mouseleave="addHover = false">
			<img v-show="addHover !== true && $route.path !== '/add'" src="../assets/icons/add.svg" alt="Add" />
			<img v-show="addHover || $route.path === '/add'" src="../assets/icons/add_selected.svg" alt="Add" />
		</RouterLink>

		<RouterLink to="`/word/${localStorage.getItem('lastWordId')}`" @mouseover="detailsHover = true" @mouseleave="detailsHover = false">
			<img v-show="detailsHover !== true && $route.path !== '/word'" src="../assets/icons/book.svg" alt="Details" />
			<img v-show="detailsHover || $route.path === '/word'" src="../assets/icons/book_selected.svg" alt="Details" />
		</RouterLink>
	</nav>
</template>

<style scoped>
.nav {
	position: fixed;
	bottom: 0;
	width: 100%;
	display: flex;
	justify-content: space-around;
	padding: 15px 10px;
	background-color: #FFFFFC;
	border-top: 1.75px solid #E6E2DE;
}

.nav a {
	position: relative;
	color: inherit;
	text-decoration: none;
}

.nav img {
	height: 2rem;
	width: auto;
	display: block;
}
</style>

