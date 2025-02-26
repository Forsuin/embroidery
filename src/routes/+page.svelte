<script lang="ts">
  import "../app.css";

  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import * as Resizable from "$lib/components/ui/resizable";
  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import * as Card from "$lib/components/ui/card";

  const tags = Array.from({ length: 50 }).map(
    (_, i, a) => `v1.2.0-beta.${a.length - i}`,
  );
</script>

<div class="min-h-full">
  <Sidebar.Provider>
    <div>
      <Resizable.PaneGroup direction="horizontal" class="min-w-screen">
        <Resizable.Pane defaultSize={20} minSize={20}>
          <div class="flex h-full">
            <AppSidebar />
          </div>
        </Resizable.Pane>
        <Resizable.Handle />
        <Resizable.Pane defaultSize={70}>
          <Sidebar.Inset>
            <ScrollArea class="h-lvh min-w-full p-4 border">
              <div
                class="grid grid-cols-[repeat(auto-fill,minmax(200px,1fr))] gap-4"
              >
                {#each tags as tag}
                  <div>
                    <Card.Root>
                      <Card.Header>
                        <Card.Title
                          >Title: This is a really long title, hopefully this
                          will make the cards bigger so I can see bigger ones</Card.Title
                        >
                        <Card.Description>Description</Card.Description>
                      </Card.Header>
                      <Card.Content>
                        {tag}
                      </Card.Content>
                      <Card.Footer>Footer</Card.Footer>
                    </Card.Root>
                  </div>
                {/each}
              </div>
            </ScrollArea>
          </Sidebar.Inset>
        </Resizable.Pane>
      </Resizable.PaneGroup>
    </div>
  </Sidebar.Provider>
</div>
