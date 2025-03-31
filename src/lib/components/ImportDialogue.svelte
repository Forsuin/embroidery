<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import Button from "./ui/button/button.svelte";
    import * as Table from "$lib/components/ui/table/index.js";
    import { Trash2 } from "lucide-svelte";
    import {
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

    const columnDefs = [
        colHelp.display({
            header: "Select",
            cell: ({ row }) =>
                renderComponent(TableCheckbox, {
                    checked: row.getIsSelected(),
                    onchange: () => {
                        row.toggleSelected();
                    },
                }),
        }),
        colHelp.accessor("name", {
            header: "Name",
            cell: ({ cell }) => renderSnippet(left_text, cell.getValue()),
        }),
        // colHelp.accessor("path", { header: "Path" }),
        colHelp.accessor("tags", {
            header: "Tags",
            cell: ({ cell }) =>
                renderComponent(TagPopover, {
                    options: tags,
                    selected_tags: cell.getValue(),
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

    let fileImports: FileImport[] = $state([]);

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
            <table>
                <thead>
                    <tr>
                        {#each table.getHeaderGroups() as headerGroup}
                            {#each headerGroup.headers as header}
                                <th>
                                    <FlexRender
                                        content={header.column.columnDef.header}
                                        context={header.getContext()}
                                    />
                                </th>
                            {/each}
                        {/each}
                    </tr>
                </thead>
                <tbody>
                    {#each table.getRowModel().rows as row}
                        <tr>
                            {#each row.getVisibleCells() as cell}
                                <td>
                                    <FlexRender
                                        content={cell.column.columnDef.cell}
                                        context={cell.getContext()}
                                    />
                                </td>
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
