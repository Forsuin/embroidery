<script lang="ts">
    import * as Select from "$lib/components/ui/select";
    import {Badge} from "$lib/components/ui/badge";

    type Option = {
        value: string;
        label: string;
    };

    type Props = {
        selection: string[];
        options: Option[];
        trigger_label?: string;
        id?: string;
    };

    let {selection = $bindable(), options, trigger_label = "Select Options...", id = ""}: Props = $props();
</script>
<div {id}>
    <Select.Root type="multiple" bind:value={selection}>
        <Select.Trigger>
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
        </Select.Trigger>
        <Select.Content>
            {#each options as option}
                <Select.Item value={option.value}>{option.label}</Select.Item>
            {/each}
        </Select.Content>
    </Select.Root>
</div>