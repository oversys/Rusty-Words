<script>
import { ask, message, open } from '@tauri-apps/plugin-dialog';
import { remove, copyFile, BaseDirectory } from '@tauri-apps/plugin-fs';
import { relaunch } from '@tauri-apps/plugin-process';

export default {
	data() {
		return {
			showMenu: false,
		};
	},
	methods: {
		toggleMenu() {
			this.showMenu = !this.showMenu;
		},

		async importDB() {
			const newDB = await open({
				multiple: false,
				directory: false,
			});

			if (!newDB) return;

			const confirm = await ask("This will overwrite the current database. Are you sure you want to proceed?", {
				title: "Import DB",
				kind: "warning",
			});

			if (confirm) {
				await copyFile(newDB, "rusty_words.db", {
					toPathBaseDir: BaseDirectory.AppData,
				})
				.then(() => relaunch())
				.catch(err => message(`Error importing database: ${err}`, { title: "Import DB", kind: "error" }));
			}
		},

		async exportDB() {
			const exportDirectory = await open({
				multiple: false,
				directory: true,
			});

			if (!exportDirectory) return;

			const date = new Date();
			const timeDate = `${String(date.getHours()).padStart(2, '0')}_${String(date.getMinutes()).padStart(2, '0')}_${String(date.getDate()).padStart(2, '0')}_${String(date.getMonth() + 1).padStart(2, '0')}_${date.getFullYear()}`;
			const destFileName = `${exportDirectory}/rusty_words_${timeDate}.db`;

			await copyFile("rusty_words.db", destFileName, {
				fromPathBaseDir: BaseDirectory.AppData,
			})
			.then(() => message(`Database exported successfully to:\n${destFileName}`, { title: "Export DB", kind: "info" }))
			.catch(err => message(`Error exporting database: ${err}`, { title: "Export DB", kind: "error" }));
		},

		async deleteDB() {
			const confirm = await ask("Are you sure you want to delete the database? This action cannot be reverted.", {
				title: "Delete DB",
				kind: "warning",
			});

			if (confirm) {
				await remove("rusty_words.db", { baseDir: BaseDirectory.AppData })
				.then(() => relaunch())
				.catch(err => message("Error deleting database: " + err, { title: "Delete DB", kind: "error" }));
			}
		}
	}
};
</script>

<template>
	<div class="header-container">
		<!-- Page name -->
		<h2>
			{{
			$route.path === '/'
			? 'My Words'
			: $route.path === '/add'
			? 'Add Word'
			: $route.path === '/word'
			? 'Word Details'
			: ''
			}}
		</h2>

		<!-- Hamburger button -->
		<div class="hamburger-wrapper">
			<div class="hamburger-container" @click="toggleMenu">
				<div class="hamburger"></div>
				<div class="hamburger"></div>
				<div class="hamburger"></div>
			</div>
		</div>

		<!-- Menu (hidden by default) -->
		<div v-if="showMenu" class="menu">
			<button @click="importDB">Import DB</button>
			<button @click="exportDB">Export DB</button>
			<button @click="deleteDB">Delete DB</button>
		</div>
	</div>
</template>

<style scoped>
.header-container {
	position: fixed;
	top: 0;
	width: 100%;
	display: flex;
	justify-content: space-between;
	align-items: center;
	padding: 1rem 0.5rem;
	background-color: #FFFFFC;
	border-bottom: 1.75px solid #D4CDC3;
}

.header-container h2 {
	flex-grow: 1;
	text-align: center;
	margin: 0;
}

.hamburger-wrapper {
	position: absolute;
	right: 3rem;
}

.hamburger-container {
	display: flex;
	flex-direction: column;
	cursor: pointer;
}

.hamburger {
	width: 25px;
	height: 3px;
	margin: 3px 0;
	background-color: black;
}

.menu {
	position: absolute;
	right: 2rem;
	top: 3rem;
	display: flex;
	flex-direction: column;
	align-items: center;
	width: 15rem;
	padding: 0.75rem 0.5rem;
	box-shadow: 0 2px 10px rgba(0, 0, 0, 0.2);
	background: #F2EEE6;
	border-radius: 0.75rem;
	border: 1.75px solid #E0DBCE;
}

.menu button {
	all: unset;
	width: 85%;
	background: #FEFEFA;
	border-radius: 0.5rem;
	border: 1.75px solid #D4CDC3;
	padding: 0.45rem;
	margin: 0.2rem 0;
	font-size: 1rem;
	text-align: center;
	cursor: pointer;
}
</style>

