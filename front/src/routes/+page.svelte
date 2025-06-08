<div class="w-100 h-96 grid place-items-center grid-cols-3">
    <div>
    </div>
    <div class="w-[200px]">
        <h1 class=""> Skak </h1>
        <p class="">{@html text}</p>
        {#if show}
        <!-- svelte-ignore a11y_click_events_have_key_events -->
        <!-- svelte-ignore a11y_no_static_element_interactions -->
        <div on:click={login} in:fade={{duration: 3000, delay: 100}} class="grid mt-5">
            <div class="auth_btn"> Kom i gang </div>
        </div>
        {/if}
    </div>
    <div>
        {#if right_board_show}
            <div 
                in:fly={{duration: 1500, y: 100}}
                out:fly={{duration: 1500, y: -100}}
                class="w-[50px] h-[50px] bg-primary-300"></div>            
        {/if}
    </div>
</div>




<script lang="ts">
    import { fade, fly } from "svelte/transition";
    let show = false;
    let right_board_show = false;
    let start = 3000;
    let after = 1000;

    make_loop();


    function make_loop() {
        right_board_show = true;


        setTimeout(() => {
            right_board_show = false;
            setTimeout(() => {
                make_loop();
            }, start + randomTime());
        }, after);
    }
    
    
    // random amount of time between 3 and 5 seconds
    const randomTime = () => {
        return Math.floor(Math.random() * (5000 - 3000 + 1)) + 3000;
    }

    let things_to_write = [
        "👋 Hej med dig!<br>Her kan du udfordre venner, familie og ikke mindst AI 🧠",
        "A game of strategy 🧠",
        "A game of skill 🥷",
        "A game of patience",
        "A game of tactics 🧐",
        "A game of wits",
    ]
    let i = 0;
    let text = "";
    let erase = false;
    let wait = 0;


    setTimeout(() => {
        const interval = setInterval(() => {
            // venter lidt tid før vi skriver igen
            if (wait > 0) {
                wait -= 1;
                return;
            }

            // hvis slutningen af teksten er nået, så skift til næste
            if (erase) {
                if (text == "") {
                    erase = false;
                    i = (i + 1) % things_to_write.length;
                }
                else {
                    // erase
                    text = text.slice(0, text.length - 1);
                }
            }
            // ellers skriv
            else {
                text = things_to_write[i].slice(0, text.length + 1);
                text = text.replaceAll("_", "<br>");
            }


            // hvis teksten er nået, så vent lidt
            if (text === things_to_write[i]) {
                clearInterval(interval);
                erase = true
                wait = 64;
                show = true;
            };
        }, 50);
    }, 1000);


    function login(event: MouseEvent) {
        event.preventDefault();
        window.location.href = "/login";
    }
</script>