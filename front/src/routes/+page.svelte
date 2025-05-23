<script lang="ts">
    import Button from '$lib/components/button.svelte';
    import Login from '$lib/components/debug/login.svelte';

    // import { greet } from 'chess_machine_lib';
    import { greet_test } from '$lib';
    
    import { active_players, engines, username } from '$lib/state';
    import { socket } from '$lib/ws';

    function new_game(engine: string) {
        $socket.send("newgame", {
            username: engine,
            timeformat: "lalal",
        })
    }

    function click() {
        $socket.send("getstate", {})
        greet_test();
    }
</script>

<div class="m-5 shadow border-rounded bg-[#ffffff2d] w-128 rounded-xl grid" style="grid-template-columns: 30% auto;">
    <div class="m-1">
        <Login/>

        <h3 class="m-1"> Online Players </h3>
        <ul>
            {#each $active_players as player_name}
                {#if player_name != $username}
                    <li class="ml-3"> - {player_name}</li>
                {/if}
            {/each}
        </ul>

        <br>

        <h3 class="m-1"> Bots </h3>
        {#each $engines as engine}
            <Button fn={ () => { new_game(engine) } }> {engine} </Button>
        {/each}

        <br><br>
        <h3 class="m-1"> Dev Tools </h3>

        <Button fn={click}> Get state </Button>
    </div>


    <div>
        
    </div>


</div>