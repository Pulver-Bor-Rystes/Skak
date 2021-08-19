const { exec } = require("child_process");
const { argv } = process 

const commands = ['svelte', 'src', 'style']
const all = (argv.length == 2)


for (const command of commands) {
    if (argv.includes(command) || all)
        execute(command)
}


function execute(command) {
    console.log('executing:', command)
    if (argv.includes('src') || all) {
        exec(`yarn ${ command }`, (err, stdout, stderr) => {
            if (err) throw err
            console.log(stdout)
            console.log(stderr)
        })
    }
}