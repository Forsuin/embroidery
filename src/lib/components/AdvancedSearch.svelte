<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import { Button } from "$lib/components/ui/button";
    import * as Select from "$lib/components/ui/select";
    import {Separator} from "$lib/components/ui/separator";
    import {Badge} from "$lib/components/ui/badge";
    import SelectionBadge from "$lib/components/SelectionBadge.svelte";
    import {Label} from "$lib/components/ui/label";
    import {Input} from "$lib/components/ui/input";
    import type {SearchQuery} from "$lib/utils";

    let top_tags: string[] = [];


    type Props = {
        tag_options: string[];
        search_query: SearchQuery;
        search_function: () => void;
    };

    let { tag_options, search_query = $bindable(), search_function }: Props = $props();

</script>

<Sidebar.Root collapsible="none"
              class="top-(--header-height) h-[calc(100svh-var(--header-height))]! min-w-full"
              variant="inset">
    <Sidebar.Content>
        <Sidebar.Group>
            <Sidebar.GroupContent class="flex flex-col gap-2 px-2">
                <div id="includes">
                    <Label for="includeTags" >Include</Label>
                    <div class="flex gap-2 items-center">
                        <SelectionBadge bind:selection={search_query.include_tags} options={tag_options} trigger_label="Include Tags..." id="includeTags"/>
                        <Button variant="secondary" size="sm" onclick={() => {search_query.include_tags = []}}>Clear tags</Button>
                    </div>
                </div>

                <div id="excludes">
                    <Label for="includeTags">Exclude</Label>
                    <div class="flex gap-2 items-center">
                        <SelectionBadge bind:selection={search_query.exclude_tags} options={tag_options} trigger_label="Exclude Tags..." id="excludeTags"/>
                        <Button variant="secondary" size="sm" onclick={() => {search_query.exclude_tags = []}}>Clear tags</Button>
                    </div>
                </div>

                <div id="textSearch">
                    <Label for="textSearchTags">Search within results</Label>
                    <Input type="text" id="textSearchTags" class="h-7"/>
                </div>

                <Button variant="default" size="sm" onclick={() => search_function()}>Sort and Filter</Button>
            </Sidebar.GroupContent>
        </Sidebar.Group>
    </Sidebar.Content>
</Sidebar.Root>