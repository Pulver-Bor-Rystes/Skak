<script lang="ts">
    import { socket } from "$lib/ws";
    import { onMount } from "svelte";
    import { active_players, engines, rating } from "$lib/state";


    onMount(() => {
        $socket.send_after("whats_my_rating", {});
        $socket.send_after("getbots", {});
    });


    $socket.on("rating", ({ result, content }) => {
        if (!result) return;
        $rating = content;
    });


    
    
    $socket.on("active_users", ({ result, content }) => {
        if (!result) return;
        active_players.set(content);
    });

    $socket.on("engines", ({ result, content }) => {
        if (!result) return;
        engines.set(content)
    });
</script>