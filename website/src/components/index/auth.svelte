<script lang="ts">
    import { onMount } from "svelte";
    import { fly } from "svelte/transition"
    import { login, npost } from "../../stores/networking";
    import { user_data } from "../../stores/state";

    // bindings
    let bind_username: HTMLInputElement;
    let bind_password: HTMLInputElement;


    let stored_username: string;

    onMount(() => {
        stored_username = localStorage.getItem("username") || "";
    })


    let switch_deb = false;


    $: {
        if ($user_data.login_failed && !switch_deb && bind_password && bind_username) {
            switch_deb = true;

            if (stored_username == "") {
                bind_username.focus();
            }
            else {
                bind_password.focus();
            }
        }
    }


    let psw_level = 0;
    let psw_levels = ["", "border-green-500", "border-red-500"];
    let input_dark_css = "dark:bg-gray-700 dark:border-none dark:text-white"




    async function login_click(event: any) {
        // Returnere hvis tasten ikke er enter
        if ("key" in event) {
            if (event.key != "Enter") {
                return;
            }
        }


        let resp = await npost('/api/v1/auth/login', {
            'username': bind_username.value,
            'password': bind_password.value,
        });

        if (resp.status) {
            login( (resp: Boolean) => {
                console.log("loggin in")
                if (resp) {
                    $user_data.logged_in = true;
                    $user_data.login_failed = false;
                    $user_data.username = localStorage.getItem("username") as string;
                }
                else {
                    $user_data.login_failed = true;
                }   
            })
        }
        else {
            console.log(resp?.errors);
        }
    }
</script>

{#if $user_data.login_failed}
    <div transition:fly={{ y: -200, duration: 500 }} on:keypress={login_click} class="fixed flex justify-center w-full h-full top-0 backdrop-blur">
        <div class="w-full max-w-xs self-center">
            <form class="bg-white dark:bg-zinc-900 shadow-md rounded px-8 pt-6 pb-8 mb-4">
                <div class="mb-4">
                    <label
                        class="block text-gray-700 text-sm font-bold mb-2"
                        for="username"
                    >
                        Brugernavn
                    </label>
                    <input
                        class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline {input_dark_css}"
                        id="username"
                        type="text"
                        placeholder="Brugernavn"
                        value={stored_username}
                        bind:this={bind_username}
                    />
                </div>
                <div class="mb-6">
                    <label
                        class="block text-gray-700 text-sm font-bold mb-2"
                        for="password"
                    >
                        Kodeord
                    </label>
                    <input
                        class="shadow appearance-none border {psw_levels[psw_level]} rounded w-full py-2 px-3 text-gray-700 mb-3 leading-tight focus:outline-none focus:shadow-outline {input_dark_css}"
                        id="password"
                        type="password"
                        bind:this={bind_password}
                    />
                    {#if psw_level == 2}
                        <p class="text-red-500 text-xs italic">
                            Forkert kode.
                        </p>
                    {/if}
                </div>
                <div class="flex items-center justify-between">
                    <button
                        class="bg-blue-700 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline"
                        type="button"
                        on:click={login_click}
                    >
                        Log ind
                    </button>
                    <div class="grid">
                        <a
                            class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800"
                            href="#"
                        >
                            Glemt kodeord?
                        </a>
                        <a
                            class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800"
                            href="#"
                        >
                            Opret konto
                        </a>
                    </div>
                </div>
            </form>
        </div>
    </div>
{/if}
