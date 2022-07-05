<script lang="ts">
    import type { Socket } from "socket.io";
    import { get_socket } from "../../stores/networking";
    import { user_data, friends, inv_received, inv_send } from "../../stores/state";
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


            // alert(res)
            console.log(res)
        }))



        socket.on("res:friends/fetch", (data) => {
            $friends = data[0];
            $inv_received = data[1];
            $inv_send = data[2];
        })
    })




</script>