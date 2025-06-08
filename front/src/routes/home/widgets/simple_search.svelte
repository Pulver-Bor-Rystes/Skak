<script lang="ts">
    import Button from "$lib/components/button.svelte";
    import { active_players, engines, rating } from "$lib/state";
    import { socket } from "$lib/ws";

    let filtered_list: string[] = $state([]);
    let search_str: string = $state("");
    let selected_user = $state("");

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

    function update(ev: KeyboardEvent | MouseEvent) {
        search_for()
    }


    function search_for() {
        let list = compile_list()
        
        // filter
        filtered_list = list.filter((item) => {
            return item.toLowerCase().includes(search_str.toLowerCase());
        });
    }


    function send_invite() {
        let name = deformat_player(selected_user);
        
        $socket.send("newgame", {
            username: name,
            timeformat: "",
        })
    }
</script>



<!-- Søgefunktion -->
<div class="grid my-rounded bg-text-100 p-3 mr-1">
    <h3 class="mb-5">Søg efter spillere</h3>

    <!-- svelte-ignore event_directive_deprecated -->
    <input on:click={update} on:keyup={update} bind:value={search_str} class="my_input w-full pl-1" placeholder="Indtast søgeord" type="text">
    

    <div class="h-40 overflow-auto">
        {#each filtered_list as item}
            <!-- svelte-ignore a11y_click_events_have_key_events -->
            <!-- svelte-ignore a11y_no_noninteractive_element_interactions -->
            <!-- svelte-ignore event_directive_deprecated -->
            <p on:click={ () => selected_user = item } class="m-2 hover:underline hover:cursor-pointer">{item}</p>
        {/each}
    </div>

    <div>

        <!-- Start game -->
        {#if selected_user == ""}
            <Button class_extra="w-full mt-2 !text-[#00000047] hover:cursor-default hover:shadow"> Vælg en modstander </Button>
        {:else}
            <Button fn={send_invite} class_extra="w-full mt-2"> Send en udfordring til { selected_user } </Button>
        {/if}
    </div>
    
</div>
