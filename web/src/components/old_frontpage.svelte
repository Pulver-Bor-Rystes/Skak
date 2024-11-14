<script lang="ts">

    const { log } = console;
    import { socket } from "../networking/ws";
    import { user } from "../stores/user"

    let players: string[] = []
    
    $socket.on("page", (data) => {
        log("page:", data);
    })

    $socket.on("active_players", (data) => {
        console.log(data)
        players = data.content.filter((username: string) => {
            if (username == $user.username) {
                return false;
            }
            return true;
        });
    })

    $socket.on("state", ({ content }) => {
        console.log("FEN:", content)
    })
    
</script>




<main class="m-5 text-black">
    <!-- svelte-ignore a11y-click-events-have-key-events -->
    <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
    <h1 class="text-4xl" on:click={() => $socket.send("getstate", {}) }> Skak </h1>

    <p class="mb-5"> Profil: {$user.username}</p>

    <p class="">Online spillere:</p>
    <ul class="">
        {#each players as username}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <li class=""> - { username } (<span on:click={() => $socket.send("newgame", { username, timeformat: "M5s3" }) } class="hover:text-blue cursor-pointer">tryk for at udfordre</span>)</li>
        {/each}
    </ul>

    <p>Engines</p>
    <button on:click={() => $socket.send("newgame", { username: "juules", timeformat: "M5s3" })}>Juules</button>
    <button on:click={() => $socket.send("newgame", { username: "stockfish", timeformat: "M5s3" })}>Stockfish</button>
</main>
