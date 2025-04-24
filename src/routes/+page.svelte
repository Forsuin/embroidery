<script lang="ts">
  import "../app.css";

  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import AppSidebar from "$lib/components/AppSidebar.svelte";
  import * as Resizable from "../lib/components/ui/resizable";
  import { ScrollArea } from "../lib/components/ui/scroll-area";

  import { invoke } from "@tauri-apps/api/core";
  import ImportDialogue from "$lib/components/ImportDialogue.svelte";
  import { emit, listen } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import "@tauri-apps/api/menu";
  import { Menu } from "@tauri-apps/api/menu";
  import { platform } from "@tauri-apps/plugin-os";
  import type { DragDropEvent } from "@tauri-apps/api/webview";
  import AdvancedSearch from "$lib/components/AdvancedSearch.svelte";
  import {fileExplorerPrompt, type SearchQuery} from "$lib/utils";
  import PatternCard from "$lib/components/PatternCard.svelte";
  import type {Pattern, Tag, PatternTags} from "$lib/types";


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
    let file_explorer_prompt = fileExplorerPrompt();

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
        // {
        //   id: "cut",
        //   text: "Cut",
        //   accelerator: accelerators.cut.accelerator,
        //   action: accelerators.cut.action,
        // },
        // {
        //   id: "copy",
        //   text: "Copy",
        //   accelerator: accelerators.copy.accelerator,
        //   action: accelerators.copy.action,
        // },
        // {
        //   id: "paste",
        //   text: "Paste",
        //   accelerator: accelerators.paste.accelerator,
        //   action: accelerators.paste.action,
        // },
        // {
        //   item: "Separator",
        // },
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

    await menu.setAsAppMenu();

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

    // get patterns
    search_patterns();
  });

  // tags to search for in db
  let search_query: SearchQuery = $state({
    include_tags: [],
    exclude_tags: [],
    custom_query: "",
  });

  let patterns: PatternTags[] = $state([]);

  function toggle(option: string): void {
    let index = search_query.include_tags.indexOf(option);

    if (index > -1) {
      search_query.include_tags = [
        ...search_query.include_tags.slice(0, index),
        ...search_query.include_tags.slice(index + 1),
      ];
    } else {
      search_query.include_tags = [...search_query.include_tags, option];
    }
  }

  listen("refresh-patterns", () => {
    // update patterns with any new ones that match existing search
    search_patterns();
  });

  function search_patterns(): void {
    // clear patterns before searching to avoid duplicates
    patterns = [];

    invoke<Pattern[]>("search_patterns", { search_query: search_query }).then(
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
        });
      },
    );
  }



  listen<PatternTags>("openPatternEdit", (event) => {
    console.log("edit pattern: ", event);
  });

  // let timeoutID: number | null = null;
  //
  // async function delayedSearch() {
  //   timeoutID = setTimeout(() => {
  //     search_patterns();
  //     timeoutID = null;
  //   }, 100);
  // }
  //
  // function resetTimer() {
  //   if (timeoutID) {
  //     clearTimeout(timeoutID);
  //     timeoutID = null;
  //
  //   }
  // }
  //
  // $effect(() => {
  //   resetTimer();
  //   if (search_query) {
  //     delayedSearch();
  //   }
  // });
</script>

<ImportDialogue bind:isOpen={isImportDialogueOpen} {tags} />
<div class="min-h-full select-none">
  <Sidebar.Provider>
    <div>
      <Resizable.PaneGroup direction="horizontal" class="min-w-screen">
        <Resizable.Pane defaultSize={20} minSize={20}>
          <div class="flex min-h-screen">
            <AdvancedSearch tag_options={tags} bind:search_query search_function={search_patterns} />
          </div>
        </Resizable.Pane>
        <Resizable.Handle />
        <Resizable.Pane defaultSize={70}>
          <Sidebar.Inset>
            {#if (patterns.length !== 0) }
              <ScrollArea class="h-lvh min-w-full ">
                <div
                        class="grid grid-cols-[repeat(auto-fill,minmax(200px,1fr))] gap-4 m-2"
                >
                  {#each patterns as pattern_tag}
                    <PatternCard {pattern_tag}/>
                  {/each}
                </div>
              </ScrollArea>
            {:else}
              <div class="grid gap-4 m-2">
                No results found
              </div>
            {/if}
          </Sidebar.Inset>
        </Resizable.Pane>
      </Resizable.PaneGroup>
    </div>
  </Sidebar.Provider>
</div>
