<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import Button from "./ui/button/button.svelte";
    import * as Table from "$lib/components/ui/table/index.js";
    import { Trash2 } from "lucide-svelte";
    import {
        ColumnFaceting,
        createColumnHelper,
        createSvelteTable,
        getCoreRowModel,
        renderComponent,
        renderSnippet,
        type RowSelectionState,
        type Updater,
    } from "$lib/table";
    import { open } from "@tauri-apps/plugin-dialog";
    import { path } from "@tauri-apps/api";
    import { invoke } from "@tauri-apps/api/core";
    import { listen } from "@tauri-apps/api/event";
    import TagPopover from "./TagPopover.svelte";
    import FlexRender from "$lib/table/flex-render.svelte";
    import { createRawSnippet } from "svelte";
    import TableCheckbox from "./TableCheckbox.svelte";
    import TableDelete from "./TableDelete.svelte";
    import TableName from "./TableName.svelte";

    type FileImport = {
        name: string;
        path: string;
        tags: string[];
    };

    let { isOpen = $bindable(), tags }: { isOpen: boolean; tags: string[] } =
        $props();

    const colHelp = createColumnHelper<FileImport>();

    const left_text = createRawSnippet<[string]>((text) => {
        return {
            render: () => `<div class="text-left">${text()}</div>`,
        };
    });

    function deleteRow(row: number) {
        fileImports = fileImports.filter((_, index) => index !== row);
    }

    function renameImport(name: string, row: number) {
        fileImports[row].name = name;
    }

    const columnDefs = [
        colHelp.display({
            id: "checkbox-table-column",
            header: ({ table }) =>
                renderComponent(TableCheckbox, {
                    checked: table.getIsAllRowsSelected(),
                    indeterminate:
                        table.getIsSomeRowsSelected() &&
                        !table.getIsAllRowsSelected(),
                    onCheckedChange: (value) =>
                        table.toggleAllRowsSelected(!!value),
                }),
            cell: ({ row }) =>
                renderComponent(TableCheckbox, {
                    checked: row.getIsSelected(),
                    onCheckedChange: (value) => row.toggleSelected(!!value),
                }),
        }),
        colHelp.accessor("name", {
            header: "Name",
            cell: ({ cell, row }) =>
                renderComponent(TableName, {
                    value: cell.getValue(),
                    oninput: renameImport,
                    index: row.index,
                }),
        }),
        ,
        colHelp.accessor("tags", {
            header: "Tags",
            cell: ({ cell }) =>
                renderComponent(TagPopover, {
                    options: tags,
                    selected_tags: cell.getValue(),
                }),
        }),
        colHelp.display({
            id: "delete-table-column",
            cell: ({ row }) =>
                renderComponent(TableDelete, {
                    onclick: () => {
                        deleteRow(row.index);
                    },
                }),
        }),
    ];

    let rowSelectionState: RowSelectionState = $state({});

    $inspect(rowSelectionState);

    function onRowSelectionChange(updater: Updater<RowSelectionState>) {
        // Update the selection state by reassigning the $state
        if (updater instanceof Function) {
            rowSelectionState = updater(rowSelectionState);
        } else {
            rowSelectionState = updater;
        }
    }

    let fileImports: FileImport[] = $state([]);

    $inspect("File imports: ", fileImports);

    // console.log("files: ", $inspect(files));
    // console.log("fileImports", $inspect(fileImports));

    const table = createSvelteTable({
        get data() {
            return fileImports;
        },
        columns: columnDefs,
        state: {
            get rowSelection() {
                return rowSelectionState;
            },
        },
        onRowSelectionChange: onRowSelectionChange,
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
    <Dialog.Content class="max-h-3/5  overflow-auto">
        <Dialog.Header>
            <Dialog.Title>Import Files</Dialog.Title>
        </Dialog.Header>
        {#if fileImports.length == 0}
            <Button
                type="button"
                variant="outline"
                onclick={async () => {
                    invoke("select_file_dialog");
                }}>Select Files</Button
            >
        {:else}
            <Table.Root>
                <Table.Header>
                    {#each table.getHeaderGroups() as headerGroup}
                        <Table.Row>
                            {#each headerGroup.headers as header}
                                <Table.Head>
                                    <FlexRender
                                        content={header.column.columnDef.header}
                                        context={header.getContext()}
                                    />
                                </Table.Head>
                            {/each}
                        </Table.Row>
                    {/each}
                </Table.Header>
                <Table.Body>
                    {#each table.getRowModel().rows as row}
                        <Table.Row>
                            {#each row.getVisibleCells() as cell}
                                <Table.Cell>
                                    <FlexRender
                                        content={cell.column.columnDef.cell}
                                        context={cell.getContext()}
                                    />
                                </Table.Cell>
                            {/each}
                        </Table.Row>
                    {/each}
                </Table.Body>
            </Table.Root>
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
