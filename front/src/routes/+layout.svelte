<script lang="ts">
    import Autologon from "$lib/components/autologon.svelte";
    import LoadData from "$lib/components/load_data.svelte";
    import "../app.css";
</script>

<slot />

<Autologon />
<LoadData/>

<style lang="postcss">
    :global(html) {
      background-color: var(--background-50);
    }
</style>