<script type="ts">
    import { post_request, logged_in } from './public'

    
</script>


<h1> Godaften { $logged_in } </h1>

<div class="layout">
    <div/>

    <div class="dashboard">
    
    </div>
</div>




<style type="scss">
    h1, h2, h3, p {
        color: #fff;
    }

    .layout {
        display: grid;
        grid-template-columns: 1fr 1fr 1fr;

        width: 100vw;
        height: 100vh;
    }

    .dashboard {
        
        background-color: aqua;
    }
</style>
