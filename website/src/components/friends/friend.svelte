<script lang="ts">
    import { get_socket } from "../../stores/networking";
    import { onMount } from "svelte";
    import { G_inv_send, G_inv_recieved, games } from "../../stores/state";
    import type { Socket } from "socket.io";
    export let name: string;
    export let online: boolean;

    let socket: Socket;
    onMount (() => {
        socket = get_socket ();

        socket.on ("notif:games/state", (data) => {
            console.log ("notif:")
            console.log (data)
        })
    })

    let in_game = false;

    $: {
        for (let game of $games) {
            if (game.includes (name)) {
                in_game = true;
            }
        }
    }

    // $: state = $G_inv_send.includes (name) ? 'pending': $G_inv_recieved.includes (name) ? 'accept':in_game ? 'in game':'invite';

    $: state = in_game ? 'in game': $G_inv_send.includes (name) ? 'pending': $G_inv_recieved.includes (name) ? 'accept':'invite';

</script>


<div class="rounded p-1 pl-2 mb-2 ml-2 mr-2 {online ? 'bg-gray-700 cursor-pointer':'bg-gray-800'}">
    <p on:click={() => { socket.emit ("games/invite", name) }} class="text-lg  {online ? '':'text-gray-500'}"> {name} <span class="text-lg text-slate-500 float-right"> {state} </span> </p>
</div>
