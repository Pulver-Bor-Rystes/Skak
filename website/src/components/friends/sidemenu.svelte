<script lang="ts">
    import type { Socket } from "socket.io";
    import { onMount } from "svelte";
    import { get_socket } from "../../stores/networking";
    import { user_data, games, friends, my_friends, type FriendsInfo } from "../../stores/state";
    import Friend from "./friend.svelte";

    let online_count = 0;

    
    let socket: Socket;
    onMount(() => {
        socket = get_socket()
    
    
        socket.on ("res:friends/get", (data: FriendsInfo) => {
            $my_friends = data;
            console.log (data)
        })



        socket.on ("res:au/test", data => console.log(data))
    
    });

</script>


<div class="grid grid-rows-[75px_1fr_75px] bg-zinc-900 h-screen">
    <span class="flex justify-center">
        <h1 class="text-2xl text-center p-5">Venner</h1>
        <button class="h-7 self-center pl-2 pr-2 bg-slate-400 rounded text-black"> tilføj </button>
    </span>

    {#if $friends.length == 0}
        <div class="h-fit self-center justify-center grid">
            <h1 class="text-center m-5 text-">Hov! Det ser ud til at du ikke har nogen venner endnu 🥺</h1>
            <button class="bg-slate-700 shadow-xl rounded-xl m-2 p-3 pr-5 pl-5 text-center text-xl transition-all hover:text-pink-200 hover:text-2xl">Tilføj ven!</button>
        </div>
    {:else}
        {#if $friends[0] == "?"}
            <div class="self-center">
                <h1 class="text-center text-3xl">⌛</h1>
                <h1 class="text-center ml-5 mr-5">Vent lige to sekunder, mens vi finder dine venner...</h1>
            </div>
        {:else}
            <div class="mr-5 ml-5 overflow-auto">
                {#each $my_friends.online as friend_name}
                    <Friend name={friend_name} online={true}/>
                {/each}
                {#each $my_friends.offline as friend_name}
                    <Friend name={friend_name} online={false}/>
                {/each}
            </div>
            <!-- <button class="bg-slate-700 shadow-xl rounded-xl m-2 p-3 pr-5 pl-5 text-center text-xl transition-all hover:text-pink-200 hover:text-2xl">Tilføj ven!</button> -->
        {/if}
    {/if}
</div>



<style>
    /* width */
    ::-webkit-scrollbar {
    width: 10px;
    }

    /* Track */
    ::-webkit-scrollbar-track {
    background: #f1f1f1;
    }

    /* Handle */
    ::-webkit-scrollbar-thumb {
    background: #888;
    }

    /* Handle on hover */
    ::-webkit-scrollbar-thumb:hover {
    background: #555;
    }
</style>