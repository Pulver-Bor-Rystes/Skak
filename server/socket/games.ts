import { Game, GameID, Username } from "../../shared/types";
import { Server, Socket } from "socket.io";
import { ActiveUsers, emit_to, is_player_online, Responder, State3 } from "./active_users";
import { check_if_it_already_exists, _request } from "./friends";
import { Chess } from "../../CSM/dist/src/chess"

let invitations: { [key: string]: Boolean } = {}
// let games: any = {}
var games: { [key: string]: Game; } = {};


let users_in_games: { [key: Username]: GameID } = {}

let state = new State3 ("games", {
    subscribed: [],
    state: "",
})


export class Games {
    

    static init () {

        ActiveUsers.subscribe_to_join_event ((sid, username) => {
            
            
        });
        

        ActiveUsers.subscribe_to_disconnect_event ((sid, username, offline) => {
            

        });
    }
    
    
    static route (route: string, socket: Socket, username: Username) {
        let portal = new Responder (socket, route);

        portal
            .on ("_invite", (player_name, answer) => {
                if (!player_name) {
                    return;
                }
        
                if (check_if_it_already_exists(games, [player_name, username])) {
                    answer (username, "already_in_game");
                    return;
                }
        
                let res: string;
                [invitations, res] = _request(invitations, username, player_name);
        
                switch (res) {
                    case "accepted":
                        send_state([username, player_name], [username, player_name]);
                        game_new(username, player_name);
                        break;
        
                    case "pending":
                        send_state([username, player_name], [username, player_name]);
                        break;
        
                    default:
                        break;
                }
            })
            .on ("invite", (target, answer) => {
                if (!target) return;

                let res = state.handle_interaction (username, target);

                if (res == "accepted") {
                    game_new (target, username);
                }
            })

            .on ("get", () => {
                send_state (username, username);
            })

            .on ("move", ( { game_id, move }: { game_id: string, move: string }, answer, fail ) => {
                // Først tjek om game_id'et overhovedet passer
                if (!(game_id in games)) {
                    fail (username, "spillet findes ikke")
                    return;
                }

                const game = games[game_id];
                
                // Derefter tjek om spilleren er en af dem der spiller.
                if (game.subscribed[0] != username && game.subscribed[1] != username) {
                    fail (username, "du er ikke med i spillet")
                    return;
                }
                

                let board = new Chess();
                let player_color = game.subscribed[0] == username ? "WHITE":"BLACK";
                
                if (board.turn != player_color) {
                    fail (username, "det er ikke din tur")
                    return;
                }

                const old_state = game.state;

                let moves = board
                    .load_pgn (game.state)
                    .gen ()
                    .moves

                    
                if (moves.includes (move)) {
                    board.move (move);
                    game.state = board.pgn_string;

                    answer (game.subscribed, [game_id, old_state, move])
                }
                else {
                    fail (username, "trækket findes ikke");
                }
            })
    }
}



export function games_socket(socket: Socket, username: string) {
    socket.on("games/state", (game_id: GameID) => {
        if (game_id in games) {
            emit_to(socket.id, "res:games/state", games[game_id]);
        }
    })


    socket.on("games/move", (game_id: string, move, pgn_before: string) => {
        if (game_id in games) {
            let temp_board = new Chess();

            let moves = temp_board
                .load_pgn (pgn_before)
                .gen ()
                .moves

            temp_board.move(move);

            temp_board.log()

            if (moves.includes (move)) {
                emit_to(games[game_id].subscribed, "notif:games/move", [game_id, pgn_before, move])
            }
        }
    })
}


function send_state(send_to: string | string[], username: string | string[]) {
    if (typeof send_to == "string" && typeof username == "string") {
            let gs = ["undefined"];
            let inv = [];
            let send = [];

            // Samler spil
            for (const game of Object.keys(games)) {
                if ([username == games[game].subscribed[0] || username == games[game].subscribed[1]]) {
                    gs[0] = game;
                }
                else {
                    gs.push(game)
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
        
            emit_to(send_to, "notif:games/state", [gs, inv, send]);
    }
    else if (send_to.length == username.length) {
        for (let i=0; i < send_to.length; i++) {
            send_state(send_to[i], username[i])
        }
    }
}


export function game_new(user1: string, user2: string) {
    const game_id = `${user1}&${user2}`;
    
    users_in_games[user1] = game_id;
    users_in_games[user2] = game_id;


    // TODO: Vælg tilfældigt hvem der skal være hvid
    let white = user1;
    let black = user2;

    const NG = {
        // insert game stuff
        subscribed: [white, black],
        state: "",
    }

    games[game_id] = NG;


    emit_to(NG.subscribed, "notif:games/created", NG);
    send_state(NG.subscribed, NG.subscribed);
}


export function is_player_in_game(username: string) {
    return username in users_in_games;
}


// TODO: Find ud af hvilke andre brugerer der er berørt af ændringer fra denne funktion og send dem en opdatering.
export function disconnect_event_games(username: string) {
    if (!is_player_online(username)) {
        let objs = [games, invitations];

        for (const obj of objs) {
            for (const rec of Object.keys(obj)) {
                if (rec.includes(username)) {

                    //!ERR: Der kan ske en fejl her, når man inviterer en person til en spil. Personen gør intet og en af personerne forlader
                    let subs = ("subscribed" in obj[rec]) ? (obj[rec] as Game)["subscribed"] : [];
                    delete obj[rec];
                    send_state(subs, subs);
                }
            }
        }
        
        if (username in users_in_games) {
            delete users_in_games[username];
        }

        // console.log(games, invitations)

        send_state(username, username);
    }
}