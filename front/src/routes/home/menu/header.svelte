<script>
    import LoadingEffect from "$lib/components/loading_effect.svelte";
    import ProfileInfo from "./profile_picture.svelte";
    import Logout from "$lib/components/logout.svelte";
    import { friends_menu, new_game_menu, rating, username } from "$lib/state";
    import Menubar from "./menubar.svelte";
</script>


<div class="grid my-grid ">
    <ProfileInfo/>
    <Menubar/>
</div>