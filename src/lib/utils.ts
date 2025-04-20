import { type ClassValue, clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export function cn(...inputs: ClassValue[]) {
	return twMerge(clsx(inputs));
}

export type SearchQuery = {
	include_tags: string[];
	exclude_tags: string[];
	custom_query: string;
};
