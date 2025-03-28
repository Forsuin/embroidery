<script lang="ts">
    import * as Popover from "$lib/components/ui/popover";
    import { CirclePlus } from "lucide-svelte";
    import Button from "./ui/button/button.svelte";
    import Separator from "./ui/separator/separator.svelte";
    import { Badge } from "./ui/badge";

    let {
        options,
        selected_tags = $bindable(),
    }: { options: string[]; selected_tags: string[] } = $props();

    let open = $state(false);
</script>

<Popover.Root bind:open>
    <Popover.Trigger>
        <Button variant="outline" size="sm" class="h-8 border-dashed">
            <CirclePlus class="mr-2 h-4 w-4" />
            Tags
        </Button>

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
    </Popover.Trigger>
    <Popover.Content class="w-[200px] p-0" align="start" side="bottom">
        <Command
    </Popover.Content>
</Popover.Root>
