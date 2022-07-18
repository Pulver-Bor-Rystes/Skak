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


const x12_valid_indexes: x12_index[] = [26, 27, 28, 29, 30, 31, 32, 33, 38, 39, 40, 41, 42, 43, 44, 45, 50, 51, 52, 53, 54, 55, 56, 57, 62, 63, 64, 65, 66, 67, 68, 69, 74, 75, 76, 77, 78, 79, 80, 81, 86, 87, 88, 89, 90, 91, 92, 93, 98, 99, 100, 101, 102, 103, 104, 105, 110, 111, 112, 113, 114, 115, 116, 117];
const x12_to_x8: { [key: x12_index]: x8_index } = {27: 1, 28: 2, 29: 3, 30: 4, 31: 5, 32: 6, 33: 7, 38: 8, 39: 9, 40: 10, 41: 11, 42: 12, 43: 13, 44: 14, 45: 15, 50: 16, 51: 17, 52: 18, 53: 19, 54: 20, 55: 21, 56: 22, 57: 23, 62: 24, 63: 25, 64: 26, 65: 27, 66: 28, 67: 29, 68: 30, 69: 31, 74: 32, 75: 33, 76: 34, 77: 35, 78: 36, 79: 37, 80: 38, 81: 39, 86: 40, 87: 41, 88: 42, 89: 43, 90: 44, 91: 45, 92: 46, 93: 47, 98: 48, 99: 49, 100: 50, 101: 51, 102: 52, 103: 53, 104: 54, 105: 55, 110: 56, 111: 57, 112: 58, 113: 59, 114: 60, 115: 61, 116: 62, 117: 63 };

interface Move {
    from: Square
    to: Square
    piece: Piece
    capture?: boolean
    delete?: Square
    extra_move?: Move
}

interface SquareData {
    index: number
    x12_index: number
    square: Square
    piece: Piece
    color: Color
    behaviour: Behaviour
}


