<script lang="ts">
    import * as Sidebar from "$lib/components/ui/sidebar/index.js";
    import { Button } from "$lib/components/ui/button";
    import * as Select from "$lib/components/ui/select";
    import {Separator} from "$lib/components/ui/separator";
    import {Badge} from "$lib/components/ui/badge";
    import SelectionBadge from "$lib/components/SelectionBadge.svelte";
    import {Label} from "$lib/components/ui/label";
    import {Input} from "$lib/components/ui/input";

    let includeTags: string[] = [];
    let excludeTags: string[] = [];
    let extraSearch: string = "";

    let top_tags: string[] = [];

    const test_options = [
        { value: "apple", label: "Apple" },
        { value: "banana", label: "Banana" },
        { value: "blueberry", label: "Blueberry" },
        { value: "grapes", label: "Grapes" },
        { value: "pineapple", label: "Pineapple" }
    ]
</script>

<Sidebar.Root collapsible="none"
              class="top-(--header-height) h-[calc(100svh-var(--header-height))]! min-w-full"
              variant="inset">

    <Sidebar.Content>
        <Sidebar.Group>
            <Sidebar.GroupContent class="flex flex-col gap-2 px-2">
                <div id="includes">
                    <Label for="includeTags">Include Tags</Label>
                    <SelectionBadge bind:selection={includeTags} options={test_options} trigger_label="Include Tags..." id="includeTags"/>
                </div>

                <div id="excludes">
                    <Label for="includeTags">Exclude Tags</Label>
                    <SelectionBadge bind:selection={excludeTags} options={test_options} trigger_label="Exclude Tags..." id="excludeTags"/>
                </div>

                <div id="textSearch">
                    <Label for="textSearchTags">Search within results</Label>
                    <Input type="text" id="textSearchTags" class="h-7"/>
                </div>

                <Button variant="default" size="sm" >Sort and Filter</Button>
            </Sidebar.GroupContent>
        </Sidebar.Group>
    </Sidebar.Content>
</Sidebar.Root>