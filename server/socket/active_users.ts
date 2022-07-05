import { Server, Socket } from "socket.io";


let io: Server;
let users: any = {};

let sid_to_username: any = {}

export function init_active_users(_io: Server) {
    io = _io;
}


/**
Kan enten bruges til at sende en besked til alle klienter, som er logget ind på brugeren, eller til et bestemt sid.
**/
export function emit_to(username: string | string[], topic: string, data?: any) {
    if (typeof username == 'object') {
        try {
            if (username.length == 0) {
                throw Error
            }
        }
        catch (err) {
            return;
        }


        username.forEach((name: string) => emit_to(name, topic, data));
    }


    if (typeof username == "string") {
        if (username == "!everybody!") {
            for (const user_key of Object.keys(users)) {
                for (const sid of users[user_key]) {
                    io.to(sid).emit(topic, data);
                }
            }

            return;
        }
        
        
        if (username in users) {
            users[username].forEach((sid: string) => {
                io.to(sid).emit(topic, data)
            });
        }
        else {
            if (username == undefined) {
                return;
            }
            else {
                io.to(username).emit(topic, data);
            }
        }
    }
}





export function join_event_active_users(username: string, sid: string) {
    if (!(username in users)) {
        users[username] = [];
    }
    
    users[username].push(sid);
    sid_to_username[sid] = username;
}

export function disconnect_event_active_users(username: string, sid: string) {
    if (username in users) {
        users[username].splice( users[username].findIndex((usid: string) => usid == sid), 1 );
        
        if (users[username].length == 0) {
            delete users[username];
        }
    }
}


export function is_player_online(username: string) {
    return username in users;
}


export function active_users(socket: Socket, io: Server) {
   
}