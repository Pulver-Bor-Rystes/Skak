console.clear()
import { Socket } from 'socket.io'
import { Request, Response, NextFunction } from 'express'
import { init_server } from './setup/server_setup'

const { app, io } = init_server()


// Socket forbindelser
io.on('connection', (socket: Socket) => {
	const sid: string = socket.id
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