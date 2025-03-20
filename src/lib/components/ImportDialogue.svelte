<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import Button from "./ui/button/button.svelte";
    import * as Table from "$lib/components/ui/table/index.js";
    import ScrollArea from "./ui/scroll-area/scroll-area.svelte";

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
    <Dialog.Content>
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
            <ScrollArea class="h-[200px] rounded-md">
                <Table.Root>
                    <Table.Caption>Selected Files</Table.Caption>
                    <Table.Header>
                        <Table.Head>Name</Table.Head>
                        <Table.Head>Size</Table.Head>
                    </Table.Header>
                    <Table.Body>
                        {#each files as file}
                            <Table.Row>
                                <Table.Cell>{file.name}</Table.Cell>
                                <Table.Cell>{file.size}</Table.Cell>
                            </Table.Row>
                        {/each}
                    </Table.Body>
                </Table.Root>
            </ScrollArea>
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
