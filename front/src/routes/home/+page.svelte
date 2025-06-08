<script lang="ts">
    import LoadingEffect from "$lib/components/loading_effect.svelte";
    import ProfilePicture from "./profile_picture.svelte";
    import LoadData from "../../lib/components/load_data.svelte";
    import SimpleSearchWidget from "./widgets/simple_search.svelte";
    import { rating, username } from "$lib/state";
    import Logout from "$lib/components/logout.svelte";
    import BoardWidget from "./widgets/board.svelte";
    import Chessboard from "$lib/components/board/chessboard.svelte";

</script>

<div class="grid m-5">
    <!-- Header thingy -->
    <div class="grid my-grid ">
        <ProfilePicture/>
        <div class="grid m-5">
            <h2 class="my-align-top"> {$username} </h2>

            <div class="grid" style="grid-template-columns: repeat(2, 1fr);">
                <div class="flex">
                    <p class="hover:underline hover:cursor-pointer">
                        Rating:
                        {#if rating}
                            { $rating }
                        {:else}
                            <LoadingEffect character="-" max={4} start="----"/>
                        {/if}
                    </p>
    
                    <div class="ml-2 mr-2">|</div>
                    
                    <p class="hover:underline hover:cursor-pointer">
                        Venner
                    </p>
                </div>


                <Logout/>
            </div>
        </div>
    </div>

    <div class="grid grid-cols-2">
        <SimpleSearchWidget/>
        <BoardWidget/>

        <!-- <Chessboard initial_fen="r2q2k1/pb3p1p/1pp5/2b3p1/3p4/1B4QP/PPP2PP1/3R1RK1 w - - 0 19"/> -->
    </div>
</div>