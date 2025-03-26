<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import Button from "./ui/button/button.svelte";
    import * as Table from "$lib/components/ui/table/index.js";
    import { Trash2 } from "lucide-svelte";
    import {
        createColumnHelper,
        createSvelteTable,
        getCoreRowModel,
    } from "$lib/table";
    import { open } from "@tauri-apps/plugin-dialog";
    import { path } from "@tauri-apps/api";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";

    type FileImport = {
        name: string;
        path: string;
        tags: string[];
    };

    const colHelp = createColumnHelper<FileImport>();

    const columnDefs = [
        colHelp.accessor("name", { header: "Name" }),
        colHelp.accessor("path", { header: "Path" }),
        colHelp.accessor("tags", { header: "Tags" }),
    ];

    let { isOpen = $bindable() }: { isOpen: boolean } = $props();

    let fileImports: FileImport[] = $state([]);

    // console.log("files: ", $inspect(files));
    // console.log("fileImports", $inspect(fileImports));

    const table = createSvelteTable({
        get data() {
            return fileImports;
        },
        columns: columnDefs,
        getCoreRowModel: getCoreRowModel(),
    });

    listen<FileImport[]>("file-import-finished", (event) => {
        fileImports = event.payload;
    });
</script>

<Dialog.Root bind:open={isOpen}>
    <!-- <input
        bind:files={dragDropFiles}
        type="file"
        id="fileElem"
        multiple
        style="display:none"
    /> -->
    <Dialog.Content class="max-h-3/5 max-w-2/5 overflow-auto">
        <Dialog.Header>
            <Dialog.Title>Import Files</Dialog.Title>
        </Dialog.Header>
        {#if fileImports.length == 0}
            <Button
                type="button"
                variant="outline"
                onclick={async () => {
                    // document.getElementById("fileElem")?.click();
                    // let selected = await open({
                    //     title: "Select patterns to import",
                    //     multiple: true,
                    //     directory: false,
                    //     defaultPath: await path.pictureDir(),
                    // });
                    // console.log("Selected: ", selected);

                    invoke("select_file_dialog");
                }}>Select Files</Button
            >
        {:else}
            <!-- <ScrollArea class="h-full w-full"> -->
            <!-- <Table.Root>
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
            </Table.Root> -->
            <!-- </ScrollArea> -->

            <table>
                <thead>
                    <tr>
                        {#each table.getHeaderGroups() as headerGroup}
                            {#each headerGroup.headers as header}
                                <th>{header.column.columnDef.header}</th>
                            {/each}
                        {/each}
                    </tr>
                </thead>
                <tbody>
                    {#each table.getRowModel().rows as row}
                        <tr>
                            {#each row.getVisibleCells() as cell}
                                <td>{cell.getValue()}</td>
                            {/each}
                        </tr>
                    {/each}
                </tbody>
            </table>
        {/if}
        <Dialog.Footer>
            <Button type="button" variant="secondary">Import</Button>
            <Button
                type="button"
                onclick={() => {
                    isOpen = false;
                    fileImports = [];
                }}>Cancel</Button
            >
        </Dialog.Footer>
    </Dialog.Content>
</Dialog.Root>
