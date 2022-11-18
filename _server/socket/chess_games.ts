import { GameID, Username } from "../../shared/types";
import { Server, Socket } from "socket.io";
import { ActiveUsers, emit_to, is_player_online, Responder, State3 } from "./active_users";
import { check_if_it_already_exists, _request } from "./friends";
import { Chess } from "../../CSM/dist/src/chess"

// Vi skal holde styr på spil som allerede er i gang + invitationer til spil

let active_games: Game[] = [];

type GameType = "Invited" | "Playing" | "Done"

class Game {
	subscribed: Username[] // hvor første er hvid, og derefter...
	game_type: GameType = "Invited"

	constructor (player1: Username, player2: Username) {
		if (Math.round(Math.random()) == 1) {
			this.subscribed = [player1, player2]
		}
		else {
			this.subscribed = [player2, player1]
		}
	}

	is_player_playing (player: Username) {
		if (player == this.subscribed[0] || player == this.subscribed[1]) {
			return true;
		}

		return false;
	}



	static get_if_exists (player1: Username, player2: Username) {
		active_games = active_games.filter((G) => {
			if (G.subscribed[0] == player1 && G.subscribed[1] == player2) {
				return true;
			}
			else (G.subscribed[0] == player2 && G.subscribed[1] == player1) {
				return true;
			}
			return false;
		})

		console.log(active_games)
	}
}


export class GameAPI {
	active_games: Game[] = []

    static init () {
        ActiveUsers.subscribe_to_join_event ((sid, username) => {
            
            
        });
        

        ActiveUsers.subscribe_to_disconnect_event ((sid, username, offline) => {
            

        });
    }
    
    
    static route (route: string, socket: Socket, username: Username) {
        let portal = new Responder (socket, route);

        portal
            .on ("invite", (target, answer) => {
                Game.get_if_exists(username, target)
            })

            // Klienten skal kunne hente et spils 'state'
            // Det kan gøres på forskellige måder
            // 1. Der skal hentes ét spil
            //     Spil opdateres forskelligt, og de bør derfor opdateres forskelligt.
            // 2. Hent alle spil, 

            .on("get_game", ({ gid }, answer, fail) => {
                
            })
    }
}