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
    
</script>

<main class="m-5 text-white">
    <h1 class="text-4xl"> Skak </h1>

    <p class="mb-5"> Profil: {$user.username}</p>

    <p class="">Online spillere:</p>
    <ul class="">
        {#each players as username}
            <!-- svelte-ignore a11y-click-events-have-key-events -->
            <!-- svelte-ignore a11y-no-static-element-interactions -->
            <li class=""> - { username } (<span on:click={() => $socket.send("invite", { username }) } class="hover:text-blue cursor-pointer">tryk for at udfordre</span>)</li>
        {/each}
    </ul>
</main>
