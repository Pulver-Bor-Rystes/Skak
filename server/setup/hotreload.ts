// Live reload
import { watch } from 'fs'
import { Server } from 'socket.io'

let _io: Server

export function hotreload_init(io: Server, paths: string[]) {
	_io = io
	watch_for_changes(paths)
}

function watch_for_changes(folders: string[]) {
	for (const folder of folders) {
		watch(
			`${process.cwd()}/${folder}`,
			{ encoding: 'utf8' },
			(_event_type: string, filename: string) => {
				if (_io) _io.emit('__hotreload', filename)
			}
		)
	}
}
