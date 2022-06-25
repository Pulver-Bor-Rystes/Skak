<script lang="ts">
    import Auth from "../components/index/auth.svelte"
    import Board from "../components/board.svelte";
    import { onMount } from "svelte";
    import { get_socket } from "../stores/networking";
    import { user_data } from "../stores/state";
    
    type PlayerData = {
        username: string
        socket_id: string
    }
    
    let socket: any;
    let lobby: PlayerData[] = [];
    let has_joined = false;
    
    let in_progress = false;

    let current_game_id: string;
    
    onMount(() => {
        socket = get_socket()            
        

        socket.on("join_failure", (reason: string) => console.log(reason))
        
        
        socket.on("update_lobby", (players: PlayerData[]) => {
            // return if has_joined is false
            if (!has_joined) return;
            lobby = players
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



    $: {
        if ($user_data.login_failed && $user_data.logged_in) {
            
        }
    }
    
    
    



    
    function join_lobby() {
        // set has_joined to true
        has_joined = true;
        socket.emit("join")
        // get players in lobby
    }
    
    
    function invite(player_name: string) {
        socket.emit("invite", player_name)
    }
    
    
</script>

<h1 class="m-1 p-1 text-3xl">
    Velkommen { $user_data.username }!
</h1>



{#if lobby.length == 0}
    <button class="m-1 p-2 rounded bg-slate-600" on:click={join_lobby}>
        Join lobby
    </button>
{/if}




{#each lobby as {username, socket_id}}
    <div on:click={() => invite(username)} class="hover:cursor-pointer m-1 p-1 {username == $user_data.username ? 'bg-yellow-900':'bg-gray-600'}">
        {username} - {socket_id}
    </div>

{/each}




{#if in_progress}
    <div class="m-1 p-1">
        <Board game_id={current_game_id}/>
    </div>
{/if}




<Auth/>