<script lang="ts">
	import { onDestroy, onMount } from "svelte";
	import "@tauri-apps/api/menu";
	import { Submenu, Menu, PredefinedMenuItem } from "@tauri-apps/api/menu";
	import { emit } from "@tauri-apps/api/event";
	import { platform } from "@tauri-apps/plugin-os";

	let { children } = $props();

	async function onKeyDown(event: KeyboardEvent) {
		let modifiers = "";
		if (event.ctrlKey) {
			modifiers += "Ctrl + ";
		}
		if (event.metaKey) {
			modifiers += "Meta + ";
		}
		if (event.shiftKey) {
			modifiers += "Shift + ";
		}
		if (event.altKey) {
			modifiers += "Alt + ";
		}

		const shortcut: string = modifiers + event.key.toUpperCase();

		console.log(`shortcut: ${shortcut}`);

		switch (shortcut) {
			case "Ctrl + I": {
				// console.log("import shortcut");
				await emit("import");
			}
		}

		// console.log(`pressed the ${modifiers}${event.key} key`);
	}

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

		const file_submenu = await Submenu.new({
			items: [
				{
					id: "import",
					text: "Import",
					action: () => {
						console.log("import pressed");
					},
					accelerator: "CmdOrCtrl+I",
				},
				{
					id: "reveal",
					text: file_explorer_prompt,
					action: () => {
						console.log("reveal pressed");
					},
					accelerator: "CmdOrCtrl+Alt+R",
				},
				{
					item: "Separator",
				},
				{
					id: "settings",
					text: "Settings",
					action: () => {
						console.log("settings presesd");
					},
					accelerator: "CmdOrCtrl+,",
				},
				{
					item: "Separator",
				},
				{
					id: "quit",
					text: "Quit",
					action: () => {
						console.log("quit pressed");
					},
					accelerator: "CmdOrCtrl+Q",
				},
			],
			text: "File",
		});

		const edit_submenu = await Submenu.new({
			items: [
				{
					id: "undo",
					text: "Undo",
					accelerator: "CmdOrCtrl+Z",
					action: () => {
						console.log("undo pressed");
					},
				},
				{
					id: "redo",
					text: "Redo",
					accelerator: "CmdOrCtrl+Shift+Z",
					action: () => {
						console.log("redo pressed");
					},
				},
				{
					item: "Separator",
				},
				{
					id: "cut",
					text: "Cut",
					accelerator: "CmdOrCtrl+X",
					action: () => {
						console.log("cut pressed");
					},
				},
				{
					id: "copy",
					text: "Copy",
					accelerator: "CmdOrCtrl+C",
					action: () => {
						console.log("copy pressed");
					},
				},
				{
					id: "paste",
					text: "Paste",
					accelerator: "CmdOrCtrl+V",
					action: () => {
						console.log("paste pressed");
					},
				},
				{
					item: "Separator",
				},
				{
					id: "select all",
					text: "Select All",
					accelerator: "CmdOrCtrl+A",
					action: () => {
						console.log("select all pressed");
					},
				},
			],
			text: "Edit",
		});

		const menu = await Menu.new({
			items: [
				// {
				// 	id: "quit",
				// 	text: "Quit",
				// 	action: () => {
				// 		console.log("quit pressed");
				// 	},
				// },
				file_submenu,
				edit_submenu,
			],
		});

		menu.setAsAppMenu();
	});

	onDestroy(() => {
		if (platform() == "windows") {
			alert("destroyed event listener");
			window.removeEventListener("keydown", onKeyDown);
		}
	});
</script>

{@render children()}
