<div class="absolute h-full w-full top-0 flex items-center justify-center">
    <div class="w-96 bg-[#111827] rounded-lg shadow overflow-hidden">
        <h1 class="text-white ml-5 mt-5 text-3xl">
            Log ind
            <!-- <i class="fa-solid fa-lock float-right text-indigo-500"></i> -->
        </h1>

        <!-- <label class="ml-5 text-gray-400" for="">logger ind som <span class="italics">rasmus</span></label> -->

        <div class="w-[200%] transition-transform delay-75 flex { next ? 'translate-x-[-50%]':''}">
            <div class="m-5 mt-8 flex-1">
                <label class="text-white" for="username">Brugernavn</label>
                <div class="flex mt-2">
                    <input on:keyup={username_input_listen_for_keypress} bind:value={username_input_value} class="p-3 w-[fill-available] rounded bg-[#1E293B] focus:outline-none focus:ring ring-inset-2 ring-blue-500 placeholder:italic placeholder:text-slate-400 text-white" type="username" name="username" placeholder="Skriv dit brugernavn">
                    <button bind:this={username_btn} on:click={header_click} class="ml-2 w-14 bg-green-600 disabled:bg-green-800 rounded flex items-center justify-center">
                        <i class="fa-solid fa-circle-right text-2xl"></i>
                    </button>
                </div>
            </div>
            <div class="m-5 mt-8 flex-1">
                <label class="text-white" for="password">Logger ind som <span class="italic">{username_input_value}</span></label>
                <div class="flex mt-2">
                    <input bind:value={password_input_value} class="p-3 w-[fill-available] rounded bg-[#1E293B] focus:outline-none focus:ring ring-inset-2 ring-blue-500 placeholder:italic placeholder:text-slate-400 text-white" type="password" name="password" placeholder="Skriv dit kodeord">
                    <button class="ml-2 w-14 bg-green-600 disabled:bg-green-800 rounded flex items-center justify-center">
                        <i class="fa-solid fa-circle-right text-2xl"></i>
                    </button>
                </div>
            </div>
        </div>
    </div>
</div>


<script lang="ts">
    let next = false // skift til true, for at bede om kodeord
    let _next = next

    let username_btn: HTMLButtonElement
    let username_valid: 'Exists'|'New'|'Nope' = 'Nope'
    $: {
        if (username_btn) {
            if (username_valid == 'Nope') {
                username_btn.setAttribute("disabled", "true")
            }
            else {
                username_btn.removeAttribute("disabled")
            }
        }
    }

    function username_input_listen_for_keypress() {
        username_valid = verify_username_input()
    }

    let username_input_value: string
    let password_input_value: string

    function header_click() {
        username_valid = verify_username_input()
        next = next ? false:username_valid != 'Nope' ? true:false
    }

    function verify_username_input() {
        if (!username_input_value) return 'Nope';
        if (username_input_value.length < 3) return 'Nope';
        return 'New'
    }

    // opdater hver gang next ændrer sig til true
    $: { // og fjern teksten fra password feltet
        if (next != _next) {
            _next = next

            if (next) {
                password_input_value = ""
            }
        }
        
    }
</script>