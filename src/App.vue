<script setup>
import { RouterLink, RouterView } from 'vue-router'
import { invoke } from "@tauri-apps/api/core";

import Footer from './components/Footer.vue'
</script>

<template>
	<div class="container">
		<p><strong>Current route path:</strong> {{ $route.fullPath }}</p>

		<RouterView />
	</div>

	<Footer />
</template>

<style scoped>
.container {
	padding: 8px;
}
</style>

