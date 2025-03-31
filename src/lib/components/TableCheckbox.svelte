<script lang="ts">
    import type { ComponentProps } from "svelte";
    import Checkbox from "./ui/checkbox/checkbox.svelte";

    // type Props = {
    //     checked: boolean;
    //     onchange: () => void;
    // };

    // let { checked = false, onchange, ...inputProps }: Props = $props();

    let {
        checked = false,
        onCheckedChange = (v) => (checked = v),
        ...restProps
    }: ComponentProps<typeof Checkbox> = $props();
</script>

<!-- <input type="checkbox" {...inputProps} /> -->
<Checkbox bind:checked={() => checked, onCheckedChange} {...restProps} />
