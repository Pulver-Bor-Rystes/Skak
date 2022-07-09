

console.clear()





// print et 12x12 bræt og vis hvor et index er på brættet
let sel = 100
let target = 87;
let i = 0;
let y = 0;

const x12_valid_indexes = [26, 27, 28, 29, 30, 31, 32, 33, 38, 39, 40, 41, 42, 43, 44, 45, 50, 51, 52, 53, 54, 55, 56, 57, 62, 63, 64, 65, 66, 67, 68, 69, 74, 75, 76, 77, 78, 79, 80, 81, 86, 87, 88, 89, 90, 91, 92, 93, 98, 99, 100, 101, 102, 103, 104, 105, 110, 111, 112, 113, 114, 115, 116, 117];
let offsets: number[] = [ -11, -13, -12, -24 ]


while (y < 12) {
    let x = 0;
    while (x < 12) {
        let inc = false

        for (let OFF of offsets) {
            let pos = sel + OFF;
            

            if (i == pos) {
                if (x12_valid_indexes.includes(i)) {
                    if (target == i) {
                        process.stdout.write(` +`);
                    }
                    else {
                        process.stdout.write(` x`);
                    }
                }
                else {
                    if (target == i) {
                        process.stdout.write(` -`);
                    }
                    else {
                        process.stdout.write(` 0`);
                    }
                }
                inc = true;
            }
        }

        if (sel == i) {
            process.stdout.write(` P`);
        }
        else if (inc) {
            // process.stdout.write(` x`);
            
        }
        else if (x >= 2 && x <= 9 && y >= 2 && y <= 9) {
            process.stdout.write(` .`);
        }
        else {
            process.stdout.write(`  `);
        }
        x ++;
        i ++;
    }

    y ++;

    process.stdout.write("\n");
}


process.stdout.write("\n");
























// const { Board } = require("./dist/index")

// let b = new Board;
// b.log()


// const square_letters = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']

// const squares = []


// for (let y = 8; y > 0; y--) {
//     for (const sl of square_letters) {
//         const square = `${sl}${y}`;
//         squares.push(square)
//     }
// }


// // for (const sl of square_letters) {
// //     for (let y = 8; y > 0; y--) {
// //         const square = `${sl}${y}`;
// //         squares.push(square)
// //     }
// // }

// process.stdout.write(`type Square = '${squares.join("' | '")}'\n`)
// process.stdout.write(`const squares: Square[] = ['${squares.join("', '")}']\n`)