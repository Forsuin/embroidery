<script lang="ts">
  import "../app.css";

  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import * as Resizable from "$lib/components/ui/resizable";
  import { ScrollArea } from "$lib/components/ui/scroll-area";
  import * as Card from "$lib/components/ui/card";
  import { invoke } from "@tauri-apps/api/core";
  import ImportDialogue from "$lib/components/ImportDialogue.svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import "@tauri-apps/api/menu";
  import { Menu } from "@tauri-apps/api/menu";
  import { platform } from "@tauri-apps/plugin-os";
  import type { DragDropEvent } from "@tauri-apps/api/webview";

  async function onKeyDown(event: KeyboardEvent) {
    let shortcut = "";

    if (event.metaKey || event.ctrlKey) {
      shortcut += "CmdOrCtrl+";
    }

    if (event.shiftKey) {
      shortcut += "Shift+";
    }

    if (event.altKey) {
      shortcut += "Alt+";
    }

    shortcut += event.key.toUpperCase();

    // console.log(shortcut);

    let action = shortcutToActionName[shortcut];
    if (action) {
      action.action();
    }

    // console.log(`shortcut: ${shortcut}`);
  }

  const accelerators = {
    import: {
      accelerator: "CmdOrCtrl+I",
      action: () => {
        console.log("Import shortcut");
        showImportDialogue();
      },
    },
    reveal: {
      accelerator: "CmdOrCtrl+Shift+O",
      action: () => {
        console.log("Reveal shortcut");
      },
    },
    settings: {
      accelerator: "CmdOrCtrl+,",
      action: () => {
        console.log("Settings shortcut");
      },
    },
    quit: {
      accelerator: "CmdOrCtrl+Q",
      action: () => {
        console.log("Quit shortcut");
      },
    },
    undo: {
      accelerator: "CmdOrCtrl+Z",
      action: () => {
        console.log("Undo shortcut");
      },
    },
    redo: {
      accelerator: "CmdOrCtrl+Shift+Z",
      action: () => {
        console.log("Redo shortcut");
      },
    },
    cut: {
      accelerator: "CmdOrCtrl+X",
      action: () => {
        console.log("Cut shortcut");
      },
    },
    copy: {
      accelerator: "CmdOrCtrl+C",
      action: () => {
        console.log("Copy shortcut");
      },
    },
    paste: {
      accelerator: "CmdOrCtrl+V",
      action: () => {
        console.log("Paste shortcut");
      },
    },
    selectAll: {
      accelerator: "CmdOrCtrl+A",
      action: () => {
        console.log("Select all shortcut");
      },
    },
  };

  const shortcutToActionName = Object.fromEntries(
    Object.entries(accelerators).map(([name, { accelerator, action }]) => [
      accelerator,
      {
        name,
        action,
      },
    ]),
  );

  // console.log(shortcutToActionName);

  onMount(async () => {
    let file_explorer_prompt;

    switch (platform()) {
      case "windows":
        file_explorer_prompt = "Reveal in File Explorer";
        break;
      case "macos":
        file_explorer_prompt = "Show in Finder";
        break;
      default:
        file_explorer_prompt = "Show in File Manager";
    }

    if (platform() == "windows") {
      // alert("added event listener");
      window.addEventListener("keydown", onKeyDown);
    }

    const file_submenu = {
      items: [
        {
          id: "import",
          text: "Import",
          action: accelerators.import.action,
          accelerator: accelerators.import.accelerator,
        },
        {
          id: "reveal",
          text: file_explorer_prompt,
          action: accelerators.reveal.action,
          accelerator: accelerators.reveal.accelerator,
        },
        {
          item: "Separator",
        },
        {
          id: "settings",
          text: "Settings",
          action: accelerators.settings.action,
          accelerator: accelerators.settings.accelerator,
        },
        {
          item: "Separator",
        },
        {
          id: "quit",
          text: "Quit",
          action: accelerators.quit.action,
          accelerator: accelerators.quit.accelerator,
        },
      ],
      text: "File",
    };

    const edit_submenu = {
      items: [
        {
          id: "undo",
          text: "Undo",
          accelerator: accelerators.undo.accelerator,
          action: accelerators.undo.action,
        },
        {
          id: "redo",
          text: "Redo",
          accelerator: accelerators.redo.accelerator,
          action: accelerators.redo.action,
        },
        {
          item: "Separator",
        },
        {
          id: "cut",
          text: "Cut",
          accelerator: accelerators.cut.accelerator,
          action: accelerators.cut.action,
        },
        {
          id: "copy",
          text: "Copy",
          accelerator: accelerators.copy.accelerator,
          action: accelerators.copy.action,
        },
        {
          id: "paste",
          text: "Paste",
          accelerator: accelerators.paste.accelerator,
          action: accelerators.paste.action,
        },
        {
          item: "Separator",
        },
        {
          id: "selectAll",
          text: "Select All",
          accelerator: accelerators.selectAll.accelerator,
          action: accelerators.selectAll.action,
        },
      ],
      text: "Edit",
    };

    const menu = await Menu.new({
      items: [file_submenu, edit_submenu],
    });

    menu.setAsAppMenu();

    await listen<DragDropEvent>("tauri://drag-drop", (event) => {
      // console.log(event);

      // paths exists, but compiler is dumb
      // @ts-ignore
      const files = event.payload.paths;

      type DragDropPayload = {
        paths: string[];
      };

      let payload = {
        paths: files,
      };

      invoke("drag_drop_file_dialog", { payload: payload });

      showImportDialogue();
    });
  });

  let isImportDialogueOpen = $state(false);

  function showImportDialogue() {
    // console.log("show import");
    isImportDialogueOpen = true;
  }

  const versions = Array.from({ length: 50 }).map(
    (_, i, a) => `v1.2.0-beta.${a.length - i}`,
  );

  // all tags from db
  let tags: string[] = $state([]);

  onMount(async () => {
    // get tags at startup
    await invoke<Array<{ name: string }>>("get_tags")
      .then((db_tags) => {
        tags = db_tags.map((tag) => tag.name);
      })
      .catch((error) => {
        console.log("Unable to get tags from database: ", error);
      });
  });

  // tags to search for in db
  let search_query: string[] = $state([]);

  type Pattern = {
    id: number;
    name: string;
    pattern_num?: number;
    thread_count?: number;
  };

  type Tag = {
    id?: number;
    name: string;
  };

  type PatternTags = {
    pattern: Pattern;
    tags: Tag[];
  };

  let patterns: PatternTags[] = $state([]);

  function toggle(option: string): void {
    let index = search_query.indexOf(option);
    if (index > -1) {
      search_query = [
        ...search_query.slice(0, index),
        ...search_query.slice(index + 1),
      ];
    } else {
      search_query = [...search_query, option];
    }
  }

  function search_patterns(): void {
    invoke<Pattern[]>("get_patterns", { query_tags: search_query }).then(
      (result_patterns: Pattern[]) => {
        result_patterns.forEach((pattern) => {
          invoke<Tag[]>("get_pattern_tags", {
            pattern_id: pattern.id,
          }).then((result_tags: Tag[]) => {
            let new_pattern = {
              pattern: pattern,
              tags: result_tags,
            };

            patterns = [...patterns, new_pattern];
          });

          // patterns.push({
          //   pattern: pattern,
          //   tags: pattern_tags,
          // });
        });
      },
    );
  }

  let is_search_updating = $state(false);
  let timeoutID: number | null = null;

  async function delayedSearch() {
    is_search_updating = true;
    timeoutID = setTimeout(() => {
      // console.log("Delayed function called after .7 second");
      search_patterns();
      is_search_updating = false;
      timeoutID = null;
    }, 350);
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

<ImportDialogue bind:isOpen={isImportDialogueOpen} {tags} />
<div class="min-h-full select-none">
  <Sidebar.Provider>
    <div>
      <Resizable.PaneGroup direction="horizontal" class="min-w-screen">
        <Resizable.Pane defaultSize={20} minSize={20}>
          <div class="flex min-h-screen">
            <AppSidebar bind:tag_options={tags} bind:search_query {toggle} />
          </div>
        </Resizable.Pane>
        <Resizable.Handle />
        <Resizable.Pane defaultSize={70}>
          <Sidebar.Inset>
            <ScrollArea class="h-lvh min-w-full ">
              <div
                class="grid grid-cols-[repeat(auto-fill,minmax(200px,1fr))] gap-4 m-2"
              >
                {#each patterns as pattern_tag}
                  <div>
                    <Card.Root>
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
                            <button
                              type="button"
                              class="chip {search_query.includes(tag.name)
                                ? 'preset-filled'
                                : 'preset-tonal'}"
                              onclick={() => toggle(tag.name)}
                              >{tag.name}
                            </button>
                          {/each}
                        </div>
                      </Card.Footer>
                    </Card.Root>
                  </div>
                {/each}

                <!-- {#each versions as tag}
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
                            <button
                              type="button"
                              class="chip {search_query.includes(chip)
                                ? 'preset-filled'
                                : 'preset-tonal'}"
                              onclick={() => toggle(chip)}
                              >{chip}
                            </button>
                          {/each}
                        </div>
                      </Card.Footer>
                    </Card.Root>
                  </div>
                {/each} -->
              </div>
            </ScrollArea>
          </Sidebar.Inset>
        </Resizable.Pane>
      </Resizable.PaneGroup>
    </div>
  </Sidebar.Provider>
</div>
