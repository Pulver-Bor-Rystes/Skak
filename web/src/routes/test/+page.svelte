<script>
    import { socket } from "$lib/ws";



    function login() {
        $socket.send("login", { username: "rasmus", password: "kodeord1234"});
    }
    function login2() {
        $socket.send("login", { username: "nikolaj", password: "kodeord1234"});
    }

    
    function invite() {
        $socket.send("invite_player", { to: "nikolaj" });
    }
</script>


<!-- svelte-ignore a11y_click_events_have_key_events -->
<!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
<p on:click={ login }>LOGIN Rasmus</p>
<p on:click={ login2 }>LOGIN Nikolaj</p>

<p on:click={ invite }>INVITE</p>

<style>
    p:hover {
        cursor: pointer;
        background-color: blue;
    }
</style>
