<script lang="ts">
    import Chessboard from "$lib/components/board/chessboard.svelte";
    import LoadingEffect from "$lib/components/loading_effect.svelte";
    import { is_in_game } from "$lib/state";
    import { socket } from "$lib/ws";


    // $socket.send_after("getstate", {});




</script>


<div class="m-5">
    {#if $is_in_game}
        <h2> Game in progress<LoadingEffect/> </h2>
        <Chessboard/>
    {:else}
        <h2> Du er ikke i gang med et spil </h2>

        <a href="/home" class="auth_btn"> Tilbage til hovedsiden </a>
    {/if}
</div>