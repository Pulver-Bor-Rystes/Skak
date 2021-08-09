<script type="ts">
    import { fade } from 'svelte/transition';

    const base_url = "https://images.chesscomfiles.com/chess-themes/pieces"

	export let theme: string = "neo_wood"
    export let size: number = 150
    export let piece: Piece

    

</script>



{#if piece}
    <div transition:fade="{{ delay: 200, duration: 1000 }}" class="piece" style="background-image: url('{base_url}/{theme}/{size}/{piece.color ? 'w':'b'}{piece.piece}.png'); background-size: contain;"/>
{/if}




<style type="scss">
	.piece {
        height: 100%;
    }
</style>