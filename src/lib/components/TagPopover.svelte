<script lang="ts">
    import * as Popover from "$lib/components/ui/popover";
    import { Check, CirclePlus } from "lucide-svelte";
    import Button from "./ui/button/button.svelte";
    import Separator from "./ui/separator/separator.svelte";
    import { Badge } from "./ui/badge";
    import * as Command from "$lib/components/ui/command";
    import { cn } from "$lib/utils";
    import Checkbox from "./ui/checkbox/checkbox.svelte";
    import Svelecte from "svelecte";
    import { invoke } from "@tauri-apps/api/core";

    let {
        options = $bindable(),
        onSelectTag,
    }: { options: string[]; onSelectTag: (tags: string[]) => void } = $props();

    let selected_tags: string[] = $state([]);
    let open = $state(false);
</script>

<Popover.Root bind:open>
    <Popover.Trigger>
        <Button variant="outline" size="sm" class="h-8 border-dashed">
            <CirclePlus class="mr-2 h-4 w-4" />
            Tags

            {#if selected_tags.length > 0}
                <Separator orientation="vertical" class="mx-2 h-4" />
                <Badge
                    variant="secondary"
                    class="rounded-sm px-1 font-normal lg:hidden"
                >
                    {selected_tags.length}
                </Badge>
                <div class="hidden space-x-1 lg:flex">
                    {#if selected_tags.length > 2}
                        <Badge
                            variant="secondary"
                            class="rounded-sm px-1 font-normal"
                        >
                            {selected_tags.length} Selected
                        </Badge>
                    {:else}
                        {#each selected_tags as tag}
                            <Badge
                                variant="secondary"
                                class="rounded-sm px-1 font-normal"
                            >
                                {tag}
                            </Badge>
                        {/each}
                    {/if}
                </div>
            {/if}
        </Button>
    </Popover.Trigger>
    <Popover.Content class="w-[200px] p-0" align="start" side="bottom">
        <!-- <Command.Root>
            <Command.Input
                placeholder="Add tags..."
                bind:value={command_input}
            />
            <Command.List>
                <Command.Empty>No results found.</Command.Empty>
                <Command.Group>
                    {#each options as option}
                        <Command.Item
                            value={option}
                            onSelect={() => {
                                handleSelect(option);
                            }}
                        >
                            <div
                                class={cn(
                                    "border-primary mr-2 flex h-4 w-4 items-center justify-center rounded-sm border",
                                    selected_tags.includes(option)
                                        ? "bg-primary text-primary-foreground"
                                        : "opacity-50 [&_svg]:invisible",
                                )}
                            >
                                <Checkbox checked={isOptionSelected(option)} />
                            </div>
                            <span>
                                {option}
                            </span>
                        </Command.Item>
                    {/each}
                    {#if potential_new_tag}
                        <Command.Item
                            value={command_input}
                            onSelect={() => {
                                console.log(command_input);
                            }}
                        >
                            <span>
                                {command_input}
                            </span>
                        </Command.Item>
                    {/if}
                </Command.Group>
                {#if selected_tags.length > 0}
                    <Command.Separator />
                    <Command.Item
                        class="justify-center text-center"
                        onSelect={() => {
                            console.log("Clear");
                            selected_tags = [];
                        }}>Clear Items</Command.Item
                    >
                {/if}
            </Command.List>
        </Command.Root> -->

        <Svelecte
            searchable
            clearable
            creatable
            multiple
            allowEditing
            keepCreated={true}
            creatablePrefix=""
            placeholder="Add tags..."
            closeAfterSelect={true}
            {options}
            bind:value={selected_tags}
            onCreateOption={(option: { value: string }) => {
                options.push(option.value);
                invoke("add_tag", { new_tag: option.value });
            }}
            searchProps={{ skipSort: true }}
            highlightFirstItem={true}
        />
    </Popover.Content>
</Popover.Root>
