<script lang="ts">

    const { log } = console;
    import { onMount } from "svelte";
    import { socket } from "../networking/ws";
    
    $socket.on("page", (data) => {
        log("page:", data);
    })
    
</script>


<h1> Skak </h1>