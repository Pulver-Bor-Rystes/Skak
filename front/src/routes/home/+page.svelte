<script lang="ts">
    import { friends_menu, new_game_menu, rating, username } from "$lib/state";
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
                <p>Først og fremmest, tak for at give min skakside et forsøg!</p>
                <p>Her på siden kan du spille skak, næsten lige så godt som chess.com! ...bare meget værre 😅</p>
                <p>For at komme i gang, skal du blotte trykke på "Nyt Spil" oppe i menuen</p>
            </div>
        {/if}
    </div>


    <PendingInvite/>
</div>