<script lang="ts">
    import { Trash2 } from "lucide-svelte";
    import Button from "$lib/components/ui/button/button.svelte";
    import Input from "$lib/components/ui/input/input.svelte";

    type Props = {
        value: string;
        oninput: (name: string, row: number) => void;
        index: number;
    };

    let { value, oninput, index }: Props = $props();
</script>

<Input type="text" bind:value onblur={() => oninput(value, index)} />
