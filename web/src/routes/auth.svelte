<script lang="ts">
    import { socket, logged_in } from "../stores/state"
    import { browser } from '$app/environment'

    // binder variables
    let username = ""
    let password = ""
    let cookie = ""

    // not that important binder elements
    let btn1;
    let btn2;
    let password_field;


    // reactive variables
    let browser_just_loaded = false
    let username_btn;
    let valid_username = false
    let next = false

    // load data
    if (browser) {
        username = localStorage.getItem("username") || ""
        cookie = localStorage.getItem("cookie") || ""

                
        if (username != "" && cookie != "") {
            login()
        } else {
            browser_just_loaded = true
        }
    }

    // interactive functions
    function verify_username_input(ev) {
        // bliver kaldt når taster bliver trykket på
        if (ev?.key == "Enter") {
            next_slide()
            return
        }
        if (!username || username.length < 3) {
            valid_username = false
        }
        else {
            valid_username = true
        }
    }

    function login() {
        verify_username_input()
        if (!valid_username) return;

        console.log({ username, password: password == "" ? cookie:password })
        $socket.emit("login", { username, password: password == "" ? cookie:password })
    }

    function next_slide() {
        if (!valid_username) return

        next = true
        setTimeout(() => {
            password_field.focus()
        }, 75*2)
    }

    // react to data
    $socket.on("/login", (resp) => {
        if (resp.ok) {
            $logged_in = true
            // save data
            localStorage.setItem("username", username)
            localStorage.setItem("cookie", resp.data)
        }

        browser_just_loaded = true

        // react to faulty data
    })
</script>


<!-- hvis enten browseren lige har åbnet siden, eller hvis brugeren er logget ind, så skal den ikke vise log ind -->
{#if !(!browser_just_loaded || $logged_in)}
    <div class="absolute h-full w-full top-0 flex items-center justify-center">
        <div class="w-96 bg-[#111827] rounded-lg shadow overflow-hidden">
            <h1 class="text-white ml-5 mt-5 text-3xl">
                Log ind
            </h1>


            <div class="w-[200%] transition-transform delay-75 flex { next ? 'translate-x-[-50%]':''}">
                <div class="m-5 mt-8 flex-1">
                    <label class="text-white" for="username">Brugernavn</label>
                    <div class="flex mt-2">
                        <input on:keyup={verify_username_input} bind:value={username} class="p-3 w-[fill-available] rounded bg-[#1E293B] focus:outline-none focus:ring ring-inset-2 ring-blue-500 placeholder:italic placeholder:text-slate-400 text-white" type="username" name="username" placeholder="Skriv dit brugernavn">
                        <button bind:this={btn1} class="ml-2 w-14 bg-green-600 disabled:bg-green-800 rounded flex items-center justify-center">
                            <i class="fa-solid fa-circle-right text-2xl"></i>
                        </button>
                    </div>
                </div>
                <div class="m-5 mt-8 flex-1">
                    <label class="text-white" for="password">Logger ind som <span class="italic">{username}</span></label>
                    <div class="flex mt-2">
                        <input bind:value={password} bind:this={password_field} class="p-3 w-[fill-available] rounded bg-[#1E293B] focus:outline-none focus:ring ring-inset-2 ring-blue-500 placeholder:italic placeholder:text-slate-400 text-white" type="password" name="password" placeholder="Skriv dit kodeord">
                        <button bind:this={btn2} on:click={login} class="ml-2 w-14 bg-green-600 disabled:bg-green-800 rounded flex items-center justify-center">
                            <i class="fa-solid fa-circle-right text-2xl"></i>
                        </button>
                    </div>
                </div>
            </div>
        </div>
    </div>
{/if}