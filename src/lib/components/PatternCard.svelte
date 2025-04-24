<script lang="ts">
    import * as Card from "$lib/components/ui/card";
    import * as ContextMenu from "$lib/components/ui/context-menu";
    import {cn, fileExplorerPrompt} from "$lib/utils";
    import {Badge} from "$lib/components/ui/badge";
    import {emit} from "@tauri-apps/api/event";
    import type {PatternTags} from "$lib/types";

    type Props = {
        pattern_tag: PatternTags;
    }

    let {pattern_tag}: Props = $props();

    function openPatternDetails() {
        emit("openPatternEdit", { pattern_tag: pattern_tag });
    }
</script>

<div>
    <ContextMenu.Root>
        <ContextMenu.Trigger>
            <Card.Root class="hover:bg-accent transition-all" ondblclick={() => openPatternDetails()}>
                <Card.Header>
                    <Card.Title>
                        {pattern_tag.pattern.name}
                    </Card.Title>
                    <Card.Description
                    >Pattern ID: {pattern_tag.pattern
                        .id}</Card.Description
                    >
                </Card.Header>
                <Card.Content>Card content here later</Card.Content>
                <Card.Footer>
                    <div class="flex flex-wrap gap-1">
                        {#each pattern_tag.tags as tag}
                            <Badge variant="secondary">{tag.name}</Badge>
                        {/each}
                    </div>
                </Card.Footer>
            </Card.Root>
        </ContextMenu.Trigger>
        <ContextMenu.Content>
            <ContextMenu.Item onSelect={() => { openPatternDetails() }}>Edit</ContextMenu.Item>
            <ContextMenu.Item>{fileExplorerPrompt()}</ContextMenu.Item>
            <ContextMenu.Item>Delete</ContextMenu.Item>
        </ContextMenu.Content>
    </ContextMenu.Root>
</div>