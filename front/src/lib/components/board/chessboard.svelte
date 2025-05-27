<script lang="ts">
    import { board_id, color, username } from "$lib/state";
    import { socket } from "$lib/ws";
    import { get_moves, get_pieces, get_destinations, get_move_name, play_move, load_fen } from "chess_machine_lib";
    import { onMount } from "svelte";

    let highlight_indexes: number[] = [];
    let drag_index = -1;
    let x = 0;
    let y = 0;

    let pieces: string[] = [];

    onMount(() => {
        console.log("board_id: ", $board_id);

        pieces = get_pieces($board_id);
    });

    $socket.on("state", async ({result, content}) => {
        load_fen($board_id, content);

        pieces = get_pieces($board_id);
    });


    function select_piece(index: number) {
        highlight_indexes = [];
        drag_index = index;
        get_destinations($board_id, $color, index).forEach(v => highlight_indexes.push(v));
    }

    function move_piece_to(index: number) {
        let name = get_move_name($board_id, drag_index, index);

        console.log("making move:", name);
        play_move($board_id, name);
        pieces = get_pieces($board_id);

        $socket.send("play_move", {
            chess_move: name,
        });
    }

    function de_select(index: number) {
        if (drag_index == index) return
        
        if (highlight_indexes.includes(index)) {
            move_piece_to(index);
        }
        
        drag_index = -1;
        highlight_indexes = [];
    }

    function mouse_ev(ev: MouseEvent) {
        x = ev.screenX - 64*0.5;
        y = ev.screenY - 64*2.5;
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div on:mousemove={mouse_ev} class="fixed inset-0 z-50 flex items-center justify-center">
    <div class="w-[512px] h-[512px] grid grid-cols-8 grid-rows-8">
        {#each pieces as piece_name, i}
            <div
                on:mousedown={ _ => de_select(i) }
                class="w-[64px] h-[64px] {i == 0 ? 'rounded-tl-lg' : ''} {i == 7
                    ? 'rounded-tr-lg'
                    : ''} {i == 56 ? 'rounded-bl-lg' : ''} {i == 63
                    ? 'rounded-br-lg'
                    : ''}"
                style="background-color: {(Math.floor(i / 8) + i) % 2 === 0
                    ? '#f0d9b5'
                    : '#b58863'}"
            >

                {#if highlight_indexes.includes(i)}
                    <div style="position: absolute; width: 64px; height: 64px; background-color: #00000074;"></div>
                {/if}
            
                {#if pieces.length != 0 && piece_name != "*"}
                    <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
                     <!-- style="{ drag_index == i ? `position: absolute; top: ${y}px; left: ${x}px`:'' }" -->
                    <img 
                        on:mousedown={_ => select_piece(i)}
                        
                        src="/{piece_name}.png"
                        alt="pawn"
                        class="w-[64px] h-[64px] hover:cursor-grab" 
                        
                    />
                {/if}
            </div>
        {/each}
    </div>
</div>
