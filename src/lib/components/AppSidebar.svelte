<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import { invoke } from "@tauri-apps/api/core";
	import Svelecte from "svelecte";
	import * as Command from "$lib/components/ui/command"
	import {Button} from "$lib/components/ui/button";

	type Props = {
		tag_options: string[];
		search_query: string[];
		toggle: (option: string) => void;
	};

	let {
		tag_options = $bindable(),
		search_query = $bindable(),
		toggle,
	}: Props = $props();

</script>

<Sidebar.Root
	collapsible="none"
	class="top-(--header-height) h-[calc(100svh-var(--header-height))]! min-w-full"
	variant="inset"
>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Tags</Sidebar.GroupLabel>
			<Sidebar.GroupContent class="flex flex-col gap-2">
				<Svelecte
					searchable
					clearable
					creatable
					multiple
					allowEditing
					keepCreated={true}
					creatablePrefix=""
					placeholder="Search Tags"
					closeAfterSelect={true}
					options={tag_options}
					bind:value={search_query}
					onCreateOption={(option: { value: string }) => {
						tag_options.push(option.value);
						invoke("add_tag", { new_tag: option.value });
					}}
					searchProps={{ skipSort: true }}
				/>
				<Button variant="outline" size="sm" class="max-w-1/2">Advanced Search</Button>
			</Sidebar.GroupContent>
		</Sidebar.Group>
		<Sidebar.Separator />
		<Sidebar.Group>
			<Sidebar.GroupContent class="flex flex-wrap gap-2 ">
				{#each tag_options as option}
					<button
						type="button"
						class="chip {search_query.includes(option)
							? 'preset-filled'
							: 'preset-tonal'}"
						onclick={() => toggle(option)}
						>{option}
					</button>
				{/each}
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
</Sidebar.Root>
