<script lang="ts">
    import Checkbox from "./ui/checkbox/checkbox.svelte";

    type Props = {
        checked: boolean;
        onchange: () => void;
    };

    let { checked = false, onchange, ...inputProps }: Props = $props();
</script>

<!-- <input type="checkbox" {...inputProps} /> -->
<Checkbox bind:checked={() => checked, onchange} {...inputProps} />
