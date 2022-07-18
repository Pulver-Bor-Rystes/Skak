import express, { Request, Response } from "express"
import fs from "fs"
import path from "path"

export const testing_router = express.Router()
testing_router.use(express.json())


testing_router.post('/svelte_files', (req: Request, res: Response) => {
	let files = get_all_files('./web/js/svelte')

	res
		.status(200)
		.send(files)
})


const get_all_files = function (dirPath: string, arrayOfFiles?: string[]) {
	let files = fs.readdirSync(dirPath)

	arrayOfFiles = arrayOfFiles || []

	// correct backslash
	let cb = process.platform == 'win32' ? '\\':'/'

	files.forEach(function (file) {
		if (fs.statSync(dirPath + "/" + file).isDirectory()) {
			arrayOfFiles = get_all_files(dirPath + "/" + file, arrayOfFiles)
		} else {
			if (file.includes('.svelte'))
				arrayOfFiles?.push( (path.join(dirPath, "/", file)).split(`${cb}js${cb}svelte${cb}`)[1] )
		}
	})

	return arrayOfFiles
}