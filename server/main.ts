console.clear()
import { Socket } from 'socket.io'
import { Request, Response, NextFunction } from 'express'
import { init_server } from './setup/server_setup'
import { authenticate } from './routes/v1/auth.router'
import User from './models/user.model'



const { app, io } = init_server()


type PlayerData = {
	username: string;
	socket_id: string;
	invited_by_players: string[];
}
const lobby: PlayerData[] = [];

type Game = {
	id: string
	white: string
	black: string
	turn: Boolean
	socket_ids: string[]
	pgn: string
}


const games: Game[] = [];


// make a function that updates the clients lobby
function update_lobby(medium: any) {
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
	socket.on('login', async (username: string, temporary_cookie: string) => {
		let [resp, user] = await authenticate(username, temporary_cookie);
		
		if (resp) {
			socket.emit('login_success');
			username = (user as User).username;
		}
		else {
			socket.emit('login_failure');
			return;
		}

		
		// handle disconnect
		socket.on('disconnect', () => {
			// remove username from lobby
			lobby.splice(lobby.findIndex(player => player.username === username), 1);

			// remove potential game
			games.splice(games.findIndex(game => game.white === username || game.black === username), 1);

			update_lobby(io)
		});
		
		
		// handle join event for socket
		socket.on('join', () => {
			// cant join if not logged in
			if (username === "") {
				socket.emit('join_failure');
				return;
			}
	
			// cant join if already in lobby
			if (lobby.find(player => player.username === username)) {
				update_lobby(socket)
				socket.emit('join_failure');
				return;
			}

			// cant join if already in a game
			if (games.find(game => game.white === username || game.black === username)) {
				socket.emit('join_failure');
				return;
			}
			
			// add username to lobby
			lobby.push({ username, socket_id: sid, invited_by_players: [] });


			update_lobby(io)


			socket.on("invite", (invitee: string) => {
				// get sid of invitee
				const invitee_sid = lobby.find(player => player.username === invitee)?.socket_id;
				if (invitee_sid) {
					// if invitee has already invited you, accept the invite
					if (lobby.find(player => player.username === invitee)?.invited_by_players.includes(username)) {
						io.to(invitee_sid).emit('invite_accepted', username);
						socket.emit('invite_accepted', invitee);

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
							io.to(sid).emit('game_created', game);
							io.to(invitee_sid).emit('game_created', game);
						}, 1000)


						// remove both the invitee and the inviter from the lobby
						lobby.splice(lobby.findIndex(player => player.username === invitee), 1);
						lobby.splice(lobby.findIndex(player => player.username === username), 1);
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
			})

			socket.on("move", (move: string) => {
				// get game with the same id as the socket id
				const game = games.find(game => game.socket_ids.includes(sid));
				if (game) {
					// make move on board
					let valid_move = false // TODO: check if move is valid

					if (sid == game.socket_ids[Number(game.turn)]) {
						// if it is the turn of the player, make the move
						valid_move = true;
					}

					if (valid_move) {
						game.turn = !game.turn;
						console.log("move made", move)
						// send move to both players
						io.to(game.socket_ids[0]).emit('move_made', game.socket_ids[0] == sid ? "opponent_move" : move);
						io.to(game.socket_ids[1]).emit('move_made', game.socket_ids[1] == sid ? "opponent_move" : move);
					}
					else {
						// tell player that move was invalid
						socket.emit('move_invalid');
					}
				}
			
			})

			socket.on("get_lobby", () => {
				update_lobby(socket)
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

