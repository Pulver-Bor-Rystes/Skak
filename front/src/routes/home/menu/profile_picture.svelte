<script>
    import { username } from "$lib/state";

</script>


<div class="grid-profile def-rounded pl-2 pr-2">
    <div class="w-16 h-16 bg-background-400 self-center my-rounded"></div>
    <!-- <div class="grid">
        <h2 class="pl-5 self-end"> Rasmus </h2>
        <p class="pl-5 -mt-3 self-start">1000</p>
    </div> -->

    <h2 class="self-center items-center justify-center pl-5">{$username}</h2>
</div>