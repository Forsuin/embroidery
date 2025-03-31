<script lang="ts">
    import type { HTMLInputAttributes } from "svelte/elements";
    import Checkbox from "./ui/checkbox/checkbox.svelte";

    type Props = {
        checked: boolean;
        onchange: () => void;
    };

    let { checked = false, onchange, ...inputProps }: Props = $props();
</script>

<!-- <input type="checkbox" {...inputProps} /> -->
<Checkbox bind:checked={() => checked, onchange} {...inputProps} />
