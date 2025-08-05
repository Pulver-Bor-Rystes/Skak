<script lang="ts">
    import { board_id, board_orientation, color, is_in_game, smart_init, username } from "$lib/state";
    import { socket } from "$lib/ws";
    import { get_destinations, get_move_name, get_pieces, load_fen, new_chessboard, play_move } from "chess_machine_lib";
    import { onMount } from "svelte";

    let { initial_fen = 'rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0' } = $props();


    let highlight_indexes: number[] = $state([]);
    let drag_index = -1;
    let x = 0;
    let y = 0;

    let pieces: string[] = $state([]);


    function update_board() {
        let new_pieces = get_pieces($board_id);

        if ($board_orientation) {
            pieces = new_pieces
            return;
        }

        // sort, så det skal først lige vendes
        let new_arr = [];
        for (const p of pieces)
            new_arr.unshift(p);

        pieces = new_arr;
    }

    

    onMount(async () => {
        console.log("[CHESS] init phase");
        await smart_init();
        console.log("[CHESS] init phase complete!");
        
        $board_id = new_chessboard(initial_fen);
        update_board();
        

        console.log("[CHESS] spawned chessboard with id:", $board_id);

        $socket.send_after("getstate", {})
    });

    $socket.on("game:fen_state", ({ result, content }) => {
        $is_in_game = result;
        if (!result) return;
        
        load_fen($board_id, content);
        update_board();
    })


    $socket.on("game:info", ({ result, content }) => {
        if (!result) return;

        $board_orientation = (content.white == $username);
    })


    function select_piece(index: number) {
        console.log($board_orientation);
        highlight_indexes = [];
        drag_index = index;
        
        let corrected_index = 63-index;
        console.log(index, corrected_index)
        get_destinations($board_id, $color, corrected_index).forEach(v => {
            if (!$board_orientation) {
                v = 63-v;
            }
            
            highlight_indexes.push(v);
        });
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


<!-- svelte-ignore a11y_mouse_events_have_key_events -->
<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore event_directive_deprecated -->
<div on:mouseover={mouse_ev} class="flex items-center justify-center h-full">
    <div class="grid grid-cols-8 grid-rows-8 h-full">
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
                        on:mousedown={() => select_piece(i)}
                        
                        src="/{piece_name}.png"
                        alt="pawn"
                        class="w-[64px] h-[64px] hover:cursor-grab" 
                        
                    />
                {/if}
            </div>
        {/each}
    </div>
</div>
