<script setup>
import { RouterLink, RouterView } from 'vue-router'
import { invoke } from "@tauri-apps/api/core";

import Header from './components/Header.vue'
import Footer from './components/Footer.vue'
</script>

<template>
	<Header />

	<div class="container">
		<RouterView />
	</div>

	<Footer />
</template>

<style scoped>
.container {
	margin-top: 2.5rem;
	padding: 8px;
	margin-bottom: 1rem;
}
</style>