function default_x(lim: number = 8): Color[] | Piece[] {
    const colors: Color[] = [];
    for (let i = 0; i < (lim*lim); i++) {
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

type x8_index = number;
type x12_index = number;



function convert_x8_to_x12(x8: x8_index): x12_index {
    let offset = 26;

    for (let off=1; off <= 9; off++) {
        if (x8 < (off*8)) {
            break;
        }

        offset += 4;
    }

    return x8 + offset;
}

function convert_x12_to_x8(x12: x12_index): x8_index {
    return x12_to_x8[x12] as x8_index;
}





class BoardData {
    extended: Boolean;
    colors: Color[];
    pieces: Piece[];

    constructor (lim: number = 8) {
        this.colors = default_x(lim) as Color[];
        this.pieces = default_x(lim) as Piece[];
        this.extended = lim != 8;
    }



    get_piece_square (piece: Piece, color: Color): Square | false {
        let square: Square | false = false;

        for (let i=0; i<this.colors.length; i++) {
            if (this.colors[i] == color && this.pieces[i] == piece) {
                let x8 = this.extended ? convert_x12_to_x8(i) : i;
                
                square = squares[x8];
            }
        }
        
        return square;
    }

    
    // loader et x8 board
    from_x8 (board: BoardData) {
        if (this.extended && !board.extended) {
            // skal kun loade et x8 board, hvi

            let start_index = 12*2 + 2;

            let index = 0;
            for (let i=0; i < 64; i++) {

                this.colors[start_index + index] = board.colors[i];
                this.pieces[start_index + index] = board.pieces[i];

                if (++index >= 8) {
                    index = 0;
                    start_index += 12;
                }
            }
        }
    }

    log (info?: { turn: Color, moves_made: number, available_moves: number, en_passant?: Square }) {
        const max = this.extended ? 12:8;
                
        let count = max;
        let y = max;
        for (let i = 0; i < (max*max); i++) {
            if (++count >= max) {
                count = 0;
                let space = y < 10 ? " ":"";

                if (info) {
                    if (y == 5) {
                        process.stdout.write(`     turn: ${info.turn}`)
                    }
                    else if (y == 4) {
                        process.stdout.write(`     number of moves made: ${info.moves_made}`)
                    }
                    else if (y == 3) {
                        process.stdout.write(`     available moves: ${info.available_moves}`)
                    }
                    else if (y == 2) {
                        process.stdout.write(`     én passant: ${info.en_passant}`)
                    }
                }

                process.stdout.write(`\n${space}${y--}|`);
            }
            let color = this.colors[i];
            let piece = this.pieces[i];

            let piece_name: string = piece[0];
            if (piece == "KNIGHT") {
                piece_name = "N";
            }

            let p = '.';

            if (color != "EMPTY" || piece != "EMPTY") {
                p = (color == "DARK") ? piece_name.toLowerCase() : piece_name;
            }

            process.stdout.write(` ${p}`);
        }

        if (this.extended) {
            process.stdout.write(`\n  | -----------------------\n       `);
        }
        else {
            process.stdout.write(`\n  | ---------------\n   `);
        }
        for (const sl of square_letters) {
            process.stdout.write(` ${sl}`)
        }
        process.stdout.write("\n");
    }

    to_x8 () {
        if (this.extended) {
            let board = new BoardData;

            let start_index = 12*2 + 2;

            let index = 0;
            for (let i=0; i < 64; i++) {

                board.colors[i] = this.colors[start_index + index];
                board.pieces[i] = this.pieces[start_index + index];

                if (++index >= 8) {
                    index = 0;
                    start_index += 12;
                }
            }

            return board;
        }
        else {
            return this;
        }
    }


}


export class Board {
    for_realsies = true;

    old_state: Board[] = [];

    x8board = new BoardData;
    temp_x8board = new BoardData;
    x12board = new BoardData(12);

    // key: ghost_x12_pos -> [ real_x8_pos, real_x12_pos ]
    // jeg er lidt i tvivl om hvorvidt index=0 overhovedet bliver brugt...
    enpassant: { [key: x12_index]: [x8_index, x12_index]; } = {};

    turn: Color = "LIGHT";
    number_of_moves: number = 0;

    moves: Move[] = [];

    simple_moves: { [key: string]: Move } = {};


    has_moved = {
        "KING": [false, false],
        "RIGHT_ROOK": [false, false],
        "LEFT_ROOK": [false, false],
    }


    constructor(fen: string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR", temp=false) {
        this.for_realsies = !temp;
        this.load_fen(fen)
        this.gen()
    }

    private gen_sliding_number_of_moves(SD: SquareData, limit?: boolean) {
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
                    let capture = false;

                    if (NEW_SD.color != "EMPTY") {
                        keep_going = false;
                        capture = true;
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
                        to: NEW_SD.square,
                        piece: SD.piece,
                        capture: capture,
                    }

                    this.moves.push( new_move )
                }
                else {
                    keep_going = false;
                }

                if (limit) {
                    keep_going = false;
                }
            }
        }
        
        // return moves;
    }


    
    private gen_king_moves(SD: SquareData) {
        this.gen_sliding_number_of_moves(SD, true);

        let has_moved_index = this.turn == "LIGHT" ? 0:1;

        if (!this.has_moved["KING"][has_moved_index]) {
            // check each direction if rooks are able

            if (!this.has_moved["LEFT_ROOK"][has_moved_index]) {
                // check if space in between is empty
                if (this.x8board.colors[SD.index-1] == "EMPTY" && this.x8board.colors[SD.index-2] == "EMPTY" && this.x8board.colors[SD.index-3] == "EMPTY") {
                    
                    let threat = false;
                    // sørg for at ingen af de indegående felter er truede.
                    for (let i=4; i>=0; i--) {
                        let square = squares[ squares.indexOf(SD.square) - i ];

                        if (this.is_square_in_danger(square)) {
                            threat = true;
                        }
                    }

                    if (!threat) {
                        this.moves.push({
                            from: SD.square,
                            to: squares[ squares.indexOf(SD.square) - 2 ],
                            piece: SD.piece,
                            extra_move: {
                                from: squares[ squares.indexOf(SD.square) - 4 ],
                                to: squares[ squares.indexOf(SD.square) - 1 ],
                                piece: "ROOK",
                            }
                        })
                    }
                }
            }

            if (!this.has_moved["RIGHT_ROOK"][has_moved_index]) {
                if (this.x8board.colors[SD.index+1] == "EMPTY" && this.x8board.colors[SD.index+2] == "EMPTY") {
                    let threat = false;
                    // sørg for at ingen af de indegående felter er truede.
                    for (let i=3; i>=0; i--) {
                        let square = squares[ squares.indexOf(SD.square) + i ];

                        if (this.is_square_in_danger(square)) {
                            threat = true;
                        }
                    }

                    if (!threat) {
                        this.moves.push({
                            from: SD.square,
                            to: squares[ squares.indexOf(SD.square) + 2 ],
                            piece: SD.piece,
                            extra_move: {
                                from: squares[ squares.indexOf(SD.square) + 3 ],
                                to: squares[ squares.indexOf(SD.square) + 1 ],
                                piece: "ROOK",
                            }
                        })
                    }
                }
            }
        }
    }


    
    private gen_pawn_moves(SD: SquareData) {
        let pawn_move = ["2", "7"].includes(SD.square[1]) ? [ 12, 24 ] : [ 12 ];
        let captures = SD.color == "DARK" ? [ 11, 13 ] : [ -11, -13 ];
        
        for (let PM of pawn_move) {
            PM = SD.color == "LIGHT" ? -PM:PM;

            const new_x12_pos = PM + SD.x12_index;
            const [res, move] = this.is_move_valid(SD, new_x12_pos);
            if (res) {
                this.moves.push(move);
            }
        }


        for (const cap of captures) {
            const new_x12_pos = cap + SD.x12_index;
            const [res, move] = this.is_move_valid(SD, new_x12_pos, true);


            if (res) {
                this.moves.push(move);
            }
            else {
                if (new_x12_pos in this.enpassant) {
                    this.moves.push({
                        piece: SD.piece,
                        from: SD.square,
                        to: squares[x12_valid_indexes.indexOf(new_x12_pos)],
                        capture: true,
                        delete: squares[x12_valid_indexes.indexOf(this.enpassant[new_x12_pos][1])]
                    })
                }
            }
        }
    }

    private is_move_valid(SD: SquareData, new_x12_pos: number, only_accept_captures?: boolean): [false, null] | [true, Move] {
        if (x12_valid_indexes.includes(new_x12_pos)) {
            // hvis den nye position er på brættet
            let new_index = x12_valid_indexes.indexOf(new_x12_pos);
            const to = squares[new_index];

            let not_the_same_color = this.x12board.colors[new_x12_pos] != SD.color;
            let not_empty = this.x12board.colors[new_x12_pos] == "EMPTY" ? false:true

            if ((not_the_same_color && !only_accept_captures) || (only_accept_captures && not_the_same_color && not_empty )) {
                // hvis den nye position ikke er optaget af egen farve.
                let new_move: Move = {
                    from: SD.square,
                    to: to,
                    piece: SD.piece,
                    capture: not_the_same_color && not_empty
                }

                return [true, new_move];
            }
        }

        return [false, null];
    }
    

    private gen_knight_moves(SD: SquareData) {
        // lav et loop, der spytter alle positioner ud
        // tjek for hver position om den er gyldig.

        let knight_offsets = [ -10, -14, -23, -25, 10, 14, 23, 25 ];

        for (const KO of knight_offsets) {
            const new_x12_pos = KO + SD.x12_index;

            const [res, move] = this.is_move_valid(SD, new_x12_pos);
            if (res) {
                this.moves.push(move);
            }
        }


    }
    

    private gen_pseudo_legal() {
        this.to_x12();
        this.moves = [];

        for (let piece_id = 0; piece_id < 64; piece_id++) {
            const SD: SquareData = this.get_square_data(piece_id);

            if (SD.color == this.turn) {
                if (SD.behaviour == "SLIDE") {
                    this.gen_sliding_number_of_moves(SD);
                }
                else if (SD.piece == "KNIGHT") {
                    this.gen_knight_moves(SD);
                }
                else if (SD.piece == "PAWN") {
                    this.gen_pawn_moves(SD);
                }
                else if (SD.piece == "KING") {
                    this.gen_king_moves(SD);
                }
            }
        }
    }


    switch_turn() {
        if (this.turn == "LIGHT") {
            this.turn = "DARK";
        }
        else {
            this.turn = "LIGHT";
        }

        this.gen()
    }


    private is_square_in_danger(square: Square) {
        if (!this.for_realsies) {
            return;
        }

        const temp_board = new Board( this.to_fen(), true );
        temp_board.switch_turn();
        
        if (Object.keys( temp_board.simple_moves ).includes(square)) {
            return true;
        }

        return false;
    }


    private pick_legal() {
        // vi har en masse træks
        // gå alle træk igennem og beregn alle nye muligheder
        // hvis en af de nye muligheder indeholder et træk som dræber kongen, stop og fjern muligheden.

        let keys_to_delete = [];

        for (const key of Object.keys(this.simple_moves)) {
            // lav kopi
            const temp_board = this.new_board_from_move( this.simple_moves[key] );
            
            const KING_SQUARES: Square[] = [];
    
            for (let i = 0; i<64; i++) {
                if (temp_board.x8board.pieces[i] == "KING") {
                    KING_SQUARES.push(squares[i]);
                }
            }
            
            
            for (const move_key of Object.keys(temp_board.simple_moves)) {
                const move = temp_board.simple_moves[move_key];
                
                if (!move.capture) {
                    continue;
                }
                
                if (KING_SQUARES.includes(move.to)) {
                    keys_to_delete.push(key)
                }
            }
        }

        for (const key of keys_to_delete) {
            delete this.simple_moves[key];
        }
    }


    private new_board_from_move(move: Move) {
        for (let i=0; i<64; i++) {
            // kopier!
            this.temp_x8board.colors[i] = this.x8board.colors[i];
            this.temp_x8board.pieces[i] = this.x8board.pieces[i];

        }
        this._move_pieces(move, this.temp_x8board);

        let fen_str = this.to_fen(this.temp_x8board);

        const temp_board = new Board(fen_str, true);
        temp_board.switch_turn();
        
        return temp_board;
    }






    to_fen(board: BoardData=this.x8board) {
        let space_so_far = 0;
        let fen_string = "";

        let x = 0;

        for (let i=0; i<64; i++) {
            x ++;

            let color = board.colors[i];
            let piece = board.pieces[i];

            let corrected_piece = piece == "KNIGHT" ? "NIGHT":piece;
            let piece_char = color == "DARK" ? corrected_piece[0].toLowerCase():corrected_piece[0];

            if (piece != "EMPTY") {
                if (space_so_far > 0) {
                    fen_string += space_so_far;
                    space_so_far = 0
                }
                fen_string += piece_char;
            }
            else {
                space_so_far ++;
            }
            


            if (x >= 8) {
                if (space_so_far > 0) {
                    fen_string += space_so_far;
                    space_so_far = 0
                }
                if (fen_string[fen_string.length-1] == "/") {
                    fen_string += "8"
                }
                fen_string += "/";
                x = 0;
                space_so_far = 0;
            }
        }


        if (fen_string[fen_string.length-1] == "/") {
            fen_string = fen_string.slice(0, fen_string.length-1);
        }


        // meta data

        fen_string += " " + (this.turn == "LIGHT" ? 'w':'b');

        // castle rights
        let rights = " ";
        for (let i of [0, 1]) {
            if (!this.has_moved["KING"][i]) {
                if (!this.has_moved["RIGHT_ROOK"][i]) {
                    rights += (i == 0) ? 'K':'k';
                }
                if (!this.has_moved["LEFT_ROOK"][i]) {
                    rights += (i == 0) ? 'Q':'q';
                }
            }
        }

        if (rights == " ") {
            rights = " -";
        }

        fen_string += rights


        // en passant
        let keys = Object.keys(this.enpassant);
        if (keys.length == 0) {
            fen_string += " -";
        }
        else {
            // @ts-ignore
            let x8_i = this.x12_index_to_x8(keys[0]);
            let square = squares[x8_i];
            fen_string += ` ${square}`;
        }


        // half moves



        // turns
        return fen_string;
    }



    private check_for_checks() {
        for (let sm of Object.keys(this.simple_moves)) {
            console.log("\n" + sm)
            let move = this.simple_moves[sm];
            // vi har et træk.
            // lav trækket på et falsk board
            // og tjek om den modsatte konge er i fare.

            let temp_board = this.new_board_from_move(move);
            temp_board.to_x12();
            let king_square = temp_board.x12board.get_piece_square("KING", temp_board.turn) // får fat i den konge hvis tur det er.

            if (!king_square) {
                return;
            }

            // vi skal egentlig bare tjekke om kongen kan blive taget i de to retninger der er.
            // 1. retningen, som er fra kongens position til den nye placering.
            // 2. retningen, som er -||- til den gamle placering.
            
            // tjek i retning som en løber og hvis den støder ind i løber eller dronning

            let check_discovered = false;
            let bishop_directions = [11, 13, -11, -13];


            const king_index = squares.indexOf(king_square);

            let old_dir = 0;
            let new_dir = 0;

            // hvis den har samme bogstav, så kan den jo kun gå op eller ned
            // 6 > 4 -> 12
            new_dir += move.to[1] != king_square[1] && move.to[1] < king_square[1]
                ? 12:-12;
            new_dir += move.to[0] != king_square[0] && square_letters.indexOf(move.to[0] as Square_Letter) > square_letters.indexOf(king_square[0] as Square_Letter)
                ? 1:-1;

            old_dir += move.from[1] != king_square[1] && move.from[1] < king_square[1]
                ? 12:-12;
            old_dir += move.from[0] != king_square[0] && square_letters.indexOf(move.from[0] as Square_Letter) > square_letters.indexOf(king_square[0] as Square_Letter)
                ? 1:-1;

            console.log(new_dir, old_dir)

            for (let dir of bishop_directions) {
                // tjek om vi støder ind i en dronning eller bishop

                let index = convert_x8_to_x12(king_index);

                while (true) {
                    index += dir;
                    if (!x12_valid_indexes.includes(index)) {
                        // så er vi uden for brættet
                        break;
                    }

                    let piece = temp_board.x12board.pieces[index];
                    let color = temp_board.x12board.colors[index];

                    if (color != this.turn) {
                        continue;
                    }

                    if (piece == "BISHOP" || piece == "QUEEN") {
                        check_discovered = true;
                        break;
                    }
                }
            }

            if (check_discovered) {
                console.log(`move: ${sm} makes a check!`);
            }
            
        }
    }



    /**
     * Beregner alle mulige træk!
    **/
    gen() {
        this.gen_pseudo_legal();
        this.simplify_moves();

        if (this.for_realsies) {
            this.pick_legal();
            this.check_for_checks();
        }
    }


    load_fen(fen: string) {
        /*
            0: pieces ✅
            1: turn ✅
            2: castling rights ✅
            3: én passant ❌
            4: half moves since last capture ❌
            5: amount of turns ❌

        */


        let index = 0;
        
        let parts_space = fen.split(' ');
        if (parts_space[1]) {
            this.turn = parts_space[1].toLowerCase() == "b" ? "DARK":"LIGHT";
        }

        if (parts_space[2]) {
            let rights = parts_space[2];

            if (rights == "-") {
                for (let i of [0, 1]) {
                    for (let key of Object.keys(this.has_moved)) {
                        // @ts-ignore
                        this.has_moved[key][i] = true;
                    }
                }
            }
            else {
                let parts = rights.split("");

                let correspond_to: { [key: string]: [number, string] } = {
                    "Q": [0, "LEFT_ROOK"],
                    "q": [1, "LEFT_ROOK"],
                    "K": [0, "RIGHT_ROOK"],
                    "k": [1, "RIGHT_ROOK"],
                }

                for (let part of parts) {
                    let [i, key] = correspond_to[part];

                    // @ts-ignore
                    this.has_moved[key][i] = false;
                }
            }
        }

        if (parts_space[3]) {
            if (parts_space[3] != "-") {   
                let square = parts_space[3] as Square;
                let gx8_i = squares.indexOf(square);
                let gx12_i = this.x8_index_to_x12(gx8_i);

                // real
                let x8 = square[1] == "3" ? this.x12_index_to_x8( gx12_i + 8 ) : this.x12_index_to_x8( gx12_i - 8 );
                let x12 = this.x8_index_to_x12(x8)
                
                this.enpassant[gx12_i] = [x8, x12];
            }
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
                                    throw Error(`unknown command: ${com}`);
                            }
                            
                            this.x8board.colors[index] = color;
                            this.x8board.pieces[index] = piece;
                            


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


    log() {
        let square: Square | null = null;
        if (Object.keys(this.enpassant).length != 0) {
            square = squares[ this.x12_index_to_x8 (Number(Object.keys(this.enpassant)[0])) ]
        }

        if (square) {
            this.x8board.log({turn: this.turn, moves_made: this.number_of_moves, available_moves: this.moves.length, en_passant: square});
        }
        else {
            this.x8board.log({turn: this.turn, moves_made: this.number_of_moves, available_moves: this.moves.length });
        }
        // this.x8board.log();
    }

    private to_x12() {
        this.x12board.from_x8(this.x8board);
    }

    private to_x8() {
        this.x8board = this.x12board.to_x8();
    }

    private x8_index_to_x12(index: x8_index) {
        let offset = 26;

        for (let off=1; off <= 9; off++) {
            if (index < (off*8)) {
                break;
            }

            offset += 4;
        }

        return index + offset;
    }

    private x12_index_to_x8(index: x12_index) {
        return x12_to_x8[index] as x8_index;
    }


    private get_square_data(index: number): SquareData {
        let SD: SquareData = {
            "index": index,
            "x12_index": this.x8_index_to_x12(index),
            "square": squares[index],
            "piece": this.x8board.pieces[index],
            "color": this.x8board.colors[index],
            "behaviour": (["QUEEN", "ROOK", "BISHOP"].includes(this.x8board.pieces[index])) ? "SLIDE" : "NOTHING",
        }

        return SD;
    }




    simplify_moves() {
        this.simple_moves = {};

        // key: to, val: Move
        const col_moves: { [ key: string ]: Move[] } = {}


        // Sorter efter brik
        for (const MV of this.moves) {
            if (MV.piece in col_moves) {
                col_moves[MV.piece].push(MV)
            }
            else {
                col_moves[MV.piece] = [MV]
            }
        }

        
        for (const key_piece of Object.keys(col_moves)) {
            if (key_piece == "PAWN") {
                for (const MV of col_moves[key_piece]) {
                    const upgrade_to = ["1", "8"].includes(MV.to[1]) ? ["=B", "=N", "=R", "=Q"]:[""];

                    for (const UT of upgrade_to) {
                        if (MV.capture) {
                            this.simple_moves[`${MV.from[0].toLowerCase()}x${MV.to.toLowerCase()}${UT}`] = MV;
                        }
                        else {
                            this.simple_moves[`${MV.to.toLowerCase()}${UT}`] = MV;
                        }
                    }
                }
            }
            else {
                let to_moves: { [ key: string ]: Move[] } = {}

                for (let MV of col_moves[key_piece]) {
                    if (!(MV.to in to_moves)) {
                        to_moves[MV.to] = [MV];
                    }
                    else {
                        to_moves[MV.to].push(MV);
                    }
                }

                for (let key of Object.keys(to_moves)) {
                    let amount = to_moves[key].length;
                    
                    for (let MV of to_moves[key]) {

                        // convert castling to correct notation
                        if (MV.extra_move) {
                            if (MV.to.includes("C")) {
                                this.simple_moves["O-O-O"] = MV;
                            }
                            else if (MV.to.includes("G")) {
                                this.simple_moves["O-O"] = MV;
                            }
                        }

                        else if (amount > 1) {
                            this.simple_moves[ `${MV.piece == "KNIGHT" ? "N":MV.piece[0]}${MV.from[0].toLowerCase()}${ MV.capture ? "x":"" }${MV.to.toLowerCase()}` ] = MV;
                        }
                        else {
                            this.simple_moves[ `${MV.piece == "KNIGHT" ? "N":MV.piece[0]}${ MV.capture ? "x":"" }${MV.to.toLowerCase()}` ] = MV;
                        }
                    }
                }
            }
        }
    }


    private _move_pieces(move_data: Move, board: BoardData=this.x8board) {
        let old_index = squares.indexOf(move_data.from)
        let new_index = squares.indexOf(move_data.to)
        
        // fjern brik
        board.colors[old_index] = "EMPTY";
        board.pieces[old_index] = "EMPTY";
        
        // erstart brik
        board.colors[new_index] = this.turn;
        board.pieces[new_index] = move_data.piece;

        if (move_data.extra_move) {
            this._move_pieces(move_data.extra_move, board);
        }

        return [old_index, new_index];
    }

    move(mv: string) {
        if (mv in this.simple_moves) {
            let move_data = this.simple_moves[mv];
            
            let [old_index, new_index] = this._move_pieces(move_data);

            // fjern evt enpassant brik

            const x12_index = this.x8_index_to_x12(new_index);

            if (x12_index in this.enpassant) {
                // fjern brik
                this.x8board.colors[this.enpassant[x12_index][0]] = "EMPTY";
                this.x8board.pieces[this.enpassant[x12_index][0]] = "EMPTY";
            }

            // ryd op i en passant
            this.enpassant = {};

            // tilføj ny enpassant, hvis nødvendigt
            // eller opgradér
            if (move_data.piece == "PAWN") {
                if (Math.abs(Number(move_data.from[1]) - Number(move_data.to[1])) == 2) {
                    let num = move_data.from[1];
                    let new_pos = num == "2" ? "3":"6";

                    let new_square = `${move_data.from[0]}${new_pos}` as Square;
                    let ns_index = squares.indexOf(new_square);
                    let nsx12 = this.x8_index_to_x12(ns_index);

                    this.enpassant[nsx12] = [new_index, nsx12 + ( this.turn == "LIGHT" ? 12:-12 )];
                }

                if (mv.includes("=")) {
                    const upgrade_to = mv.split("=")[1];

                    switch (upgrade_to) {
                        case "B":
                            this.x8board.pieces[new_index] = "BISHOP";
                            break;
                        case "N":
                            this.x8board.pieces[new_index] = "KNIGHT";
                            break;
                        case "R":
                            this.x8board.pieces[new_index] = "ROOK";
                            break;
                        default:
                            this.x8board.pieces[new_index] = "QUEEN";
                            break;
                    }
                }
            }

            let has_moved_index = this.turn == "LIGHT" ? 0:1;

            if (move_data.piece == "KING") {
                this.has_moved[move_data.piece][has_moved_index] = true;
            }
            else if (move_data.piece == "ROOK") {
                let side = move_data.from[0] == 'A' ? "LEFT":move_data.from[0] == 'H' ? "RIGHT":"NOPE";
                if (side != "NOPE") {
                    // @ts-ignore
                    this.has_moved[`${side}_${move_data.piece}`][has_moved_index] = true;
                }
            }

            this.simple_moves = {};
            if (this.turn == "LIGHT") {
                this.turn = "DARK";
            }
            else {
                this.turn = "LIGHT";
            }

            this.gen()
        }
        else {
            throw `${mv} is not a move!`;
        }
    }


    generate_moves() {
        if (this.simple_moves = {}) {
            this.gen();
        }

        return Object.keys(this.simple_moves);
    }
}





if (process.argv[2] == "test") {
    console.clear()
    let b = new Board("b6k/1p6/8/8/p3K2p b - -");
    // let b = new Board;
    
    b.log()
    console.log(b.simple_moves)
    // console.log(b.to_fen())
}


