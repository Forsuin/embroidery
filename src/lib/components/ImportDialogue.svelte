<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import Button from "./ui/button/button.svelte";
    import { open } from "@tauri-apps/plugin-dialog";

    let { isOpen = $bindable() } = $props();

    let files: string[] = $state([]);

    async function showFileOpenDialogue() {
        files = await window.showOpenFilePicker({ multiple: true });

        console.log(files);
    }
</script>

<!-- <button onclick={() => (isOpen = true)}>Open Dialogue</button> -->

<Dialog.Root bind:open={isOpen}>
    <Dialog.Content>
        <Dialog.Header>
            <Dialog.Title>Import Files</Dialog.Title>
        </Dialog.Header>
        <div class="flex">
            
        </div>
        <Button type="button" variant="outline" onclick={showFileOpenDialogue}
            >Import</Button
        >
        <Dialog.Footer>
            <Button type="button" variant="secondary">Import</Button>
            <Button
                type="button"
                onclick={() => {
                    files = [];
                    isOpen = false;
                }}>Cancel</Button
            >
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
