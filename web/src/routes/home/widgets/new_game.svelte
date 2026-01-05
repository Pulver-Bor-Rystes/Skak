<script lang="ts">
    import { active_players, engines, invited, new_game_menu } from "$lib/state";
    import { socket } from "$lib/ws";
    import { fly } from "svelte/transition";


    let filtered_list: string[] = $state([]);
    let search_str: string = $state("");
    let selected = $state(-1);

    engines.subscribe(v => setTimeout(search_for, 10));
    active_players.subscribe(() => setTimeout(search_for, 10));

    const bot_tag = " (bot)";
    const format_player = (name: string, is_bot = false): string => is_bot ? name + bot_tag : name;
    const deformat_player = (name: string): string => name.includes(bot_tag) ? name.split(bot_tag)[0] : name;

    
    function compile_list(): string[] {
        let list: string[] = [];

        for (let engine of $engines) {
            list.push(format_player(engine, true));
        }

        for (let player of $active_players) {
            list.push(player)
        }

        return list;
    }

    function update(ev: KeyboardEvent) {
        search_for()

        if (ev.key == "ArrowDown") {
            ev.preventDefault();
            selected += 1;

            if (selected >= filtered_list.length) {
                selected = 0;
            }
        }
        else if (ev.key == "ArrowUp") {
            ev.preventDefault();
            selected -= 1;

            if (selected < 0) {
                selected = filtered_list.length - 1;
            }
        }
        else if (ev.key == "Enter") {
            ev.preventDefault();
            send_invite();
        }

        while (selected >= filtered_list.length) {
            selected -= 1;
        }

        if (selected == -1) {
            selected = 0;
        }
    }


    function search_for() {
        let list = compile_list()
        
        // filter
        filtered_list = list.filter((item) => {
            return item.toLowerCase().includes(search_str.toLowerCase());
        });
    }


    function send_invite() {
        if (selected < 0 || selected >= filtered_list.length) {
            return;
        }

        let player = deformat_player(filtered_list[selected]);
        console.log("Sending invite to", player);


        $socket.send("invite_player", { to: player });


        $invited.push(player);
        $invited = $invited;
    }
</script>



<div in:fly={{x: -1500}} class=" min-w-[300px] w-2/4 max-w-[500px]">
    <h2 class="mb-2 p-2 pb-0">Nyt spil?</h2>

    <!-- svelte-ignore event_directive_deprecated -->
    <input on:click={search_for} on:keyup={update} bind:value={search_str} class="p-2 outline-none" type="text" placeholder="Søg efter modstander">


    <div class="h-40 overflow-auto">
        {#each filtered_list as item, i}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore event_directive_deprecated -->
            <p on:click={ () => selected = i } class="m-2 hover:underline hover:cursor-pointer">
                {#if selected == i}
                    &rAarr; {item} &lAarr;
                {:else}
                    {item}
                {/if}
            </p>
        {/each}
    </div>


    {#if filtered_list.length > 0 && selected >= 0}
        <!-- svelte-ignore event_directive_deprecated -->
        <button on:click={() => send_invite()} class="menu-btn mt-5 w-full">Send invitation til: <span class="font-bold">{filtered_list[selected]}</span></button>
    {/if}
</div>
