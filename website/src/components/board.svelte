<script lang="ts">
    import { Chess, type Square } from 'chess.js'
    import { get_socket } from "../stores/networking";
    import { onMount } from "svelte";
	import { user_data } from '../stores/state';


    export let game_id: string;
    let socket: any;
    
    
    let game: any;
    let board = new Chess()

    let current_square = ""
    $: current_moves = board.moves({ square: current_square })

    onMount(() => {
        // get socket
        socket = get_socket();

        // @ts-ignore
        window["board"] = board; window["update"] = update_board_state; window["game"] = game;
        
        socket
            .on("game_created", (new_game_state: any) => {
				game = new_game_state;
                board.load_pgn(new_game_state.pgn);
				update_board_state();
				if (game.black == $user_data.username) {
					switch_direction()
				}
            })
            .on("move_made", (move: string) => {
                board.move(move);
				update_board_state();
            })
            .on("move_invalid", (pgn: string) => {
				board.load_pgn(pgn)
				update_board_state();
            });



		// henter opdatering automatisk		
		socket.emit("get_game_state", game_id);
    })


    

    function update_board_state() {
        switch_direction()
        switch_direction()
    }







	let letters = [ 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h' ]

	function switch_direction() {
		letters = letters.reverse()
	}



	function get_src(square: string) {
		if (!board) return ""

		let piece = board.get(square as Square)
		if (piece) {
			return `${link}${piece.color}${piece.type}.png`
		}
		return ""
	}


	async function move_piece(square: string) {
		let new_move
		for (const move of current_moves)
			if (move.includes(square))
				new_move = move

		if (new_move) {
			// if it is my turn to play
			if ((board.turn() == "w" && game?.white == $user_data.username) || board.turn() == "b" && game?.black == $user_data.username) {
				board.move(new_move) 
				// send move to server
				socket.emit("move", {
					"new_move": new_move,
					"pgn": board.pgn()
				})
				current_square = ""
			}
		}
		else {
			if (board.get(square as Square)) {
				current_square = square
			}
			else {
				current_square = ""
			}
		}
	}



	/** VSCode brokker sig gevaldigt når man prøver at bruge en string i stedet for typen Square
	 * og da man ikke kan angive typer i svelte's konstante værdier er det her desværre løsningen...
	*/
	function wrapper_board_get(square: string) {
		return board.get(square as Square)
	}


	let link = "https://images.chesscomfiles.com/chess-themes/pieces/neo_wood/150/"
</script>




<div class="wrapper" style="--size: 300px">
    <div class="board">
        {#each [1, 0, 1, 0, 1, 0, 1, 0] as x_tile, iy}
            {#each [1, 0, 1, 0, 1, 0, 1, 0] as y_tile, ix}
                {@const square = `${letters[ix]}${ letters[0] == "a" ? 8-iy:iy+1 }`}
                
                <p hidden>{square}</p>
                <div on:click={() => move_piece(square)} id="{square}" class={ (x_tile + y_tile ) == 1 ? "black":"white" }>
                    {#if wrapper_board_get(square)}
                        <img 
                            class="obj"
                            src={get_src( square )}
                            alt=""
                        >
                    {/if}
                    
                    {#each current_moves as move}
                        {#if move.includes( square )}
                            <div class="highlight {move.includes("+") || move.includes("#") ? "h-red":"h-green"}"></div>
                        {/if}
                    {/each}
                </div>

            {/each}
        {/each}
    </div>
</div>

<!-- Controls -->
<p>{game?.white} VS. {game?.black}</p>
<button on:click={switch_direction}>x</button>







<style lang="css">

	.highlight {
		width: 100%;
		height: 100%;
	}
	
	.h-green {
		background-color: rgba(0, 255, 106, 0.356);
	}
	
	.h-red {
		background-color: rgba(255, 51, 0, 0.356);
	}


	.black {
		background-color: black;
	}

	.white {
		background-color: white;
	}

	.obj {
		position: absolute;
		width: calc(var(--size) / 8);
        height: calc(var(--size) / 8);
	}

	.wrapper {
		width: 300px;
		height: 300px;
	}

	.board {
		height: 100%;
		width: 100%;

		display: grid;
		grid-template-columns: repeat(8, 1fr);
		grid-template-rows: repeat(8, 1fr);

		border: 1px solid;
	}
</style>