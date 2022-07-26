import { Server, Socket } from "socket.io";
import { ActiveUsers, emit_to, is_player_online, Responder } from '../socket/active_users';
import { collections } from "../setup/database";
import { Username } from "../../shared/types";


let invitations: { [key: string]: Boolean } = {}
let friendships: { [key: string]: Boolean } = {}



export class Friends {
    static init() {
        ActiveUsers.subscribe_to_join_event ((sid, username) => {
            let my_friends = Friends.get_friends (username);

            for (let friend of my_friends) {
                Friends.who_of_my_friends_are_online (friend, friend);
            }
        });

        ActiveUsers.subscribe_to_disconnect_event ((sid, username, offline) => {
            if (offline) {
                let my_friends = Friends.get_friends (username);
    
                for (let friend of my_friends) {
                    Friends.who_of_my_friends_are_online (friend, friend);
                }
            }
        });
    }



    static who_of_my_friends_are_online(target_user: string, about_user: Username) {
        let online = [];
        let offline = [];

        for (const friendship of Object.keys (friendships)) {
            if (!friendship.includes (about_user)) continue;

            const partners = friendship.split("&");
            const friend = partners[0] == about_user ? partners[1]:partners[0];

            if (is_player_online (friend)) {
                online.push (friend);
            }
            else {
                offline.push (friend);
            }
        }


        
        online.sort ((a, b) => a.localeCompare (b));
        offline.sort ((a, b) => a.localeCompare (b));
        let combined = online.concat (offline).sort ((a, b) => a.localeCompare (b));

        ActiveUsers.emit_to (target_user, "res:friends/get", {
            combined,
            online,
            offline,
        });
    }


    static route (route: string, socket: Socket, username: Username) {
        let portal = new Responder (socket, route);

        portal
            .on ("request", (player_name, answer, fail) => {
                // Stop hvis der prøves at invitere sig selv
                if (username == player_name) return;

                // Stop hvis venskabet allerede eksisterer
                if (check_if_it_already_exists(friendships, [player_name, username])) {
                    fail (socket.id, "already_friends");
                    return;
                }
        
                let res: string;
                [invitations, res] = _request(invitations, username, player_name);
                
                if (res == "accepted") {
                    friendships[`${username}&${player_name}`] = true;
                    add_friendship (username, player_name);
                    remove_friend_request (username, player_name);
        
                    // den første bruges som notifikation at noget er sket, og den anden bliver brugt til at fortælle klienten præcis hvad der sker
                    answer ([username, player_name], [res, [username, player_name]])
                    send_state ([username, player_name]); // TODO: Find på noget bedre...
                }
        
                if (res == "pending") {
                    add_friend_request(username, player_name);
                    answer ([username, player_name], [res, [username, player_name]]);
                    send_state ([username, player_name]);
                }
        
                if (res == "already_requested") {
                    answer (username, [res, player_name])
                }
            })
        .on ("get", (_, answer, fail) => {
            Friends.who_of_my_friends_are_online (socket.id, username);
        })
    }



    static get_friends (username: Username) {
        let my_friends = [];
        
        for (let friendship of Object.keys (friendships)) {
            if (friendship.includes (username)) {
                let friends = friendship.split("&")
                let f1 = friends[0]
                let f2 = friends[1]

                my_friends.push (f1 == username ? f2:f1);
            }
        }

        return my_friends;
    }
}


export function friends_socket(socket: Socket, username: string) {
    socket.on("friends/fetch", () => {
        send_state (socket.id, username);


    })



    socket.on ("friends/get", () => {
        
    })
}


export function check_if_it_already_exists(obj: any, names: string[]) {
    for (const instance of Object.keys(obj)) {
        let is_friends = true;
        for (const name of names) {
            if (!instance.includes(name)) {
                is_friends = false;
            }
        }

        if (is_friends) {
            return;
        }
    }

    return false;
}


function send_state(send_to: string | string[], username?: string | string[]) {
    if (!username) {
        username = send_to;
    }

    if (typeof send_to == "string" && typeof username == "string") {
            let friends = [];
            let inv = [];
            let send = [];
            
        
            // Samler alle venskaber
            for (const friendship of Object.keys(friendships)) {
                if (friendship.includes(username)) {
                    const frs = friendship.split("&");
                    const fr = frs[0] == username ? frs[1]:frs[0];
                    
                    friends.push(fr)
                }
            }
        
            // Samler anmodninger
            for (const invis of Object.keys(invitations)) {
                const invi = invis.split("->");
                const from = invi[0];
                const to = invi[1];
        
                if (from == username) {
                    send.push(to);
                }
                if (to == username) {
                    inv.push(from);
                }
            }

            ActiveUsers.emit_to(send_to, "res:friends/fetch", [friends, inv, send]);
    }
    else if (send_to.length == username.length) {
        for (let i=0; i < send_to.length; i++) {
            send_state(send_to[i], username[i])
        }
    }
}

/** Kan returnerer tre ting:

accepted, pending, already_requested
 */
export function _request(
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
        delete obj[rev_key];
        return [obj, "accepted"];
    }

    else {
        obj[key] = true;
        return [obj, "pending"];
    }
}




function add_friend_request(from: string, to: string) {
    collections.users?.updateOne({ username: to }, { $push: { friend_requests: from } })
}


function add_friendship(from: string, to: string) {
    collections.users?.updateOne({ username: from }, { $push: { friends: to } })
    collections.users?.updateOne({ username: to }, { $push: { friends: from } })
}


function remove_friend_request(from: string, to: string) {
    collections.users?.updateOne(
        { username: from },
        { $pull:
            { friend_requests:
                { $in: [to] }
            }
        }
    )
}


// TODO: Kan gøres på en bedre måde...
function remove_friendship(from: string, to: string) {
    collections.users?.updateOne(
        { username: from },
        { $pull:
            { friends:
                { $in: [to] }
            }
        }
    )

    collections.users?.updateOne(
        { username: to },
        { $pull:
            { friends:
                { $in: [from] }
            }
        }
    )
}



/** Henter data fra serveren */
export async function friends_sync() {
    if (JSON.parse(JSON.stringify(process.env.debug) || "{}")?.reset_friend_stuff) {
        console.log("resetting friend stuff...");

        reset_friend_requests();
        reset_friendships();
    }


    let cursor = await collections.users?.find().toArray()

    cursor?.forEach((user) => {
        let fr = user.friends
        let inv = user.friend_requests

        fr.forEach((friend_name: string) => {
            if (!(`${friend_name}&${user.username}` in friendships)) {
                friendships[`${user.username}&${friend_name}`] = true;
            }
        })

        inv.forEach((friend_name: string) => {
            invitations[`${friend_name}->${user.username}`] = true;
        })
    })
}


/**  Nulstiller friend_requests */
export function reset_friend_requests() {
    collections.users?.updateMany(
        { username: { $ne: "" } },
        { $set: { friend_requests: [] } }
    )
}

/**  Nulstiller friendships */
export function reset_friendships() {
    collections.users?.updateMany(
        { username: { $ne: "" } },
        { $set: { friends: [] } }
    )
}