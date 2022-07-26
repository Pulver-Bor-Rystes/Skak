<script lang="ts">
    import type { Socket } from "socket.io";
    import { get_socket } from "../../stores/networking";
    import { user_data, friends, inv_received, inv_send, games, G_inv_recieved, G_inv_send } from "../../stores/state";
    import { onMount } from "svelte";

    let socket: Socket;

    onMount(() => {
        socket = get_socket()

        socket.on("res:friends/request", (([res, data]: any[]) => {
            switch (res) {
                case "accepted":
                    
                    break;
            
                case "pending":
                    
                    break;

                    
                default:
                    break;
            }


            // socket.emit("friends/fetch")

            console.log (res, data)

            // alert(res)
        }))


        socket.on("notif:games/state", ([gs, rec, send]) => {
            $games = gs;
            $G_inv_recieved = rec;
            $G_inv_send = send;
        })



        socket.on("res:friends/fetch", (data) => {
            console.log (data)
            $friends = data[0];
            $inv_received = data[1];
            $inv_send = data[2];
        })


        let iid = setInterval (() => {
            if ($user_data.login_failed) clearInterval (iid);
            if ($user_data.logged_in) {
                socket.emit ("friends/fetch")
                socket.emit ("friends/get")
                socket.emit ("games/get")
                clearInterval (iid);
            }

        }, 100)
    })




</script>