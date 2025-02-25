<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import Svelecte from "svelecte";

	let options = [
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
	];
</script>

<Sidebar.Root
	collapsible="none"
	class="top-[--header-height] !h-[calc(100svh-var(--header-height))] min-w-full"
>
	<Sidebar.Content>
		<Sidebar.Group>
			<Sidebar.GroupLabel>Search</Sidebar.GroupLabel>
			<Sidebar.GroupContent>
				<Sidebar.Menu>
					<Svelecte
						searchable
						clearable
						multiple
						allowEditing
						keepCreated
						placeholder="Search Tags"
						{options}
					/>
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
		<Sidebar.Separator />
		<Sidebar.Group>
			<Sidebar.GroupLabel>Tags</Sidebar.GroupLabel>
			<Sidebar.GroupContent class="flex flex-wrap">
				{#each options as option}
					<div class="tag">option</div>
				{/each}
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
</Sidebar.Root>
