import { Server, Socket } from "socket.io";
import { emit_to, is_player_online } from "./active_users";
import { _request } from "./friends";


let lobby: any = {}
let invitations: any = {}

export function lobby_socket(socket: Socket, username: string) {
    socket.on("lobby/join", () => {
        if (username in lobby) {
            // nothing
            emit_to(socket.id, "res:lobby/join", format_lobby())
            return;
        }


        // TODO: Tjek at spilleren ikke er i gang med et spil.

        lobby[username] = true;
        emit_to("!everybody!", "res:lobby/join", format_lobby())
    })


    socket.on("lobby/invite", (invitee_username: string) => {
        // TODO: Tjek at spilleren ikke er i gang med et spil
        
        
        let res: string;
        [invitations, res] = _request(invitations, username, invitee_username);

        emit_to(username, "res:lobby/invite", res)
        // TODO: GAME.new
    })
}


export function join_event_lobby(sid: string, username: string) {
    if (username in lobby) {
        emit_to(sid, "res:lobby/join", format_lobby())
    }
}


export function disconnect_event_lobby(username: string) {
    if (username in lobby) {
        if (!is_player_online(username)) {
            delete lobby[username];
            emit_to("!everybody!", "res:lobby/join", format_lobby())
        }
    }
}




function format_lobby() {
    return Object.keys(lobby)
}