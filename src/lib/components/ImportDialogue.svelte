<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import Button from "./ui/button/button.svelte";
    import * as Table from "$lib/components/ui/table/index.js";
    import ScrollArea from "./ui/scroll-area/scroll-area.svelte";
    import { Trash2 } from "lucide-svelte";

    let {
        isOpen = $bindable(),
        dragDropFiles = $bindable(),
    }: { isOpen: boolean; dragDropFiles: FileList | undefined } = $props();

    let filelist: FileList | undefined = $derived(dragDropFiles);
    let files: File[] = $derived(Array.from(filelist ?? []));
</script>

<Dialog.Root bind:open={isOpen}>
    <input
        bind:files={dragDropFiles}
        type="file"
        id="fileElem"
        multiple
        style="display:none"
    />
    <Dialog.Content class="max-h-3/5 max-w-2/5 overflow-auto">
        <Dialog.Header>
            <Dialog.Title>Import Files</Dialog.Title>
        </Dialog.Header>
        {#if !filelist}
            <Button
                type="button"
                variant="outline"
                onclick={() => {
                    document.getElementById("fileElem")?.click();
                }}>Select Files</Button
            >
        {:else}
            <!-- <ScrollArea class="h-full w-full"> -->
            <Table.Root>
                <Table.Caption>Selected Files: {files.length}</Table.Caption>
                <Table.Header>
                    <Table.Head class="w-[100px]">Name</Table.Head>
                    <Table.Head>Tags</Table.Head>
                    <Table.Head class="text-right">Cancel</Table.Head>
                </Table.Header>
                <Table.Body>
                    {#each files as file}
                        <Table.Row>
                            <Table.Cell class="w-[100px]"
                                >{file.name}</Table.Cell
                            >
                            <Table.Cell>{file.size}</Table.Cell>
                            <Table.Cell class="text-right">
                                <Button variant="secondary" size="icon">
                                    <Trash2 />
                                </Button>
                            </Table.Cell>
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
            <!-- </ScrollArea> -->
        {/if}
        <Dialog.Footer>
            <Button type="button" variant="secondary">Import</Button>
            <Button
                type="button"
                onclick={() => {
                    isOpen = false;
                    dragDropFiles = undefined;
                }}>Cancel</Button
            >
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
