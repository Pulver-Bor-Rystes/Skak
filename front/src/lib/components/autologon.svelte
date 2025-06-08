<script lang="ts">
    import { cookie, username } from "$lib/state";
    import { socket } from "$lib/ws";
    import { onMount } from "svelte";

    let last_login_was_with_cookie = false;

    $socket.on("login", ({ result, content }) => {
        // console.log({result, content});
        
        if (result) {
            // Hvis forsøget var et normalt login forsøg (altså med kodeord)
            // så gemmer vi cookien
            if (!last_login_was_with_cookie) {
                $cookie = content.Cookie;
                window.location.href = "/home"
            }
        }
        else {
            if (last_login_was_with_cookie) {
                to_login();
            }
            else {
                alert(content);
            }
        }
        
        last_login_was_with_cookie = false;
    })


    // automatic login
    onMount(() => {
        let username_storage = localStorage.getItem('Username');
        let cookie = localStorage.getItem('Cookie');
        if (username_storage == null || cookie == null) {
            // not logged in
            to_login();
            return;
        }

        console.log("last_login_was_with_cookie");

        last_login_was_with_cookie = true;
        $username = username_storage;
        $socket.send('login', { username: username_storage, password: cookie })
    })


    function to_login() {
        if (window.location.href.includes("/login") || window.location.href.includes("/signup")) return;
        window.location.href = "/login";
    }
</script>