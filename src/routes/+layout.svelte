<script lang="ts">
	import "@tauri-apps/api/menu";

	let { children } = $props();
</script>

<!-- <div
	id="drop-area"
	ondrop={handleDrop}
	ondragover={dragOverHandler}
	role="none"
> -->

<main>
	{@render children()}
</main>
<!-- </div> -->
