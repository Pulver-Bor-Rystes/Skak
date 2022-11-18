<script lang="ts">
    import { login } from "../stores/networking";
    import { user_data } from "../stores/state";
    import { onMount } from "svelte";
    import "../app.css";

    


    // on mount, send login event via socket.io
    onMount(() => {
        login((resp: Boolean) => {
            if (resp) {
                $user_data.logged_in = true;
                $user_data.username = localStorage.getItem("username") as string;
            }
            else {
                $user_data.login_failed = true;
            }   
        })
    });
</script>

<div hidden class="bg-black text-white"></div>

<slot/>