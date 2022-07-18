<script lang="ts">
    import { onMount } from "svelte";
    import type { Socket } from "socket.io";
    import { user_data, inv_received, inv_send, friends } from "../../stores/state"
    import { get_socket } from "../../stores/networking"
    
    export let username: string;
    let socket: Socket;
    onMount(() => {
        socket = get_socket()

        if ($friends.length == 0) {
            socket.emit("friends/fetch")
        }
    })



    function send_f_request(username: string) {
        socket.emit("friends/request", username);
    }
</script>



{#if username != $user_data.username}
    <button on:click={ () => send_f_request(username) } class="p-1 pr-2 pl-2 mr-1 ml-1 bg-slate-700 hover:bg-slate-800 float-right rounded">
    {#if $friends.includes(username)}
        Venner
    {:else if $inv_received.includes(username)}
        Accepter venneanmodning
    {:else if $inv_send.includes(username)}
        Venneanmodning er sendt
    {:else}
        Tilføj ven
    {/if}
    </button>
{/if}