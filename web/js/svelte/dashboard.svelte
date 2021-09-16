<script type="ts">
    /* Farver

        BG GRÅ: #383941
        

    */
</script>

<main>
    <div />
    <div class="bg">
        <h1>Godaften, Peter ;)</h1>

        <div class="content">
            <div class="third_split">
                <div id="main" class="section" />
                <div id="stats" class="section" />
            </div>

            <div id="live" class="section" />
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
