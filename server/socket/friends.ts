import { Server, Socket } from "socket.io";



export function friends(socket: Socket, io: Server) {
    socket.on("request_friendship", (player_name: string) => {
        console.log(player_name)
    })


    socket.on("accept_friendship", (player_name: string) => {

    })
}