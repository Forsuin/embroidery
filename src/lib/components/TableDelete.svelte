<script lang="ts">
    import { Trash2 } from "lucide-svelte";
    import Button from "./ui/button/button.svelte";

    type Props = {
        onclick: () => void;
    };

    let { onclick }: Props = $props();
</script>

<div class="text-center">
    <Button {onclick} variant="outline" size="icon" class="space-evenly">
        <Trash2 />
    </Button>
</div>
