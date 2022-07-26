<script defer lang="ts">
    import type { Socket } from "socket.io";
    import { onMount } from "svelte";
    import { get_socket } from "../stores/networking";
    import { user_data, games, my_friends } from "../stores/state";

    import SideMenu from "../components/friends/sidemenu.svelte";
    import Friends from "../components/index/friends.svelte";
    import Auth from "../components/index/auth.svelte";

    
    let socket: Socket;
    onMount(() => socket = get_socket());



</script>


<div class="grid h-screen grid-cols-[1fr_300px]">
    <div class="grid grid-rows-2 items-center">
        <div class="grid">
            <h1 class="text-2xl mt-5 ml-5 justify-self-center"> Hej med dig {$user_data.username} 😆</h1>
            <p class="text-lg mt-5 ml-5 justify-self-center"> ...jeg ved ikke helt hvad der skal være her endnu 🤷‍♀️🤷‍♀️ </p>
        </div>

        <!-- <div class="grid grid-cols-4 mt-5">
            <button class="col-start-2 text-xl bg-gray-700 rounded-xl m-2 max-h-40 p-5 shadow-2xl cursor-pointer transition-all hover:text-pink-200 hover:mt-1"> Åben lobby </button>
            <button class="text-xl bg-gray-700 rounded-xl m-2 max-h-40 p-5 shadow-2xl cursor-pointer transition-all hover:text-pink-200 hover:mt-1"> Ranked </button>
        </div> -->

        {#each $games as game}
            {@const p1 = game.split ("&")[0]}
            {@const p2 = game.split ("&")[1]}
            {#if $my_friends.online.includes (p1) || $my_friends.online.includes (p2)}
                {game}
            {/if}
        {/each}
    </div>

    <div class="">
        <SideMenu/>
    </div>
</div>



<!-- <h1> {JSON.stringify ($user_data, null, 2)} </h1> -->

<Friends/>
<Auth/>