<script>
    import { pending_invite } from "$lib/state";

</script>


{#if $pending_invite}
    <div class="media-object grid absolute def-border pl-5 pr-5 h-16 bottom-5 right-5">
        <h4 class="self-center">Udfordring sendt til <span class="font-bold">stockfish</span></h4>
    </div>
{/if}


<style>
    .media-object {
        --border-width: 1px;
        border: var(--border-width) solid transparent;
    }

    .media-object::before {
        content: " ";
        position: absolute;
        inset: calc(var(--border-width) * 0);
        z-index: 10;
        border: inherit;
        border-radius: inherit;
        background-image: conic-gradient(from var(--angle), #E4E4E4 80%, #E0D1FF 88%, #59b3d9 92%, #3487aa 100%);
        background-origin: border-box;
        -webkit-mask:
            linear-gradient(#E4E4E4, #E4E4E4) content-box,
            linear-gradient(#E4E4E4, #E4E4E4);  
        mask: linear-gradient(#E4E4E4, #E4E4E4),
                linear-gradient(#E4E4E4, #E4E4E4);
        -webkit-mask-clip: content-box, border-box;  
        mask-clip: content-box, border-box;
        -webkit-mask-composite: xor;  
        mask-composite: exclude;
        animation: spin 3s linear infinite;
    }



    @property --angle {
        syntax: "<angle>";
        inherits: true;
        initial-value: 0turn;
    }

    @keyframes spin {
        to {
            --angle: 1turn;
        }
    }
</style>