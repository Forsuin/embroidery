<script lang="ts">
    import * as Popover from "$lib/components/ui/popover";
    import { Check, CirclePlus } from "lucide-svelte";
    import Button from "./ui/button/button.svelte";
    import Separator from "./ui/separator/separator.svelte";
    import { Badge } from "./ui/badge";
    import * as Command from "$lib/components/ui/command";
    import { cn } from "$lib/utils";
    import Checkbox from "./ui/checkbox/checkbox.svelte";

    let {
        options,
        selected_tags = $bindable(),
    }: { options: string[]; selected_tags: string[] } = $props();

    let open = $state(false);

    function handleSelect(currentValue: string) {
        console.log("Current value: ", currentValue);
        if (
            Array.isArray(selected_tags) &&
            selected_tags.includes(currentValue)
        ) {
            selected_tags = selected_tags.filter((v) => v !== currentValue);
        } else {
            selected_tags = [
                ...(Array.isArray(selected_tags) ? selected_tags : []),
                currentValue,
            ];
        }
    }

    $inspect(options, selected_tags);
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
        <Command.Root>
            <Command.Input placeholder="Add tags..." />
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
                                <Checkbox />
                            </div>
                            <span>
                                {option}
                            </span>
                        </Command.Item>
                    {/each}
                </Command.Group>
                {#if options.length > 0}
                    <Command.Separator />
                    <Command.Item
                        class="justify-center text-center"
                        onselect={() => (selected_tags = [])}
                        >Clear Options</Command.Item
                    >
                {/if}
            </Command.List>
        </Command.Root>
    </Popover.Content>
</Popover.Root>
