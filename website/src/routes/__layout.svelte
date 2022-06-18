<script lang="ts">
    import { user_data } from "../stores/state";
    import { onMount } from "svelte";
    import "../app.css";

    function get_from_cookie(name: string) {
        var nameEQ = name + "=";
        var ca = document.cookie.split(';');
        for (var i = 0; i < ca.length; i++) {
            var c = ca[i];
            while (c.charAt(0) == ' ') c = c.substring(1, c.length);
            if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length, c.length);
        }
        return null;
    }


    // on mount, send login event via socket.io
    onMount(() => {
        // get username
        var username = get_from_cookie("username");
        // get cookie
        var cookie = get_from_cookie("cookie");
        
        // attach socket from window's variable
        // @ts-ignore
        const socket = window.socket;
        socket.emit("login", username, cookie);

        socket
            .on("login_success", () => {
                console.log("login success");
                $user_data.logged_in = true;
                $user_data.username = username as string;
            })
            .on("login_failure", () => {
                console.log("login failure");
                if (!window.location.href.includes("/auth")) {
                    window.location.href = "/auth"
                }
            })
    });
</script>

<div hidden class="bg-black text-white"></div>

<slot/>