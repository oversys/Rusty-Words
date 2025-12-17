<script>
import { ask, message, open, save } from "@tauri-apps/plugin-dialog";
import { remove, readFile, writeFile, BaseDirectory } from "@tauri-apps/plugin-fs";
import { relaunch } from "@tauri-apps/plugin-process";

export default {
	data() {
		return {
			showMenu: false,
		};
	},
	methods: {
		async importDB() {
			const newDB = await open({
				multiple: false,
				directory: false,
			});

			if (!newDB) return;

			const confirm = await ask("This will overwrite the current database. Are you sure you want to proceed? (Consider exporting the current DB first)", {
				title: "Import DB",
				kind: "warning",
			});

			if (!confirm) return;

			try {
				const dbContents = await readFile(newDB);

				await writeFile("rusty_words.db", dbContents, {
					baseDir: BaseDirectory.AppData,
				});

				await relaunch();
			} catch (err) {
				await message(`Error importing database: ${err}`, { title: "Import DB", kind: "error" });
			}
		},

		async exportDB() {
			const date = new Date();
			const timeDate = `${String(date.getHours()).padStart(2, '0')}_${String(date.getMinutes()).padStart(2, '0')}_${String(date.getDate()).padStart(2, '0')}_${String(date.getMonth() + 1).padStart(2, '0')}_${date.getFullYear()}`;

			const destPath = await save({defaultPath: `rusty_words_${timeDate}.db`});

			if (!destPath) return;

			try {
				const dbContents = await readFile("rusty_words.db", {
					baseDir: BaseDirectory.AppData,
				});

				await writeFile(destPath, dbContents);

				await message(`Database exported successfully to:\n${destPath}`, { title: "Export DB", kind: "info" });
			} catch (err) {
				await message(`Error exporting database: ${err}\n\nPATH: ${destPath}`, { title: "Export DB", kind: "error" });
			}
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
		},

		handleClickOutside(event) {
			// Close the menu if clicked outside
			if (this.showMenu && !this.$refs.menu.contains(event.target) && !this.$refs.hamburger.contains(event.target))
				this.showMenu = false;
		},

		handleEscapeKey(event) {
			// Close the menu if the Escape key is pressed
			if (event.key === "Escape")
				this.showMenu = false;
		}
	},
	mounted() {
		document.addEventListener("mousedown", this.handleClickOutside);
		document.addEventListener("keydown", this.handleEscapeKey);
	},
	beforeDestroy() {
		document.removeEventListener("mousedown", this.handleClickOutside);
		document.removeEventListener("keydown", this.handleEscapeKey);
	}
};
</script>

<template>
	<div class="header-container">
		<!-- Page name -->
		<h2>
			{{
			$route.path === "/"
			? "My Words"
			: $route.path === "/add"
			? "Add Word"
			: $route.path.startsWith("/word")
			? "Word Details"
			: $route.path.startsWith("/edit")
			? "Edit Word"
			: ""
			}}
		</h2>

		<!-- Hamburger button -->
		<div class="hamburger-wrapper">
			<div class="hamburger-container" @click="showMenu = !showMenu" ref="hamburger">
				<div class="hamburger"></div>
				<div class="hamburger"></div>
				<div class="hamburger"></div>
			</div>
		</div>

		<!-- Menu (hidden by default) -->
		<div v-if="showMenu" class="menu" ref="menu">
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
	left: 0;
	right: 0;
	padding-top: calc(0.5rem + env(safe-area-inset-top));
	padding-bottom: 0.5rem;
	padding-left: 0.5rem;
	padding-right: 0.5rem;

	width: 100%;
	display: flex;
	justify-content: space-between;
	align-items: center;
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

	-webkit-tap-highlight-color: transparent;
	-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

.hamburger {
	width: 25px;
	height: 3px;
	margin: 3px 0;
	background-color: black;
}

.menu {
	position: absolute;
	right: calc(2rem + env(safe-area-inset-right));
	top: calc(3.5rem + env(safe-area-inset-top));
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

	-webkit-tap-highlight-color: transparent;
	-webkit-tap-highlight-color: rgba(0, 0, 0, 0);
}

/* Compact mode / mobile optimization */
@media (max-height: 850px) {
	.header-container h2 {
		font-size: 1.57rem;
	}

	.hamburger {
		margin: 2.5px 0;
	}

	.menu {
		width: 13rem;
		padding: 0.5rem 0.3rem;
	}

	.menu button {
		padding: 0.4rem;
		font-size: 0.95rem;
	}
}
</style>

