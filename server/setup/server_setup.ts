import express, { Application, Request, Response, NextFunction } from 'express'
import cookieParser from 'cookie-parser'
import { readFileSync } from 'fs'
import { Server } from 'socket.io'
import { hotreload_init } from './hotreload'
import { genereate_files } from './gen_files'

// Database
import { connect_to_db } from './database'
import { info_router } from "../routes/example.router"
import { auth_router, background_auth_router } from "../routes/auth.router"
import { testing_router } from "../routes/testing.router"

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



export function init_server(): init_return {
	const env = JSON.parse(
		readFileSync(process.cwd() + '/config/env.json', 'utf8')
	)
	process.env = { ...process.env, ...env }

	const port: number = Number(process.env.port)

	const app: Application = express()
	const server = app.listen(port)
	const io: Server = new Server(server, { serveClient: true })
	if (Boolean(env.enable_hotreload))
		hotreload_init(io, [
			'web/compiled',
			'web/compiled/js',
			'web/compiled/css',
			'web/compiled/images',
		])

	// Parsers
	app.use(cookieParser());

	// View engine
	app.set('views', 'web/views')
	app.set('view engine', 'ejs')



	// Background authentication
	app.use(background_auth_router)

	// Meta data til siderne
	app.get('*', (req: Request, _res: Response, next: NextFunction) => {
		req.meta = env.meta
		next()
	})

	// Public
	app.use('/', express.static('node_modules/socket.io/client-dist/'))
	app.use('/', express.static('web/compiled'))



	// Database
	connect_to_db()
		.catch((error: Error) => {
			console.error("Database connection failed", error);
			process.exit();
		});

	app.use("/users", info_router);
	app.use("/auth", auth_router);
	app.use("/testing", testing_router);


	let stuff: init_return = {
		app,
		io
	}

	return stuff
}
