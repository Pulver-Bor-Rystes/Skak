<script>
    import Button from '$lib/components/button.svelte';
    import { socket } from '$lib/ws';
    import { onMount } from 'svelte';

    onMount(() => {
        $socket.send("test", "hej");
    });
</script>

<div class="m-5 shadow border-rounded bg-[#ffffff2d] w-128 h-72 rounded-xl grid" style="grid-template-columns: 30% auto;">
    <div class="m-1">
        <Button> Play </Button>

        <h3 class="m-1"> Bots </h3>
        <Button> Stockfish </Button>
    </div>
    <div>
        
    </div>
</div>