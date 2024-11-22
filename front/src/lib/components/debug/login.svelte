<script lang="ts">
    import Button from "../button.svelte";
    import { socket } from "$lib/ws";
    import { onMount } from "svelte";
    import { username, cookie, logged_in } from '$lib/state';

    let last_login_was_with_cookie = false;

    // control
    
    function login() {
        console.log('loggin in!');
        $socket.send('login', { username: 'rasmus', password: 'kodeord1234' })
    }

    function logout() {
        $username = ''
        $cookie = '_'
    }


    // react
    $socket.on("login", ({ result, content }) => {
        if (result) {
            $username = "rasmus";
            
            // hvis man har forsøgt at logge ind med cookie, så behøver browseren ikke at gemme nogen oplysninger!
            if (!last_login_was_with_cookie) {
                // localStorage.setItem("Cookie", content.Cookie);
                $cookie = content.Cookie;
                console.log(' > Cookie saved')
            }
        }
        else {
            console.log("could not login", content);
        }
        last_login_was_with_cookie = false;
    })


    // automatic login
    onMount(() => {
        let username = localStorage.getItem('Username');
        let cookie = localStorage.getItem('Cookie');
        if (username == null || cookie == null) return;

        last_login_was_with_cookie = true;
        $socket.send('login', { username, password: cookie })
    })
</script>


<div>
    <Button fn={login} color='#54FE69'> Log ind </Button>
    <Button fn={logout} color='#FF2947'> Log ud </Button>
</div>