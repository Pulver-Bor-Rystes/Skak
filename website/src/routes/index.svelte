<script lang="ts">
    import type { Socket } from "socket.io";
    import Auth from "../components/index/auth.svelte";
    import Friends from "../components/index/friends.svelte";
    import Board from "../components/board.svelte";
    import AddButton from "../components/friends/add_btn.svelte";
    import InviteToGame from "../components/friends/invite_to_game.svelte";
    import { onMount } from "svelte";
    import { get_socket } from "../stores/networking";
    import { user_data, games } from "../stores/state";
    
    type PlayerData = {
        username: string
        socket_id: string
    }
    
    let socket: Socket;
    let lobby: string[] = [];
    $: has_joined = lobby.includes($user_data.username);
    
    let in_progress = false;

    let current_game_id: string;
    
    onMount(() => {
        socket = get_socket()
        

        socket.on("join_failure", (reason: string) => console.log(reason))
        
        
        socket.on("res:lobby/join", (player_names: string[]) => lobby = player_names)
        socket.on("res:lobby/invite", (status: string) => console.log("status:", status))
        socket.on("update_lobby", (players: PlayerData[]) => {
            // return if has_joined is false
            if (!has_joined) return;
            // lobby = players
        })


        socket.on("invite_from", (username: string) => {
            console.log("invite from", username)
        })

        socket.on("invite_accepted", (username: string) => {
            console.log("invite accepted", username)
            in_progress = true;
            has_joined = false;
            lobby = [];
        })


        socket.emit("get_current_game");
        socket.on("current_game", (game_id: string) => {
            console.log("setting current game:", game_id)
            current_game_id = game_id;
            in_progress = true;
        })
        
    })



    


    
    function join_lobby() {
        // socket.emit("join")
        socket.emit("lobby/join");

        // get players in lobby
    }
    
    
    function invite(player_name: string) {
        socket.emit("invite", player_name)
    }
    
    
</script>

<h1 class="m-1 p-1 text-3xl">
    Velkommen { $user_data.username }!
</h1>



{#if !has_joined}
    <button class="m-1 p-2 rounded bg-slate-600" on:click={join_lobby}>
        Join lobby
    </button>
{:else}
    {#each lobby as username}
        <div on:click={() => invite(username)} class="grid grid-cols-2 hover:cursor-pointer m-1 p-1 {username == $user_data.username ? 'bg-yellow-900':'bg-gray-600'}">
            <p class="self-center">{username}</p>
            <div>
                <InviteToGame username={username}/>
                <AddButton username={username}/>
            </div>
        </div>
    {/each}
{/if}







<div class="m-1 p-1">
    <Board game_id={$games[0]}/>
</div>

<a class="p-5 mt-5 text-xl text-yellow-300" href="/home"> Tryk her for at komme til den nye hjemmeside!</a>



<Friends/>
<Auth/>