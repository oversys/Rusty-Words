<script setup>
import { RouterLink, RouterView } from 'vue-router'
import { invoke } from "@tauri-apps/api/core";

import Header from './components/Header.vue'
import Footer from './components/Footer.vue'
</script>

<template>
	<div class="app-container">
		<Header />

		<div class="page-container">
			<RouterView />
		</div>

		<Footer />
	</div>
</template>

<style scoped>
.app-container {
	/* Ensure content fits safely on screens with notches or nav bars */
	padding-top: env(safe-area-inset-top);
	padding-bottom: env(safe-area-inset-bottom);
	padding-left: env(safe-area-inset-left);
	padding-right: env(safe-area-inset-right);
}

.page-container {
	margin-top: 2.5rem;
	padding: 8px;
	margin-bottom: 1rem;
}
</style>

