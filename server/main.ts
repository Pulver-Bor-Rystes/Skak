console.clear()
import { Socket } from 'socket.io'
import { Request, Response, NextFunction } from 'express'
import { init_server } from './setup/server_setup'
import { authenticate } from './routes/v1/auth.router'
import User from './models/user.model'
import { Friends, friends_socket, friends_sync } from './socket/friends'
import { ActiveUsers, emit_to } from './socket/active_users'
import { lobby_socket } from './socket/lobby'
import { Games, games_socket } from './socket/games'

// test()


const { app, io } = init_server(() => {
	friends_sync();
});

ActiveUsers.init (io);
Friends.init ();


type PlayerData = {
	username: string;
	socket_id: string;
	invited_by_players: string[];
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
			ActiveUsers.join_event (sid, username);
			ActiveUsers.emit_to(sid, 'login_success');


			ActiveUsers.route ("au", socket, username);
			Games.route ("games", socket, username);
			Friends.route ("friends", socket, username);

			
			lobby_socket(socket, username);
			friends_socket(socket, username);
			games_socket(socket, username);
		}
		else {
			emit_to(sid, 'login_failure');
			return;
		}

		
		// handle disconnect
		socket.on('disconnect', () => {
			ActiveUsers.disconnect_event (sid, username);

		});	
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

