export function test(): void {
    console.log("test... it works!")
}

type Square = 'A8' | 'B8' | 'C8' | 'D8' | 'E8' | 'F8' | 'G8' | 'H8' | 'A7' | 'B7' | 'C7' | 'D7' | 'E7' | 'F7' | 'G7' | 'H7' | 'A6' | 'B6' | 'C6' | 'D6' | 'E6' | 'F6' | 'G6' | 'H6' | 'A5' | 'B5' | 'C5' | 'D5' | 'E5' | 'F5' | 'G5' | 'H5' | 'A4' | 'B4' | 'C4' | 'D4' | 'E4' | 'F4' | 'G4' | 'H4' | 'A3' | 'B3' | 'C3' | 'D3' | 'E3' | 'F3' | 'G3' | 'H3' | 'A2' | 'B2' | 'C2' | 'D2' | 'E2' | 'F2' | 'G2' | 'H2' | 'A1' | 'B1' | 'C1' | 'D1' | 'E1' | 'F1' | 'G1' | 'H1'
const squares: Square[] = ['A8', 'B8', 'C8', 'D8', 'E8', 'F8', 'G8', 'H8', 'A7', 'B7', 'C7', 'D7', 'E7', 'F7', 'G7', 'H7', 'A6', 'B6', 'C6', 'D6', 'E6', 'F6', 'G6', 'H6', 'A5', 'B5', 'C5', 'D5', 'E5', 'F5', 'G5', 'H5', 'A4', 'B4', 'C4', 'D4', 'E4', 'F4', 'G4', 'H4', 'A3', 'B3', 'C3', 'D3', 'E3', 'F3', 'G3', 'H3', 'A2', 'B2', 'C2', 'D2', 'E2', 'F2', 'G2', 'H2', 'A1', 'B1', 'C1', 'D1', 'E1', 'F1', 'G1', 'H1']

type Square_Letter = 'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H';
const square_letters: Square_Letter[] = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H']

type Color = 'LIGHT' | 'DARK' | 'EMPTY';
const DEFAULT_COLOR: Color = 'EMPTY';

type Player = 'WHITE' | 'BLACK';

type Piece = 'PAWN' | 'KNIGHT' | 'BISHOP' | 'ROOK' | 'QUEEN' | 'KING' | 'EMPTY';
const DEFAULT_PIECE = 'EMPTY';


type Behaviour = 'SLIDE' | "NOTHING"


interface Move {
    from: Square
    to: Square
}

interface SquareData {
    index: number
    square: Square
    piece: Piece
    color: Color
    behaviour: Behaviour
}


function default_x(): Color[] | Piece[] {
    const colors: Color[] = [];
    for (let i = 0; i < 64; i++) {
        colors.push("EMPTY");
    }
    return colors;
}




const UP = -8;
const DOWN = 8;
const RIGHT = 1;
const LEFT = -1;

const UR = -7;
const UL = -9;
const DR = 9;
const DL = 7;






export class Board {
    colors = default_x() as Color[];
    pieces = default_x() as Piece[];

    turn: Color = "LIGHT";
    number_of_moves: number = 0;

    moves: Move[] = [];


    constructor(fen: string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR") {
        this.load_fen(fen)
    }

    private gen_sliding_number_of_moves(moves: Move[], SD: SquareData, limit?: boolean): Move[] {
        let directions_rook = [UP, DOWN, RIGHT, LEFT];
        let directions_bishop = [UR, UL, DR, DL];
        let directions_queen = directions_rook.concat(directions_bishop);

        let dirs: number[] = []

        switch (SD.piece) {
            case "ROOK":
                dirs = dirs.concat(directions_rook);
                break;
            case "BISHOP":
                dirs = dirs.concat(directions_bishop);
                break;
            default:
                dirs = directions_queen;
                break;
        }


        for (const dir of dirs) {
            // gå et skridt i den retning
            // tilføj det træk, hvis farven er tom eller modsat.
            // stop hvis kanten når brikken når kanten af brættet.


            
            let keep_going = true;
            let index = SD.index;

            while (keep_going) {
                index = index + dir;
                if (index < 0 || index > 63) {
                    keep_going = false;
                    break;
                }



                const NEW_SD = this.get_square_data(index)
                
                if (NEW_SD.color != SD.color) {
                    if (NEW_SD.color != "EMPTY") {
                        keep_going = false;
                    }
                    // good to go :))
                    
                    // Nu mangler jeg bare at tjekke om den har ændret enten 
                    if (SD.piece == "ROOK" || SD.piece == "QUEEN") {
                        if ([UP, DOWN].includes(dir)) {
                            if (NEW_SD.square[0] != SD.square[0]) {
                                // Så er den ikke på samme rank og alt skal afbryddes.
                                keep_going = false;
                                break;
                            }
                        }
                        else if ([RIGHT, LEFT].includes(dir)) {
                            if (NEW_SD.square[1] != SD.square[1]) {
                                keep_going = false;
                                break;
                            }
                        }
                    }

                    if (SD.piece == "BISHOP" || SD.piece == "QUEEN") {
                        if ([UR, UL, DR, DL].includes(dir)) {
                            if (NEW_SD.square[0] == SD.square[0]) {
                                keep_going = false;
                                break;
                            }
                            else if (['A', 'H'].includes(NEW_SD.square[0]) || [1, 8].includes(Number(NEW_SD.square[1]))) {
                                keep_going = false;
                            }
                        }
                    }

                    let new_move: Move = {
                        from: SD.square,
                        to: NEW_SD.square
                    }

                    moves.push( new_move )
                }
                else {
                    keep_going = false;
                }

                if (limit) {
                    keep_going = false;
                }
            }
        }
        
        return moves;
    }

