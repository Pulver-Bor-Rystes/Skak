<script lang="ts">
    import "../app.css";
    import { onMount } from "svelte";
    import { socket } from "../networking/ws";
    import Login from "../components/login.svelte"

    onMount(() => {
        $socket.on("test", (data) => {

        })
    });
</script>


<Login />
<slot />