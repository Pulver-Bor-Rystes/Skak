<script lang="ts">
    import Button from '../components/basic_btn.svelte';
    import { socket } from '../networking/ws';
    import { user } from '../stores/user';

    let players: string[] = [];

    $socket.on("page", (data) => {
        console.log("page:", data);
    })

    $socket.on("active_players", (data) => {
        console.log(data)
        players = data.content.filter((username: string) => {
            if (username == $user.username) {
                return false;
            }
            return true;
        });
    })

    $socket.on("state", ({ content }) => {
        console.log("FEN:", content)
    })

    $socket.send("getstate", {})
</script>

