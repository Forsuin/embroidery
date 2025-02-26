<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import Svelecte from "svelecte";
	import { TagsInput } from "@skeletonlabs/skeleton-svelte";

	let options = $state([
		"First",
		"Second",
		"Third",
		"Fourth",
		"Fifth",
		"Sixth",
		"Seventh",
		"Eight",
		"Ninth",
		"Tenth",
	]);

	let selectedValues: string[] = $state([]);

	function toggle(option: string): void {
		let index = selectedValues.indexOf(option);
		if (index > -1) {
			selectedValues.splice(index, 1);
		} else {
			selectedValues.push(option);
		}
	}
</script>

<Sidebar.Root
	collapsible="none"
	class="top-(--header-height) h-[calc(100svh-var(--header-height))]! min-w-full"
	variant="inset"
>
	<Sidebar.Content>
		<Sidebar.Group>
			<!-- <Sidebar.GroupLabel>Search</Sidebar.GroupLabel> -->
			<Sidebar.GroupLabel>Tags</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
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
					{options}
					bind:value={selectedValues}
					onCreateOption={(option: { value: string }) => {
						options.push(option.value);
					}}
					searchProps={{ skipSort: true }}
				/>
			</Sidebar.GroupContent>
		</Sidebar.Group>
		<Sidebar.Separator />
		<Sidebar.Group>
			<Sidebar.GroupContent class="flex flex-wrap gap-3">
				{#each options as option}
					<button
						type="button"
						class="chip {selectedValues.includes(option)
							? 'preset-filled'
							: 'preset-tonal'}"
						onclick={() => toggle(option)}>{option}</button
					>
				{/each}
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
</Sidebar.Root>
