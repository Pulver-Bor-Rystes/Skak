const { Board } = require("./dist/index")

let b = new Board;
b.log()


const square_letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']

const squares = []


for (let y = 8; y > 0; y--) {
    for (const sl of square_letters) {
        const square = `${sl}${y}`;
        squares.push(square)
    }
}


// for (const sl of square_letters) {
//     for (let y = 8; y > 0; y--) {
//         const square = `${sl}${y}`;
//         squares.push(square)
//     }
// }

process.stdout.write(`type Square = '${squares.join("' | '")}'\n`)
process.stdout.write(`const squares: Square[] = ['${squares.join("', '")}']\n`)