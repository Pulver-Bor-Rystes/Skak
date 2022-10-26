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

    /** Når spilleren joiner, logges det her */
    static join_event (socket: Socket) {
        const username = socket.data.username;
        const sid = socket.id;
        
        if (!(username in users)) {
            users[username] = [];
        }
        
        users[username].push(sid);
        sid_to_username[sid] = username;

        // Lader alle andre vide at nu er spilleren online
        for (let callback of join_event_callbacks) {
            callback (sid, username);
        }
    }

    /** Når spilleren disconnecter, logges det her */
    static disconnect_event(socket: Socket) {
        const username = socket.data.username;
        const sid = socket.id;

        if (username in users) {
            let is_offline = false;

            users[username].splice( users[username].findIndex((usid: string) => usid == sid), 1 ); // fjerner sid
            
            if (users[username].length == 0) {
                delete users[username];
                is_offline = true;
            }
            
            
            console.log (`${username} disconnected and is now: ${is_offline ? 'offline':'still online :)'}`)


            // Lader alle andre vide at spilleren er logget af.
            // Men da en spiller kan være logget på samtidigt flere steder, videregiver vi også variablen is_offline,
            // så programmet ved om spilleren er helt offline, eller om der stadig er en browser logget på.
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




    static route (route: string, socket: Socket) {
        const username = socket.data.username;
        const portal = new Responder (socket, route);

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



export class State3 {
    connections: { [key: string]: Boolean } = {};
    invitations: { [key: string]: Boolean } = {};

    /** Underretter ens ven eller den person brugeren har en forbindelse til */
    notify_conn_mate = false;
    topic: string;
    new_conn_state: any;


    // connection, through, handle, discover, interaction
    constructor (topic?: string, new_conn_state?: any) {
        this.topic = topic || "";
        this.new_conn_state = new_conn_state || true;
    }


    log () {
        console.log (this.topic + ": CONNS & INVS")
        console.log (this.connections)
        console.log (this.invitations)
    }



    /** En funktion som håndterer når en spiller prøver at invitere/acceptere. Erstatter det gamle request/invite */
    handle_interaction (from: Username, to: Username) {
        // Gør alt hvad den normale request funktion gjorde, men underretter også de påvirkede spillere
        // Dog er der nogen gange hvor alle på netværket skal vide hvad der sker.

        if (from == to) return;
        if (this.connection_exists (from, to)) return;
        
        let res: string;
        [this.invitations, res] = this.request (this.invitations, from, to);

        if (["accepted", "pending"].includes (res)) {
            this.update_player_state (from, true);
        }
        else {
            emit_to (from, `${this.topic}/feedback`, [res, to]);
        }

        return res;
    }

    connection_exists (from: Username, to: Username) {
        for (let conn of Object.keys (this.connections)) {
            if (conn.includes (from) && conn.includes (to)) {
                return true;
            }
        }

        return false;
    }

    /** En funktion som sender en masse informationer til en bestemt spiller */
    update_player_state (name: Username, _notify_conn_mate?: boolean) {
        const user_and_conn_mates = [name].concat (this.get_conn_mates (name));

        let state = new State3;
        
        for (const name of user_and_conn_mates) {
            state.apply_state_by_name (this, name);
        }

        emit_to ((this.notify_conn_mate || _notify_conn_mate) ? user_and_conn_mates:name, `${this.topic}/update`, {
            "connections": state.connections, 
            "invitations": state.invitations,
        })
    }


    get_conn_mates (name: Username) {
        let mates: Username[] = [];

        for (const conn of Object.keys (this.connections)) {
            if (!conn.includes (name)) {
                continue;
            }

            const names = conn.split ("&");
            mates.push (names[0] == name ? names[1]:names[0]);
        }

        return mates;
    }


    apply_state_by_name (old_state: State3, name: Username) {
        for (const conn of Object.keys (old_state.connections))
            if (conn.includes (name))
                this.connections[conn] = true;

        for (const conn of Object.keys (old_state.invitations))
            if (conn.includes (name))
                this.connections[conn] = true;
    }


    private request(
        obj: { [key: string]: Boolean }, 
        key1: string, 
        key2: string
    ): [{ [key: string]: Boolean }, string] {
        const key = `${key1}->${key2}`;
        const rev_key = `${key2}->${key1}`;
        
        if (key in obj) {
            return [obj, "already_requested"];
        }
    
        if (rev_key in obj) {
            this.connections[`${key1}/${key2}`] = this.new_conn_state;
            delete obj[rev_key];
            return [obj, "accepted"];
        }
    
        else {
            obj[key] = true;
            return [obj, "pending"];
        }
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