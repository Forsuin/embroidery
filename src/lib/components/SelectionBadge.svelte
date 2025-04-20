<script lang="ts">
    import * as Select from "$lib/components/ui/select";
    import {Badge} from "$lib/components/ui/badge";
    import * as Command from "$lib/components/ui/command/index.js";
    import * as Popover from "$lib/components/ui/popover/index.js";
    import {Button} from "$lib/components/ui/button";
    import Check from "@lucide/svelte/icons/check";
    import {cn} from "$lib/utils";

    type Props = {
        selection: string[];
        options: string[];
        trigger_label?: string;
        id?: string;
    };

    let {selection = $bindable(), options, trigger_label = "Select Options...", id = ""}: Props = $props();

    let open: boolean = $state(false);
    let triggerRef = $state<HTMLButtonElement>(null!);

    function toggleFromSelection(value: string) {
        let index = selection.indexOf(value);

        if (index > -1) {
            selection = [
                ...selection.slice(0, index),
                ...selection.slice(index + 1),
            ];
        } else {
            selection = [...selection, value];
        }
    }
</script>

<div {id}>
    <Popover.Root bind:open>
        <Popover.Trigger bind:ref={triggerRef}>
            {#snippet child({props})}
                <Button variant="outline" {...props} role="combobox" aria-expanded={open} class="font-normal">
                    {trigger_label}
                    {#if selection.length > 0}
                        <div class="hidden space-x-1 lg:flex">
                            {#if selection.length > 2}
                                <Badge variant="secondary" class="rounded-sm px-1 font-normal">
                                    {selection.length} Selected
                                </Badge>
                            {:else}
                                {#each selection as tag}
                                    <Badge variant="secondary" class="rounded-sm px-1 font-normal">
                                        {tag}
                                    </Badge>
                                {/each}
                            {/if}
                        </div>
                    {/if}
                </Button>
            {/snippet}
        </Popover.Trigger>
        <Popover.Content>
            <Command.Root>
                <Command.Input placeholder={trigger_label}/>
                <Command.List>
                    {#each options as option}
                        <Command.Item value={option} onSelect={() => { toggleFromSelection(option); }}>
                            <Check class={cn("mr-2 size-4", !selection.includes(option) && "text-transparent")}/>
                            {option}
                        </Command.Item>
                    {/each}
                </Command.List>
            </Command.Root>
        </Popover.Content>
    </Popover.Root>

</div>

<!--<div {id}>-->
<!--    <Select.Root type="multiple" bind:value={selection}>-->
<!--        <Select.Trigger>-->
<!--            {trigger_label}-->
<!--            {#if selection.length > 0}-->
<!--                <div class="hidden space-x-1 lg:flex">-->
<!--                    {#if selection.length > 2}-->
<!--                        <Badge variant="secondary" class="rounded-sm px-1 font-normal">-->
<!--                            {selection.length} Selected-->
<!--                        </Badge>-->
<!--                    {:else}-->
<!--                        {#each selection as tag}-->
<!--                            <Badge variant="secondary" class="rounded-sm px-1 font-normal">-->
<!--                                {tag}-->
<!--                            </Badge>-->
<!--                        {/each}-->
<!--                    {/if}-->
<!--                </div>-->
<!--            {/if}-->
<!--        </Select.Trigger>-->
<!--        <Select.Content>-->
<!--            {#each options as option}-->
<!--                <Select.Item value={option}>{option}</Select.Item>-->
<!--            {/each}-->
<!--        </Select.Content>-->
<!--    </Select.Root>-->
<!--</div>-->