    private gen_king_moves(moves: Move[], SD: SquareData): Move[] {
        return this.gen_sliding_number_of_moves(moves, SD, true);
    }

    private gen_pseudo_legal() {
        let moves: Move[] = [];

        for (let piece_id = 0; piece_id < 64; piece_id++) {
            const SD: SquareData = this.get_square_data(piece_id);

            if (SD.color == this.turn) {   
                if (SD.behaviour == "SLIDE") {
                    moves = this.gen_sliding_number_of_moves(moves, SD)
                }
                else if (SD.piece == "KNIGHT") {
                    
                }
                else if (SD.piece == "PAWN") {
                }
                else if (SD.piece == "KING") {
                    this.gen_king_moves(moves, SD);
                }
            }
        }

        this.moves = moves;
    }

    private pick_legal() {

    }


    gen() {
        this.gen_pseudo_legal()
        this.pick_legal()
    }


    load_fen(fen: string) {
        let index = 0;
        
        let parts_space = fen.split(' ');
        if (parts_space[1]) {
            this.turn = parts_space[1].toLowerCase() == "b" ? "DARK":"LIGHT";
        }

        let ranks = parts_space[0].split('/');

        for (const rank of ranks) {
            for (const com of rank.split("")) {
                try {
                    let num = Number(com);
                    if (!num) {
                        throw "not a number";
                    }
                    if (num > 8) {
                        throw "number too high";
                    }

                    index = index + num;
                }
                catch (err) {
                    if (err == "number too high") {
                        throw err;
                    }
                    try {
                        if (com.length != 1) {
                            throw new Error("piece length is not 1!")
                        }
                        else {
                            let color: Color = (com == com.toUpperCase()) ? "LIGHT":"DARK";
                            let piece: Piece;

                            switch (com.toLowerCase()) {
                                case "r":
                                    piece = "ROOK"
                                    break;

                                case "n":
                                    piece = "KNIGHT"
                                    break;

                                case "b":
                                    piece = "BISHOP"
                                    break;

                                case "q":
                                    piece = "QUEEN"
                                    break;

                                case "k":
                                    piece = "KING"
                                    break;

                                case "p":
                                    piece = "PAWN"
                                    break;
                            
                                default:
                                    throw new Error(`unkown piece: "${com}"`);
                            }

                            this.colors[index] = color;
                            this.pieces[index] = piece;


                            index ++;
                        }
                    }
                    catch (err) {
                        throw err;
                    }
                }
            }
        }
    }

    load_pgn(pgn: string) {

    }


    private index_to_square(index: number) {
        return squares[index];
    }


    private is_sliding_piece(index: number): Behaviour {
        return ["QUEEN", "ROOK", "BISHOP"].includes(this.pieces[index]) ? "SLIDE" : "NOTHING";
    }

    private get_square_data(index: number): SquareData {
        let SD: SquareData = {
            "index": index,
            "square": squares[index],
            "piece": this.pieces[index],
            "color": this.colors[index],
            "behaviour": (["QUEEN", "ROOK", "BISHOP"].includes(this.pieces[index])) ? "SLIDE" : "NOTHING",
        }

        return SD;
    }


    log(): void {
        let count = 8;
        let y = 8;
        for (let i = 0; i < 64; i++) {
            if (++count >= 8) {
                if (y == 4) {
                    process.stdout.write(`     turn: ${this.turn}`)
                }
                else if (y == 3) {
                    process.stdout.write(`     number of moves made: ${this.number_of_moves}`)
                }
                else if (y == 2) {
                    process.stdout.write(`     available moves: ${this.moves.length}`)
                }
                
                count = 0;
                process.stdout.write(`\n${y--}|`);
            }
            let color = this.colors[i];
            let piece = this.pieces[i];

            let piece_name: string = piece[0];
            if (piece == "KNIGHT") {
                piece_name = "N";
            }

            let p = '*';

            if (color != "EMPTY" || piece != "EMPTY") {
                p = (color == "DARK") ? piece_name.toLowerCase() : piece_name;
            }

            process.stdout.write(` ${p}`);
        }

        process.stdout.write("\n |----------------\n  ");
        for (const sl of square_letters) {
            process.stdout.write(` ${sl}`)
        }
        process.stdout.write("\n");
    }
}





if (process.argv[2] == "test") {
    // let b = new Board("8885R3QK7BK");
    let b = new Board("rnbqkbnr/ppp1pppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    
    b.gen();
    b.log();

    let last_from = ""

    for (let move of b.moves) {
        if (last_from != move.from) {
            // console.log("\nFROM:", move.from)
            last_from = move.from;
        }
        
        // console.log(" >", move.to)
    }
}