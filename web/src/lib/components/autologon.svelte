<script lang="ts">
    import { cookie, logged_in, username } from "$lib/state";
    import { socket } from "$lib/ws";
    import { onMount } from "svelte";

    $socket.on("login:cookie", (result, content) => {
        if (result) {
            $logged_in = true;
            $username = localStorage.getItem("Username") || "...";
        } else {
            $logged_in = false;
            $cookie = "";
        }
    });

    // automatic login
    onMount(() => {
        let username_storage = localStorage.getItem("Username");
        let cookie = localStorage.getItem("Cookie");
        if (username_storage == null || cookie == null) {
            // not logged in
            to_login();
            return;
        }

        $username = username_storage;
        $socket.send("login:cookie", {
            username: username_storage,
            cookie: cookie,
        });
    });

    function to_login() {
        if (
            window.location.href.includes("/login") ||
            window.location.href.includes("/signup")
        )
            return;
        // window.location.href = "/login";
    }
</script>
