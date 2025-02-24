<script lang="ts">
	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import Svelecte from "svelecte";

	let value: string;
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
					<Svelecte options={["One", "Two", "Three"]} bind:value />
				</Sidebar.Menu>
			</Sidebar.GroupContent>
		</Sidebar.Group>
	</Sidebar.Content>
</Sidebar.Root>
