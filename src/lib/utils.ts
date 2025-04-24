import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";
import {platform} from "@tauri-apps/plugin-os";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export type SearchQuery = {
	include_tags: string[];
	exclude_tags: string[];
	custom_query: string;
};

export function fileExplorerPrompt() {
	switch (platform()) {
		case "windows":
			return "Reveal in File Explorer";
		case "macos":
			return "Show in Finder";
		default:
			return "Show in File Manager";
	}
}