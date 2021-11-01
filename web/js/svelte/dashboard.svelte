<script type="ts">
    import { post_request } from './public'

    const firstname_promise = post_request("/api/me/name");
    
    /* Farver

        BG GRÅ: #383941
        

    */
</script>

<main>
    <div />
    <div class="bg">
        {#await firstname_promise}
            <h1> Godaften, ... </h1>
        {:then firstname } 
            <h1> Godaften, {firstname} ;) </h1>
        {:catch err}
            <h1> Du er ikke logget ind </h1>
        {/await}



        <div class="content">
            <div class="third_split">
                <div id="main" class="section">
                    main
                </div>

                <div id="stats" class="section">
                    stats
                </div>
            </div>

            <div id="live" class="section">
                live
            </div>
        </div>
    </div>
</main>

<style type="scss">
    main {
        display: grid;
        grid-template-columns: 0 1fr 0;

        width: 100vw;
        height: 100vh;
    }

    .bg {
        margin: 5rem auto;

        width: 93%;
        max-width: 70rem;

        display: grid;
        grid-template-rows: auto 1fr;

        .content {
            // background: #383941;

            display: grid;
            grid-template-rows: 2fr 1fr;

            .third_split {
                display: grid;
                grid-template-columns: 2fr 1fr;
            }
        }

        h1 {
            color: #fff;
            margin-left: 7px;
        }
    }

    .section {
        background: #383941;
        margin: 7px;

        border-radius: 5px;
    }
</style>
