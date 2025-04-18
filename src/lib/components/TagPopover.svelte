<script lang="ts">
    import * as Popover from "$lib/components/ui/popover";
    import { Check, CirclePlus } from "lucide-svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import Separator from "$lib/components/ui/separator/separator.svelte";
    import { Badge } from "./ui/badge";
    import * as Command from "$lib/components/ui/command";
    import { cn } from "$lib/utils";
    import Checkbox from "$lib/components/ui/checkbox/checkbox.svelte";
    import Svelecte from "svelecte";
    import { invoke } from "@tauri-apps/api/core";

    let {
        options = $bindable(),
        onSelectTag,
        onCreateTag,
    }: { options: string[]; onSelectTag: (tags: string[]) => void, onCreateTag: (new_tag: string) => void } = $props();

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
            onChange={() => {
                onSelectTag(selected_tags);
            }}
            onCreateOption={(option: { value: string }) => {
                onCreateTag(option.value);
                options = [...options, option.value];
                invoke("add_tag", { new_tag: option.value });
            }}
            searchProps={{ skipSort: true }}
            highlightFirstItem={true}
        />
    </Popover.Content>
</Popover.Root>
