<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import Button from "$lib/components/ui/button/button.svelte";
    import * as Table from "$lib/components/ui/table/index.js";
    import {
        createColumnHelper,
        createSvelteTable,
        getCoreRowModel,
        renderComponent,
        type RowSelectionState,
        type Updater,
    } from "$lib/table";
    import { invoke } from "@tauri-apps/api/core";
    import {emit, listen} from "@tauri-apps/api/event";
    import TagPopover from "./TagPopover.svelte";
    import FlexRender from "$lib/table/flex-render.svelte";
    import TableCheckbox from "./TableCheckbox.svelte";
    import TableDelete from "./TableDelete.svelte";
    import TableName from "./TableName.svelte";

    let {
        isOpen = $bindable(),
        tags = $bindable(),
    }: { isOpen: boolean; tags: string[] } = $props();


    type FileImport = {
        name: string;
        path: string;
        tags: string[];
    };

    let fileImports: FileImport[] = $state([]);

    const colHelp = createColumnHelper<FileImport>();

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
            cell: ({ row }) =>
                renderComponent(TagPopover, {
                    options: tags,
                    onSelectTag: (tags: string[]) => {
                        fileImports[row.index].tags = tags;
                    },
                    onCreateTag: (new_tag: string) => {
                        tags = [...tags, new_tag];
                    }
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

    function onRowSelectionChange(updater: Updater<RowSelectionState>) {
        // Update the selection state by reassigning the $state
        if (updater instanceof Function) {
            rowSelectionState = updater(rowSelectionState);
        } else {
            rowSelectionState = updater;
        }
    }

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

    listen<FileImport[]>("file-import-populate", (event) => {
        fileImports = event.payload;
    });
</script>

<Dialog.Root bind:open={isOpen} onOpenChange={() => {fileImports = []}}>
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
            <Button
                type="button"
                variant="secondary"
                onclick={async () => {
                    await invoke("import_files", { files: fileImports }).then(
                        () => {
                            isOpen = false;
                            fileImports = [];
                        },
                    );

                    await emit("refresh-patterns");
                }}>Import</Button
            >
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
