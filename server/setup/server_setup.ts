import express, { Application, Request, Response, NextFunction } from 'express'
import cookieParser from 'cookie-parser'
import cors from 'cors'
import { readFileSync } from 'fs'
import { Server } from 'socket.io'
import { hotreload_init } from './hotreload'
import { genereate_files } from './gen_files'

// Database
import { connect_to_db } from './database'

// Routers
import { auth_router, background_auth_router } from "../routes/v1/auth.router"
import { info_router } from "../routes/legacy/example.router"
import { testing_router } from "../routes/v1/testing.router"
import { users_router } from '../routes/v1/users.router'
import { user_info } from '../routes/v1/user_info.router'

// Interfaces / Types
import User from '../models/user.model'


genereate_files()


type init_return = {
	app: Application
	io: Server
}




export interface Meta {
	url: string
	title: string
	description: string
	lang: string
}

declare module 'express' {
	export interface Request {
		meta?: Meta, // Tilføjer en custom type til Request
		user?: User,
	}
}



export function init_server(db_ready_callback: Function): init_return {
	const env = JSON.parse(
		readFileSync(process.cwd() + '/config/env.json', 'utf8')
	)
	process.env = { ...process.env, ...env }

	const port: number = Number(process.env.port)

	const app: Application = express()
	
	// Parsers
	app.use(cookieParser());
	app.use(cors());


	const server = app.listen(port)
	const io: Server = new Server(server, { serveClient: true })
	if (Boolean(env.enable_hotreload))
		hotreload_init(io, [
			// 'web/compiled',
			// 'web/compiled/js',
			// 'web/compiled/css',
			// 'web/compiled/images',
		])


	// View engine
	// app.set('views', 'web/views')
	// app.set('view engine', 'ejs')



	// Background authentication
	app.use(background_auth_router)

	// Meta data til siderne
	app.get('*', (req: Request, _res: Response, next: NextFunction) => {
		req.meta = env.meta
		next()
	})

	// Public
	app.use('/api/', express.static('node_modules/socket.io/client-dist/'))
	// app.use('/', express.static('web/compiled'))


	process.stdout.write("Forbinder til database...")


	// Database
	connect_to_db()
		.catch((error: Error) => {
			process.stdout.write("❌\n")
			process.exit();
		})
		.then(() => {
			process.stdout.write("✅\n")
			if (db_ready_callback) {
				db_ready_callback();
			}
		});



	// Routes
	const general_api_router = express.Router().use(express.json());

	// Versioning
	const v1 = express.Router().use(express.json());
	const legacy = express.Router().use(express.json());
	
	app.use("/api", general_api_router)
	general_api_router.use("/v1", v1)
	general_api_router.use("/legacy", legacy)



	legacy.use("/users", info_router);
	v1.use("/auth", auth_router);
	v1.use("/testing", testing_router);
	v1.use("/me", user_info);
	v1.use("/users", users_router);


	let stuff: init_return = {
		app,
		io
	}

	return stuff
}
