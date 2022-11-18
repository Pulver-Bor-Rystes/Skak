<script lang="ts">
    import { onMount } from "svelte";
    import type { Socket } from "socket.io";
    import { user_data, G_inv_recieved, G_inv_send, games } from "../../stores/state"
    import { get_socket } from "../../stores/networking"
    
    export let username: string;
    let socket: Socket;
    onMount(() => {
        socket = get_socket()


        socket.on("notif:games/state", ([gs, rec, send]) => {
            $games = gs;
            $G_inv_recieved = rec;
            $G_inv_send = send;

            console.log(gs)
        })

        socket.on("notif:game/created", (game) => {
            console.log(game)

            
            $games[0] = game;
        })
    })


    


    function send_game_invite(username: string) {
        socket.emit("games/invite", username);
    }
</script>


{#if username != $user_data.username}
<button on:click={() => send_game_invite(username)} class="p-1 pr-2 pl-2 mr-1 ml-1 bg-slate-700 hover:bg-slate-800 float-right rounded">
    {#if $games.includes(username)}
        Joining...
    {:else if $G_inv_recieved.includes(username)}
        Accepter spil anmodning
    {:else if $G_inv_send.includes(username)}
        Inviteret...
    {:else}
        Inviter til spil
    {/if}
</button>
{/if}