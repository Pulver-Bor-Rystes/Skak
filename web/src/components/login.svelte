<script lang="ts">
    import { fly } from 'svelte/transition';
    import { user } from '../stores/user';
    import { socket } from '../networking/ws';
    import { browser } from '$app/environment';

    // variable that bind to html input field's values
    let username: string;
    let password: string;

    // fired when button is pressed
    function login() {
        console.log(password)
        
        $socket.send("login", {
            username,
            password,
        });
    }

    // handle login reply
    $socket.on("login", (data) => {
        if (data.error == "UsernameNotFound") {
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
    $socket.on("signup", (data) => {
        console.log("signup:", data);
        if (data.result) {
            localStorage.setItem("username", username);
            localStorage.setItem("token", data.data);
        }
    })

    // will try to login if there is a token in localstorage
    if (browser) {
        if (localStorage.getItem("token")) {
            $socket.send("login", {
                username: localStorage.getItem("username"),
                password: localStorage.getItem("token")
            })
        }
    }

</script>

<main class="h-screen">
    {#if !$user.logged_in}
        <div transition:fly={{ y: 200, duration: 500 }} class="w-fit translate-x-10 translate-y-10 outline-1 p-5 outline-primary rounded">
            <h1 class="text-primary text-3xl">Tid til at <span class="text-accent"> logge </span> ind 😊</h1>
            <input bind:value={username} class="rounded p-2 mt-5 bg-primary text-white" placeholder="Brugernavn" type="username">
            <input bind:value={password} class="rounded p-2 mt-5 bg-primary text-white" placeholder="Adgangskode" type="password">

            <button on:click={login} class="border-primary-btn border-2 hover:bg-primary-btn transition-colors rounded p-2 mt-5"> Næste </button>
        </div>
    {/if}
</main>


<style lang="postcss">
    :global(html) {
      background-color: rgb(0, 0, 0);
    }
  </style>