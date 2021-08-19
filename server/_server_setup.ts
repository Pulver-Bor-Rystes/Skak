import express, { Application, Request, Response, NextFunction } from 'express'
import { readFileSync } from 'fs'
import { Server } from 'socket.io'
import { __hotreload_init__ } from './_hotreload'


type init_return = {
	app: Application
	io: Server
}


interface Meta {
	url: string
	title: string
	description: string
	lang: string
}

declare module 'express' {
	export interface Request {
		meta?: Meta // Tilføjer en custom type til Request
	}
}

export function init_server() {
	const env = JSON.parse(
		readFileSync(process.cwd() + '/config/env.json', 'utf8')
	)
	process.env = { ...process.env, ...env }

	const port: number = Number(process.env.port)

	const app: Application = express()
	const server = app.listen(port)
	const io: Server = new Server(server, { serveClient: true })
	if (Boolean(env.enable_hotreload))
		__hotreload_init__(io, [
			'web/compiled_public',
			'web/views',
			'web/public'
		])

	// View engine
	app.set('views', 'web/views')
	app.set('view engine', 'ejs')

	// Public
	app.use(
		'/socket.io.js',
		express.static('node_modules/socket.io/client-dist/socket.io.min.js')
	)
	app.use('/', express.static('web/compiled_public'))
	app.use('/', express.static('web/public/'))
	app.use('/global.css', express.static('public/global.css'))

	app.get('*', (req: Request, _res: Response, next: NextFunction) => {
		req.meta = env.meta
		next()
	})

	let stuff: init_return = {
		app,
		io
	}

	return stuff
}
