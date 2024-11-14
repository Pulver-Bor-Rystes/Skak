<script lang="ts">
    import "../app.css";
    import { onMount } from "svelte";
    import { socket } from "../networking/ws";
    import Login from "../components/login.svelte"

    onMount(() => {
        $socket.on("test", (data) => {
            console.log(data)
        })
    });
</script>

<slot />

<!-- <Login /> -->

<style lang="postcss">
    :global(html) {
      background-color: rgb(217, 217, 217);
    }
</style>