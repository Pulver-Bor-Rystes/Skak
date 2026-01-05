<script lang="ts">
    import { cookie, username } from "$lib/state";
    import { socket } from "$lib/ws";

    let username_str: string;
    let password: string;

    $socket.on("signup", ({result, content}) => {
        if (!result) {
            alert(content)
            return;
        }

        $cookie = content;
        $username = username_str;

        window.location.href = "/home"
    })

    const signup = (ev: any) => {
        if (!(ev?.button === 0 || ev?.key === "Enter")) return;

        $socket.send("signup", { username: username_str, password });
    }
</script>


<div class="w-100 h-96 grid place-items-center grid-cols-3">
    <div></div>
    <div class="w-[200px]">
        <h2 class="mb-2"> Opret konto </h2>

        <form action="/api/signup" method="post">
            <input on:keydown={signup} type="username" placeholder="Indtast dit brugernavn" bind:value={username_str} class="my_input" />
            <input on:keydown={signup} type="password" placeholder="Indtast dit kodeord" bind:value={password} class="my_input mt-5" />
    
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_static_element_interactions -->
            <div on:click={signup} class="grid mt-5">
                <div class="auth_btn"> Log ind </div>
            </div>
        </form>
    </div>
    <div></div>
</div>