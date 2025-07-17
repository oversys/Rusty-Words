<script>
import { invoke } from "@tauri-apps/api/core";

export default {
	data() {
		return {
			name: 'name',
			greetMsg: ''
		}
	},
	async mounted() {
		await this.greet();
		this.greetMsg = await invoke("greet", { name: this.name });
	},
	methods: {
		async greet() {
		}
	}
}
</script>

<template>
	<p>{{ greetMsg }}</p>
</template>

<style scoped>
</style>

