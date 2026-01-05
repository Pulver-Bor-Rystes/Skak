<script>
    import Logout from "$lib/components/logout.svelte";
    import { friends_menu, new_game_menu } from "$lib/state";

</script>


<div class="grid m-5">
    <div class="grid" style="grid-template-columns: repeat(2, 1fr);">
        <div class="flex">
            
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <button class="menu-btn mr-1 {$new_game_menu ? 'border-text-200':''}" on:click={() => $new_game_menu = true}>Nyt spil</button>
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <button class="menu-btn mr-1 {$friends_menu ? 'border-text-200':''}" on:click={() => $friends_menu = true}>Venner</button>
            
        </div>

        <div class="justify-self-end flex">
            <p class="menu-btn mr-1">Indstillinger</p>
            <Logout/>
        </div>
    </div>
</div>