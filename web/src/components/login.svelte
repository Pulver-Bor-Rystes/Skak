<script lang="ts">
    import { fly } from 'svelte/transition';
    import { user } from '../stores/user';
    import { socket } from '../networking/ws';
    import { browser } from '$app/environment';

    // variable that bind to html input field's values
    let username: string;
    let password: string;

    let show_prompt = false;
    let ignore_login_reply = false;

    // fired when button is pressed
    function login() {
        $socket.send("login", {
            username,
            password,
        });
    }


    function save_login_info(token: string | null = null) {
        localStorage.setItem("username", username);
        if (token) {
            localStorage.setItem("token", token);
        }
    }


    // handle login reply
    $socket.on("login", (msg) => {
        if (msg.result) {
            $user.logged_in = true;
            $user.username = username;

            save_login_info(msg.content.Cookie);

            return
        }
        show_prompt = true

        if (ignore_login_reply) {
            ignore_login_reply = false;
            return;
        }
        
        if (msg.content == "UsernameNotFound") {
            let resp = prompt("Brugernavn ikke fundet, vil du oprette en bruger? (y/n)");
            if (resp?.toLowerCase().includes("y")) {
                $socket.send("signup", {
                    username,
                    password,
                })
            }
        }
    })

    // handle signup reply
    $socket.on("signup", (msg) => {
        if (msg.result) {
            save_login_info(msg.content);
            $user.logged_in = true;
            $user.username = username;
        }
    })

    // will try to login if there is a token in localstorage

    if (browser) {
        if (localStorage.getItem("token")) {
            username = localStorage.getItem("username") || "";
            ignore_login_reply = true; // Den skal ikke forsøge at oprette en bruger når den prøver automatisk at logge ind
            $socket.send("login", {
                username: localStorage.getItem("username"),
                password: localStorage.getItem("token")
            })
        }
        else {
            show_prompt = true;
        }
    }

</script>

{#if !$user.logged_in && show_prompt}
    <div class="absolute top-0 w-screen h-screen bg-black">
        <div transition:fly={{ y: 200, duration: 500 }} class="w-fit translate-x-10 translate-y-10 outline-1 p-5 outline-primary rounded">
            <h1 class="text-primary text-3xl">Tid til at <span class="text-accent"> logge </span> ind 😊</h1>
            <input bind:value={username} class="rounded p-2 mt-5 bg-primary text-white" placeholder="Brugernavn" type="username">
            <input bind:value={password} class="rounded p-2 mt-5 bg-primary text-white" placeholder="Adgangskode" type="password">

            <button on:click={login} class="border-primary-btn border-2 hover:bg-primary-btn transition-colors rounded p-2 mt-5"> Næste </button>
        </div>
    </div>
{/if}