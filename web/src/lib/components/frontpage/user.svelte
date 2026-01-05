<script lang="ts">
    import { socket } from "$lib/ws";
    let { name = "", rating = 0 } = $props();

    function click() {
        console.log("User clicked:", name);

        $socket.send("game:invite", { user: name });
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore event_directive_deprecated -->
<p1 on:click={click} class="hover:cursor-pointer">{name} ({rating})</p1>
