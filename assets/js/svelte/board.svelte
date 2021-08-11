<script type="ts">
	import Piece_elem from "./piece.svelte"

	const piece_settings = {
		link: 'https://images.chesscomfiles.com/chess-themes/pieces',
		theme: 'neo_wood',
		size: 150,
	}

	let colors: boolean[] = []
	let sw: boolean = false

	for (let x = 0; x < 8; x++) {
		for (let y = 0; y < 8; ++y)
			colors.push(sw = !sw)

		sw = !sw
	}

	/*
	
	################################
	#           Eksempel           #
	################################

	11100101111
	111 001 011 1 1 = hvid bonde som står på H2, som har rykket sig

	000 - b
	001 - k
	010 - n
	011 - p
	100 - q
	101 - r
	110
	111
	
	#################################
	#     Position og betydning     #
	#################################

    0-2 = x
	3-5 = y
	6-8 = brik type
	  9 = sort/hvid
	 10 = om brikken har rykket
	*/

	const char_pieces: char[] = [ 'b', 'k', 'n', 'p', 'q', 'r' ]

	type Piece = {
		id: number;
		x: number;
		y: number;
		piece: char;
		color: boolean;
		moved: boolean;
	}


	function convert_bitpiece_piece(bit_piece: number, reverse: boolean): Piece {
		const bit_piece_str: string = (''+bit_piece)

		let x: number = parseInt( bit_piece_str.slice(0, 3), 2 )
		let y: number = parseInt( bit_piece_str.slice(3, 6), 2 )

		let piece: Piece = {
			id: ((reverse ? 8-x:x)+1)*((reverse ? 8-y:y)+1),
			x: reverse ? 8-x:x,
			y: reverse ? 8-y:y,
			piece: char_pieces[ parseInt( bit_piece_str.slice(6, 9), 2 ) ],
			color: Boolean( bit_piece_str[9] ),
			moved: Boolean( bit_piece_str[10] )
		}

		return piece
	}

	let pieces: Piece[] = []
	
	let wp: Piece = convert_bitpiece_piece( 11100100011, 0 )
	console.log(wp)

	pieces[ (wp.x+1) * (wp.y+1) ] = wp


	// Returnerer et link til et billede af en skak brik.
	function make_link(piece: Piece): string {
		if (!piece) return ''

		const color: string = piece.color ? 'w':'b'
		const link: string = `${piece_settings.link}/${piece_settings.theme}/${piece_settings.size}/${color}${piece.piece}.png`

		return `<div class="piece" style="background-image: url("${link}"); background-size: contain;"/>`
	}


</script>




<div class="board">
	{#each colors as color, id}
		<div class="tile { color ? 'white':'black' }">
			<Piece_elem piece={pieces[id]}/>
		</div>
	{/each}
</div>




<style type="scss">
	.board {
		display: grid;
		grid-template-columns: repeat(8, 1fr);
		grid-template-rows: repeat(8, 1fr);

		width: 600px;
		height: 600px;

		background: grey;


		.tile {
			width: 100%;
			background-size: contain;
		}
		
		.white {
			background: #CEAF87;
		}

		.black {
			background: #7C6A51;
		}


		.piece {
			height: 100%;
		}
	}
</style>
