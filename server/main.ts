console.clear()
import { Socket } from 'socket.io'
import { Request, Response, NextFunction } from 'express'
import { init_server } from './setup/server_setup'
import { authenticate } from './routes/v1/auth.router'
import User from './models/user.model'
import { friends_socket, friends_sync } from './socket/friends'
import { disconnect_event_active_users, emit_to, init_active_users, join_event_active_users } from './socket/active_users'
import { disconnect_event_lobby, join_event_lobby, lobby_socket } from './socket/lobby'
import { disconnect_event_games, games_socket, join_event_games } from './socket/games'
// import { test } from "../../CSM/dist/index"

// test()


const { app, io } = init_server(() => {
	friends_sync();
});
init_active_users(io);


type PlayerData = {
	username: string;
	socket_id: string;
	invited_by_players: string[];
}
const lobby: PlayerData[] = [];

export type Game = {
	id: string
	white: string
	black: string
	turn: Boolean
	socket_ids: string[]
	pgn: string
}


const games: Game[] = [];


// make a function that updates the clients lobby
function update_lobby_on_clients(medium: any) {
	// send the lobby to the client without the invited_by_players
	medium.emit('update_lobby', lobby.map(player => ({
		username: player.username,
		socket_id: player.socket_id,
	})));
}


// Socket forbindelser
io.on('connection', (socket: Socket) => {
	const sid: string = socket.id
	let username: string = "";

	
	// handle login event
	socket.on('login', async (login_username: string, temporary_cookie: string) => {
		let [resp, user] = await authenticate(login_username, temporary_cookie);


		if (resp) {
			username = (user as User).username;
			join_event_active_users(username, sid);
			join_event_lobby(sid, username);
			join_event_games(username, sid);
			emit_to(sid, 'login_success');

			// burde gerne opdaterer sid i spillet
			for (const game of games) {
				let found = false;
				if (game.white == username) {
					game.socket_ids[0] = sid;
					found = true;
				}

				if (game.black == username) {
					game.socket_ids[1] = sid;
					found = true;
				}

				if (found) {
					emit_to(sid, "current_game", game.id);

					break;
				}
			}

			lobby_socket(socket, username);
			friends_socket(socket, username);
			games_socket(socket, username);

			socket.on("move", ({new_move, pgn}) => {
				// get game with the same id as the socket id
				const game = games.find(game => game.socket_ids.includes(sid));
				if (game) {
					// make move on board
					let valid_move = false // TODO: check if move is valid

					if (sid == game.socket_ids[Number(game.turn)]) {
						// if it is the turn of the player, make the move
						valid_move = true; // TODO: Er det et gyldigt træk
					}

					if (valid_move) {
						game.turn = !game.turn;
						game.pgn = pgn;
						// send move to both players
						io.to(game.socket_ids[0]).emit('move_made', game.socket_ids[0] == sid ? "opponent_move" : new_move);
						io.to(game.socket_ids[1]).emit('move_made', game.socket_ids[1] == sid ? "opponent_move" : new_move);
					}
					else {
						// tell player that move was invalid
						emit_to(sid, 'move_invalid', game.pgn);
					}
				}
			
			})

			socket.on("get_game_state", (game_id: string) => {
				const game = games.find(game => game.id == game_id);
				if (!game) {
					return;
				}

				emit_to(sid, "game_created", game);
			})
		}
		else {
			emit_to(sid, 'login_failure');
			return;
		}

		
		// handle disconnect
		socket.on('disconnect', () => {
			disconnect_event_active_users(username, sid);
			disconnect_event_lobby(username);
			disconnect_event_games(username);


			// if player is in lobby
			if (lobby.find(player => player.socket_id === sid)) {
				
				// remove username from lobby
				lobby.splice(lobby.findIndex(player => player.username === username), 1);
				
				update_lobby_on_clients(io)
			}

			// if player is in game
			// remove game

			setTimeout(() => {
				// check if user has been active
				// finder spilleren, og tjekker om sid er ens
				// hvis ja, så har brugeren ikke været aktiv = slet spil
				// hvis nej, gør ingenting
				let active_game = games.find(game => game.socket_ids.includes(sid));
				
				// remove from game
				if (active_game) {
					console.log("removed active game")
					games.splice(games.findIndex(game => game.socket_ids.includes(sid)), 1);
				}
			}, 100 * 60 * 2)
		});
		
		// * OUTDATED
		// handle join lobby event for socket
		socket.on('join', () => {
			// cant join if not logged in
			if (username === "") {
				emit_to(sid, 'join_failure', "You are not logged in");
				return;
			}
	
			// cant join if already in lobby
			if (lobby.find(player => player.username === username)) {
				update_lobby_on_clients(socket)
				emit_to(sid, 'join_failure', "You are already in the lobby");
				return;
			}

			// cant join if already in a game
			if (games.find(game => game.white === username || game.black === username)) {
				emit_to(sid, 'join_failure', "You are already in a game");
				return;
			}
			
			// add username to lobby
			lobby.push({ username, socket_id: sid, invited_by_players: [] });


			update_lobby_on_clients(io)


			socket.on("invite", (invitee: string) => {
				// get sid of invitee
				const invitee_sid = lobby.find(player => player.username === invitee)?.socket_id;
				if (invitee_sid) {
					// if invitee has already invited you, accept the invite
					if (lobby.find(player => player.username === username)?.invited_by_players.includes(invitee)) {
						io.to(invitee_sid).emit('invite_accepted', username);
						emit_to(username, 'invite_accepted', invitee);


						setTimeout(async () => {
							// make new game with an unique id
							const game: Game = {
								id: Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15),
								white: username,
								black: invitee,
								turn: false, // false = white, true = black
								socket_ids: [sid, invitee_sid],
								pgn: "",
							}
	
							// add game to games
							games.push(game);
	
							// send game to both players
							emit_to(username, 'game_created', game);
							emit_to(invitee, 'game_created', game);
						}, 1000)


						// remove both the invitee and the inviter from the lobby
						lobby.splice(lobby.findIndex(player => player.username === invitee), 1);
						lobby.splice(lobby.findIndex(player => player.username === username), 1);

						// uninvite all players that the two players have invited
						lobby.forEach(player => {
							// remove potential invite from player
							if (player.invited_by_players.includes(invitee)) {
								player.invited_by_players.splice(player.invited_by_players.findIndex(inviter => inviter === invitee), 1);
							}
							if (player.invited_by_players.includes(username)) {
								player.invited_by_players.splice(player.invited_by_players.findIndex(inviter => inviter === username), 1);
							}
						});
					}
					else {	
						// send invite to invitee
						io.to(invitee_sid).emit("invite_from", username);
						// add username to invited_by_players if not already invited
						if (!lobby.find(player => player.username === invitee)?.invited_by_players.includes(username)) {
							lobby.find(player => player.username === invitee)?.invited_by_players.push(username);
						}
					}
				}

				update_lobby_on_clients(io);
			})

			socket.on("get_lobby", () => {
				update_lobby_on_clients(socket)
			})
		})
	})



})


// Basic request
app.get('/', (req: Request, res: Response) => {
	res.redirect('/test/oversigt')
})



app.get('/test/:component', (req: Request, res: Response) => {
	let parameters = { 'component': req.params.component.replace(/>/g, '/') }
	Object.assign(parameters, req.meta)

	res.render('test_components', parameters)
})

