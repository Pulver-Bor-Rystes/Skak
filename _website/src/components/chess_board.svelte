<script lang="ts">
    /*

    TODO:
    - Sørg for at brættet selv finder ud af det når det bliver oprættet
      Her snakker vi om når game_id'et skifter.

    - Lav et bræt, hvor man kan rykke rundt på brikkerne, og er renderet ud fra outputtet fra game engine
    
    Sidst:
    Jeg er nået til at tegne brikkerne på brættet.

    */
    
    
    
    
    
    import { Chess } from "../../../CSM/src/chess"
    import { onMount } from "svelte";
    import type { Socket } from "socket.io";
    import { user_data, inv_received, inv_send, friends } from "../stores/state"
    import { get_socket } from "../stores/networking"

    export let game_id: string;
    let is_compact = true;

    
    const chess = new Chess ();
    chess.load_default ();

    let pieces: any[] = [];
    chess.board.get_all_pieces ("*", "*", (square, number, piece, color) => {
        if (color == "EMPTY") return;
        pieces.push ({
            square,
            piece,
            color,
        });
    })
    
    let socket: Socket;
    onMount(() => {
        socket = get_socket ();
    })


    let letters = [ 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h' ];
</script>



<div class="cboard bg-pink-400">
    {#each [1, 0, 1, 0, 1, 0, 1, 0] as y_tile, iy}
        {#each [1, 0, 1, 0, 1, 0, 1, 0] as x_tile, ix}
            {@const square = `${letters[ix]}${ letters[0] == "a" ? 8-iy:iy+1 }`}
            {@const color = x_tile + y_tile == 1}
            
            {#if color}
                <div class="bg-[#272623]"/>
            {:else}
                <div class="bg-[#babbb9]"/>
            {/if}
        {/each}
    {/each}

</div>
<p>{game_id}</p>


<style>
    .cboard {
        display: grid;
        grid-template-columns: repeat(8, 1fr);
        grid-template-rows: repeat(8, 1fr);

        height: 300px;
        width: 300px;
    }
</style>