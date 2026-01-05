<script lang="ts">
    import { friends_menu, new_game_menu, rating, username, invited } from "$lib/state";
    import Header from "./menu/header.svelte";
    import NewGame from "./widgets/new_game.svelte";
    import Friends from "./widgets/friends.svelte";
    import PendingInvite from "./widgets/pending_invite.svelte";

</script>



<div class="grid m-5">
    <Header/>

    <!-- <div class="border border-[#E4E4E4] mt-5"></div> -->

    <div class="mt-5 flex flex-col gap-5">
        {#if $new_game_menu}
            <NewGame/>
        {:else if $friends_menu}
            <Friends/>
        {:else}
            <div>
                <h1>Hej {$username} 👋</h1>
                <p>
                    Velkommen til min ydmyge skak side.

                    <br>

                    Du kan evt. tjekke projektet ud på github. Link kommer snart!
                </p>
            </div>
        {/if}
    </div>


    {#each $invited as invite}
        <PendingInvite who={invite}/>
    {/each}
</div>
