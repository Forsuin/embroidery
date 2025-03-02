<script lang="ts">
  import "../app.css";

  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import * as Resizable from "$lib/components/ui/resizable";
  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import * as Card from "$lib/components/ui/card";

  const versions = Array.from({ length: 50 }).map(
    (_, i, a) => `v1.2.0-beta.${a.length - i}`,
  );

  // all tags from db
  let tags: string[] = $state([
    "First",
    "Second",
    "Third",
    "Fourth",
    "Fifth",
    "Sixth",
    "Seventh",
    "Eight",
    "Ninth",
    "Tenth",
  ]);

  // tags to search for in db
  let search_query: string[] = $state([]);

  $inspect(search_query);

  function selectOptions(): string[] {
    let temp = tags.slice();
    const shuffled = temp.sort(() => 0.5 - Math.random());

    let selected = shuffled.slice(0, 3);

    return selected;
  }

  // function call_rust() {
  //   invoke("format_search_query", { args: search_query }).then((message) =>
  //     console.log(message),
  //   );
  // }

  // $effect(() => {
  //   call_rust();
  // });

  let is_search_updating = $state(false);
  let timeoutID: number | null = null;

  async function delayedSearch() {
    is_search_updating = true;
    timeoutID = setTimeout(() => {
      console.log("Delayed function called after 1 second");
      is_search_updating = false;
      timeoutID = null;
    }, 1000);
  }

  function resetTimer() {
    if (timeoutID) {
      clearTimeout(timeoutID);
      timeoutID = null;
      is_search_updating = false;
    }
  }

  $effect(() => {
    resetTimer();
    if (search_query) {
      delayedSearch();
    }
  });
</script>

<div class="min-h-full">
  <Sidebar.Provider>
    <div>
      <Resizable.PaneGroup direction="horizontal" class="min-w-screen">
        <Resizable.Pane defaultSize={20} minSize={20}>
          <div class="flex min-h-screen">
            <AppSidebar bind:tag_options={tags} bind:search_query />
          </div>
        </Resizable.Pane>
        <Resizable.Handle />
        <Resizable.Pane defaultSize={70}>
          <Sidebar.Inset>
            <ScrollArea class="h-lvh min-w-full p-4">
              <div
                class="grid grid-cols-[repeat(auto-fill,minmax(200px,1fr))] gap-4"
              >
                {#each versions as tag}
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
                      <Card.Footer>
                        <div class="flex flex-wrap gap-1">
                          {#each selectOptions() as chip}
                            <div class="chip preset-filled-primary-500">
                              {chip}
                            </div>
                          {/each}
                        </div>
                      </Card.Footer>
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
