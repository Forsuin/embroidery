<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import Svelecte from "svelecte";

	let { tag_options = $bindable(), search_query = $bindable() } = $props();

	function toggle(option: string): void {
		let index = search_query.indexOf(option);
		if (index > -1) {
			search_query = [
				...search_query.slice(0, index),
				...search_query.slice(index + 1),
			];
		} else {
			search_query = [...search_query, option];
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
					options={tag_options}
					bind:value={search_query}
					onCreateOption={(option: { value: string }) => {
						tag_options.push(option.value);
					}}
					searchProps={{ skipSort: true }}
				/>
			</Sidebar.GroupContent>
		</Sidebar.Group>
		<Sidebar.Separator />
		<Sidebar.Group>
			<Sidebar.GroupContent class="flex flex-wrap gap-2">
				{#each tag_options as option}
					<button
						type="button"
						class="chip {search_query.includes(option)
							? 'preset-filled'
							: 'preset-tonal'}"
						onclick={() => toggle(option)}>{option}</button
					>
				{/each}
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
</Sidebar.Root>
