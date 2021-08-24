import express, { Application, Request, Response, NextFunction } from 'express'
import { readFileSync } from 'fs'
import { Server } from 'socket.io'
import { hotreload_init } from './hotreload'
import { genereate_files } from './gen_files'

// Database
import { connect_to_db } from './database'
import { info_router } from "../routes/example"
import { users_router } from "../routes/users"

genereate_files()


type init_return = {
	app: Application
	io: Server
}

interface User {
	username: string
	id: string
}


interface Meta {
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

	// View engine
	app.set('views', 'web/views')
	app.set('view engine', 'ejs')

	// Public
	app.use('/', express.static('node_modules/socket.io/client-dist/'))
	app.use('/', express.static('web/compiled'))

	app.get('*', (req: Request, _res: Response, next: NextFunction) => {
		req.meta = env.meta
		next()
	})


	// Database
	connect_to_db()
		.then(() => {
			app.use("/users", info_router);
			app.use("/auth", users_router);
		})
		.catch((error: Error) => {
			console.error("Database connection failed", error);
			process.exit();
		});


	let stuff: init_return = {
		app,
		io
	}

	return stuff
}
