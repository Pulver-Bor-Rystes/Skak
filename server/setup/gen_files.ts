// Live reload
import { accessSync, writeFileSync } from 'fs'


const files: any = {
    'config/env.json': {
        "port": 4000,
        "enable_hotreload": true,
        "meta": {
            "title": "Skakk",
            "description": "Beskrivelse",
            "url": "localhost:4000",
            "language": "dk",
        },
        "database": {
            "uri": "mongodb://localhost:27017",
            "password": "",
            "name": "skakk",
            "collections": ["users"],
        }
    }
}


export function genereate_files(): void {
    const keys = Object.keys(files)

    keys.forEach(file => {
        let content: any = files[file]
        let type = typeof content

        try {
            accessSync(file)
        } catch (err) {
            if (type != 'string')
                content = JSON.stringify(content, null, 4)

            writeFileSync(file, content)
        }
    })
}