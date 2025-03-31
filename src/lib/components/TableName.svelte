<script lang="ts">
    import { Trash2 } from "lucide-svelte";
    import Button from "./ui/button/button.svelte";
    import Input from "./ui/input/input.svelte";

    type Props = {
        value: string;
        oninput: (name: string, row: number) => void;
        index: number;
    };

    let { value, oninput, index }: Props = $props();
</script>

<Input type="text" bind:value onblur={() => oninput(value, index)} />
