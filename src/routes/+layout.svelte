<script lang="ts">
	import "../app.css";

	import * as Sidebar from "$lib/components/ui/sidebar/index.js";
	import AppSidebar from "$lib/components/AppSidebar.svelte";
	import Menu from "$lib/components/Menu.svelte";
	import * as Resizable from "$lib/components/ui/resizable";

	let { children } = $props();
</script>

<div class="[--header-height:calc(theme(spacing.12))]">
	<Sidebar.Provider class="flex flex-col">
		<!-- <Menu /> -->
		<div id="AppSpace" class="min-w-screen flex flex-auto">
			<Resizable.PaneGroup
				direction="horizontal"
				class="min-w-screen min-h-[200px] items-stretch rounded-lg border"
			>
				<Resizable.Pane defaultSize={15} minSize={15}>
					<div class="flex h-full">
						<AppSidebar />
					</div>
				</Resizable.Pane>
				<Resizable.Handle />
				<Resizable.Pane defaultSize={75}>
					<div class="flex h-full">
						<main>
							{@render children?.()}
						</main>
					</div>
				</Resizable.Pane>
			</Resizable.PaneGroup>
		</div>
	</Sidebar.Provider>
</div>
