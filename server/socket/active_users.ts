import { Username } from "shared/types";
import { Server, Socket } from "socket.io";


let io: Server;


type SID = string;
let users: {[key: Username]: SID[]} = {};

let sid_to_username: any = {}


type Callback = (sid: string, username: Username, offline?: boolean) => void;

let join_event_callbacks: Callback[] = [];
let disconnect_event_callbacks: Callback[] = [];


export class ActiveUsers {
    static init (_io: Server) {
        io = _io;
    }

    /**
    Kan enten bruges til at sende en besked til alle klienter, som er logget ind på brugeren, eller til et bestemt sid.
    **/
    static emit_to(username: string | string[], topic: string, data?: any) {
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

    
    static join_event (sid: string, username: Username) {
        if (!(username in users)) {
            users[username] = [];
        }
        
        users[username].push(sid);
        sid_to_username[sid] = username;

        for (let callback of join_event_callbacks) {
            callback (sid, username);
        }
    }


    static disconnect_event (sid: string, username: Username) {
        console.log ("disconnecting:", username)

        if (username in users) {
            let is_offline = false;

            users[username].splice( users[username].findIndex((usid: string) => usid == sid), 1 ); // fjerner sid
            
            if (users[username].length == 0) {
                delete users[username];
                is_offline = true;
            }
            
            
            console.log (`${username} disconnected and is now: ${is_offline ? 'offline':'still online :)'}`)


            
            for (let callback of disconnect_event_callbacks) {
                callback (sid, username, is_offline);
            }
        }
    }





    static subscribe_to_join_event (callback: Callback) {
        join_event_callbacks.push (callback);
    }
    static subscribe_to_disconnect_event (callback: Callback) {
        disconnect_event_callbacks.push (callback);
    }




    static route (route: string, socket: Socket, username: Username) {
        let portal = new Responder (socket, route);

        portal
            .on ("test", (data, answer) => {
                console.log (data);
                answer (null, "muhahahahah")
            })
    }
}


export class Responder {
    socket: Socket;
    title: string;
    // callbacks: ((socket: Socket, topic: string, data: any) => this)[] = [];

    // callbacks: {[key: string]: (socket: Socket, topic: string, data: any) => Responder} 


    constructor (socket: Socket, title: string) {
        this.socket = socket;
        this.title = title;
        return this;
    }

    on (topic: string, callback: (
        data: any, 
        answer: (receiver: SID | Username | string[] | null, response_data: any) => void,
        fail: (receiver: SID | Username | string[] | null, response_data: any) => void
        ) => void) {
        this.socket.on (`${this.title}/${topic}`, data => {
            callback (data, (receiver: SID | Username | string[] | null, response_data: any) => {
                ActiveUsers.emit_to (receiver || this.socket.id, `res:${this.title}/${topic}`, [true, response_data]);
            }, (receiver: SID | Username | string[] | null, response_data: any) => {
                ActiveUsers.emit_to (receiver || this.socket.id, `res:${this.title}/${topic}`, [false, response_data]);
            }
            )
        })

        return this;
    }
}




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