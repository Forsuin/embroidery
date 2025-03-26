<script lang="ts">
	import { onDestroy, onMount } from "svelte";
	import "@tauri-apps/api/menu";
	import { Menu } from "@tauri-apps/api/menu";
	import { platform } from "@tauri-apps/plugin-os";
	import ImportDialogue from "$lib/components/ImportDialogue.svelte";
	import { listen } from "@tauri-apps/api/event";
	import type { DragDropEvent } from "@tauri-apps/api/webview";
	import type { Path } from "$env/static/private";
	import { invoke } from "@tauri-apps/api/core";

	let { children } = $props();

	async function onKeyDown(event: KeyboardEvent) {
		let shortcut = "";

		if (event.metaKey || event.ctrlKey) {
			shortcut += "CmdOrCtrl+";
		}

		if (event.shiftKey) {
			shortcut += "Shift+";
		}

		if (event.altKey) {
			shortcut += "Alt+";
		}

		shortcut += event.key.toUpperCase();

		// console.log(shortcut);

		let action = shortcutToActionName[shortcut];
		if (action) {
			action.action();
		}

		// console.log(`shortcut: ${shortcut}`);
	}

	const accelerators = {
		import: {
			accelerator: "CmdOrCtrl+I",
			action: () => {
				console.log("Import shortcut");
				showImportDialogue();
			},
		},
		reveal: {
			accelerator: "CmdOrCtrl+Shift+O",
			action: () => {
				console.log("Reveal shortcut");
			},
		},
		settings: {
			accelerator: "CmdOrCtrl+,",
			action: () => {
				console.log("Settings shortcut");
			},
		},
		quit: {
			accelerator: "CmdOrCtrl+Q",
			action: () => {
				console.log("Quit shortcut");
			},
		},
		undo: {
			accelerator: "CmdOrCtrl+Z",
			action: () => {
				console.log("Undo shortcut");
			},
		},
		redo: {
			accelerator: "CmdOrCtrl+Shift+Z",
			action: () => {
				console.log("Redo shortcut");
			},
		},
		cut: {
			accelerator: "CmdOrCtrl+X",
			action: () => {
				console.log("Cut shortcut");
			},
		},
		copy: {
			accelerator: "CmdOrCtrl+C",
			action: () => {
				console.log("Copy shortcut");
			},
		},
		paste: {
			accelerator: "CmdOrCtrl+V",
			action: () => {
				console.log("Paste shortcut");
			},
		},
		selectAll: {
			accelerator: "CmdOrCtrl+A",
			action: () => {
				console.log("Select all shortcut");
			},
		},
	};

	const shortcutToActionName = Object.fromEntries(
		Object.entries(accelerators).map(([name, { accelerator, action }]) => [
			accelerator,
			{
				name,
				action,
			},
		]),
	);

	// console.log(shortcutToActionName);

	onMount(async () => {
		let file_explorer_prompt;

		switch (platform()) {
			case "windows":
				file_explorer_prompt = "Reveal in File Explorer";
				break;
			case "macos":
				file_explorer_prompt = "Show in Finder";
				break;
			default:
				file_explorer_prompt = "Show in File Manager";
		}

		if (platform() == "windows") {
			// alert("added event listener");
			window.addEventListener("keydown", onKeyDown);
		}

		const file_submenu = {
			items: [
				{
					id: "import",
					text: "Import",
					action: accelerators.import.action,
					accelerator: accelerators.import.accelerator,
				},
				{
					id: "reveal",
					text: file_explorer_prompt,
					action: accelerators.reveal.action,
					accelerator: accelerators.reveal.accelerator,
				},
				{
					item: "Separator",
				},
				{
					id: "settings",
					text: "Settings",
					action: accelerators.settings.action,
					accelerator: accelerators.settings.accelerator,
				},
				{
					item: "Separator",
				},
				{
					id: "quit",
					text: "Quit",
					action: accelerators.quit.action,
					accelerator: accelerators.quit.accelerator,
				},
			],
			text: "File",
		};

		const edit_submenu = {
			items: [
				{
					id: "undo",
					text: "Undo",
					accelerator: accelerators.undo.accelerator,
					action: accelerators.undo.action,
				},
				{
					id: "redo",
					text: "Redo",
					accelerator: accelerators.redo.accelerator,
					action: accelerators.redo.action,
				},
				{
					item: "Separator",
				},
				{
					id: "cut",
					text: "Cut",
					accelerator: accelerators.cut.accelerator,
					action: accelerators.cut.action,
				},
				{
					id: "copy",
					text: "Copy",
					accelerator: accelerators.copy.accelerator,
					action: accelerators.copy.action,
				},
				{
					id: "paste",
					text: "Paste",
					accelerator: accelerators.paste.accelerator,
					action: accelerators.paste.action,
				},
				{
					item: "Separator",
				},
				{
					id: "selectAll",
					text: "Select All",
					accelerator: accelerators.selectAll.accelerator,
					action: accelerators.selectAll.action,
				},
			],
			text: "Edit",
		};

		const menu = await Menu.new({
			items: [file_submenu, edit_submenu],
		});

		menu.setAsAppMenu();

		await listen<DragDropEvent>("tauri://drag-drop", (event) => {
			// console.log(event);

			// paths exists, but compiler is dumb
			// @ts-ignore
			const files = event.payload.paths;

			type DragDropPayload = {
				paths: string[];
			};

			let payload = {
				paths: files,
			};

			invoke("drag_drop_file_dialog", { payload: payload });

			showImportDialogue();
		});
	});

	let isImportDialogueOpen = $state(false);

	function showImportDialogue() {
		// console.log("show import");
		isImportDialogueOpen = true;
	}
</script>

<!-- <div
	id="drop-area"
	ondrop={handleDrop}
	ondragover={dragOverHandler}
	role="none"
> -->
<ImportDialogue bind:isOpen={isImportDialogueOpen} />
<main>
	{@render children()}
</main>
<!-- </div> -->
