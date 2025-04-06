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

    function handleSelect(currentValue: string) {
        if (selected_tags.includes(currentValue)) {
            selected_tags = selected_tags.filter((v) => v !== currentValue);
        } else {
            selected_tags = [...selected_tags, currentValue];
        }

        onSelectTag(selected_tags);
    }

    function isOptionSelected(option: string): boolean {
        return selected_tags.includes(option);
    }

    let command_input: string = $state("");

    let potential_new_tag: boolean = $derived.by(() => {
        return command_input !== "" && !options.includes(command_input);
    });
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
        <Svelecte
            searchable
            clearable
            creatable
            multiple
            allowEditing
            keepCreated={true}
            creatablePrefix=""
            placeholder="Add tags..."
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
