export type Square = '' | 'a8' | 'b8' | 'c8' | 'd8' | 'e8' | 'f8' | 'g8' | 'h8' | 'a7' | 'b7' | 'c7' | 'd7' | 'e7' | 'f7' | 'g7' | 'h7' | 'a6' | 'b6' | 'c6' | 'd6' | 'e6' | 'f6' | 'g6' | 'h6' | 'a5' | 'b5' | 'c5' | 'd5' | 'e5' | 'f5' | 'g5' | 'h5' | 'a4' | 'b4' | 'c4' | 'd4' | 'e4' | 'f4' | 'g4' | 'h4' | 'a3' | 'b3' | 'c3' | 'd3' | 'e3' | 'f3' | 'g3' | 'h3' | 'a2' | 'b2' | 'c2' | 'd2' | 'e2' | 'f2' | 'g2' | 'h2' | 'a1' | 'b1' | 'c1' | 'd1' | 'e1' | 'f1' | 'g1' | 'h1';
export const squares: Square[] = ['a8', 'b8', 'c8', 'd8', 'e8', 'f8', 'g8', 'h8', 'a7', 'b7', 'c7', 'd7', 'e7', 'f7', 'g7', 'h7', 'a6', 'b6', 'c6', 'd6', 'e6', 'f6', 'g6', 'h6', 'a5', 'b5', 'c5', 'd5', 'e5', 'f5', 'g5', 'h5', 'a4', 'b4', 'c4', 'd4', 'e4', 'f4', 'g4', 'h4', 'a3', 'b3', 'c3', 'd3', 'e3', 'f3', 'g3', 'h3', 'a2', 'b2', 'c2', 'd2', 'e2', 'f2', 'g2', 'h2', 'a1', 'b1', 'c1', 'd1', 'e1', 'f1', 'g1', 'h1'];


export type Square_Letter = 'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h';
export const square_letters: Square_Letter[] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

export type Color = 'LIGHT' | 'DARK' | 'EMPTY';
export const DEFAULT_COLOR: Color = 'EMPTY';


export type PlayerColor = 'WHITE' | 'BLACK';

export type Piece = 'PAWN' | 'KNIGHT' | 'BISHOP' | 'ROOK' | 'QUEEN' | 'KING' | 'EMPTY';
export const DEFAULT_PIECE = 'EMPTY';

export type x8_index = number;
export type x12_index = number;

export const x12_valid_indexes: x12_index[] = [26, 27, 28, 29, 30, 31, 32, 33, 38, 39, 40, 41, 42, 43, 44, 45, 50, 51, 52, 53, 54, 55, 56, 57, 62, 63, 64, 65, 66, 67, 68, 69, 74, 75, 76, 77, 78, 79, 80, 81, 86, 87, 88, 89, 90, 91, 92, 93, 98, 99, 100, 101, 102, 103, 104, 105, 110, 111, 112, 113, 114, 115, 116, 117];
export const x12_to_x8: { [key: x12_index]: x8_index } = {27: 1, 28: 2, 29: 3, 30: 4, 31: 5, 32: 6, 33: 7, 38: 8, 39: 9, 40: 10, 41: 11, 42: 12, 43: 13, 44: 14, 45: 15, 50: 16, 51: 17, 52: 18, 53: 19, 54: 20, 55: 21, 56: 22, 57: 23, 62: 24, 63: 25, 64: 26, 65: 27, 66: 28, 67: 29, 68: 30, 69: 31, 74: 32, 75: 33, 76: 34, 77: 35, 78: 36, 79: 37, 80: 38, 81: 39, 86: 40, 87: 41, 88: 42, 89: 43, 90: 44, 91: 45, 92: 46, 93: 47, 98: 48, 99: 49, 100: 50, 101: 51, 102: 52, 103: 53, 104: 54, 105: 55, 110: 56, 111: 57, 112: 58, 113: 59, 114: 60, 115: 61, 116: 62, 117: 63 };

interface MoveData {
    from: Square
    to: Square
    piece: Piece
    capture?: Piece | undefined
    is_check?: boolean
    is_checkmate?: boolean
    upgrade_to?: Piece
}


import { dir } from "console";
import { off } from "process";
import { load_fen } from "./ugly_functions";



class helpers {
    static default_x (lim: number = 8): Color[] | Piece[] {
        const colors: Color[] = [];
        for (let i = 0; i < (lim*lim); i++) {
            colors.push("EMPTY");
        }
        return colors;
    }

    static convert_player_color_to_color (pl_col: PlayerColor) {
        return pl_col == "WHITE" ? "LIGHT":"DARK";
    }

    static convert_x8_to_x12 (x8: x8_index): x12_index {
        let offset = 26;
    
        for (let off=1; off <= 9; off++) {
            if (x8 < (off*8)) {
                break;
            }
    
            offset += 4;
        }
    
        return x8 + offset;
    }
    
    static convert_x12_to_x8 (x12: x12_index): x8_index {
        return x12_to_x8[x12] as x8_index;
    }

    static calc_dir_towards_square (from: Square, towards: Square): x12_index {
        if ((from+towards).length != 4) {
            return 0;
        }

        let dir = 0;

        dir += Number (towards[1]) > Number (from[1]) ? 1:-1;
        dir += square_letters.indexOf (towards[0] as Square_Letter) > square_letters.indexOf (from[0] as Square_Letter) ? 12:-12;

        return dir;
    }
}




class xBoard {
    colors: Color[];
    pieces: Piece[];
    
    is_the_ext: boolean;
    ext_board: xBoard | undefined;


    constructor(lim = 8) {
        this.colors = helpers.default_x(lim) as Color[];
        this.pieces = helpers.default_x(lim) as Piece[];

        if (lim == 8) {
            this.is_the_ext = false;
            this.ext_board = new xBoard(12);
            this.update_ext_board();
        }
        else {
            this.is_the_ext = true;
        }
    }


    update_ext_board() {
        if (!this.ext_board) {
            return;
        }

        let start_index = 12*2 + 2;

        let index = 0;
        for (let i=0; i < 64; i++) {

            this.ext_board.colors[start_index + index] = this.colors[i];
            this.ext_board.pieces[start_index + index] = this.pieces[i];

            
            if (++index >= 8) {
                index = 0;
                start_index += 12;
            }
        }
    }


    get_piece (piece: Piece, color: Color): Square {
        let lim = this.ext_board ? 64:144;

        for (let i = 0; i < lim; i++) {
            if (this.colors[i] == "EMPTY") {
                continue;
            }

            if (this.colors[i] == color && this.pieces[i] == piece) {
                let i_8 = this.ext_board ? i:helpers.convert_x12_to_x8 (i);
                
                return squares[i_8];
            }
        }

        return "";
    }


    get_piece_on_square (square: Square): [Piece, Color] {
        let x8 = squares.indexOf (square);

        let index = this.ext_board ? x8 : helpers.convert_x8_to_x12 (x8);
        return [this.pieces[index], this.colors[index]];
    }


    move (move: MoveData, turn: PlayerColor) {
        let from_i8 = squares.indexOf(move.from);
        let to_i8 = squares.indexOf(move.to);

        let from_x = this.ext_board ? from_i8 : helpers.convert_x8_to_x12 (from_i8);
        let to_x = this.ext_board ? to_i8 : helpers.convert_x8_to_x12 (to_i8);


        // fjern den gamle
        this.colors[from_x] = "EMPTY";
        this.pieces[from_x] = "EMPTY";

        // tilføj den nye
        this.colors[to_x] = helpers.convert_player_color_to_color (turn);
        this.pieces[to_x] = move.upgrade_to || move.piece;

        // returnér en passant
        if (move.piece == "PAWN") {
            if (move.from[1] == "2" && move.to[1] == "4") {
                return `${move.from[0]}3`;
            }
            else if (move.from[1] == "7" && move.to[1] == "5") {
                return `${move.from[0]}3`;
            }
        }
    }

    /** Afprøver et træk på x12 brættet. Returnerer enten et gyldigt træk eller værdien: false */
    try_move (index: x8_index, offset: x12_index) {
        if (!this.ext_board) {
            return;
        }

        
        let from = squares[index];
        let to_index: x12_index = (helpers.convert_x8_to_x12 (index) + offset);
        let to = squares[ helpers.convert_x12_to_x8 (to_index) ];
        
        let from_color = this.colors[index];
        let to_color = this.ext_board.colors[to_index];
        
        // hvis "to" er udenfor brættet stopper det her
        if (x12_valid_indexes.includes(to_index)) {

            if (from_color != to_color) {
                let capture: Piece = this.get_piece_on_square (to)[0];
        
                let move: MoveData = {
                    from,
                    to,
                    capture: capture == "EMPTY" ? undefined : capture,
                    piece: this.get_piece_on_square (from)[0],
                }
        
                return move;
            }
            else {
                return false;
            }
        }
        else {
            return false;
        }
    }

    clear_board () {
        let limit = this.ext_board ? 64:144;
        for (let i = 0; i < limit; i++) {
            this.pieces[i] = "EMPTY";
            this.colors[i] = "EMPTY";
        }
    }

    log (turn?: string, moves_made?: number, available_moves?: number, en_passant?: string) {
        const max = this.is_the_ext ? 12:8;
        
        let count = max;
        let y = max;
        for (let i = 0; i < (max*max); i++) {
            
            if (++count >= max) {
                count = 0;
                let space = y < 10 ? " ":"";
                
                if (!this.is_the_ext) {
                    let msgs = [
                        turn != undefined ? `turn: ${turn}`:"",
                        moves_made != undefined ? `number of moves made: ${moves_made}`:"",
                        available_moves != undefined ? `available moves: ${available_moves}`:"",
                        en_passant != undefined ? `én passant: ${en_passant}`:"",
                    ]
    
                    for (let o=0; o < msgs.length; o++) {
                        if (y == (msgs.length - o) + 1) {
                            process.stdout.write(`     ${msgs[o]}`);
                        }
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

        if (this.is_the_ext) {
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
}











interface Threat {
    square: Square
    direction_away_from_king: x12_index
}


export class Chess {
    board = new xBoard;

    // stats
    turn: PlayerColor = "WHITE";
    moves_made: number = 0;

    raw_moves: MoveData[] = [];
    available_moves: { [key: string]: MoveData } = {};
    en_passant: Square = "";
    

    /** key: PlayerColor */
    known_threats: { [key: string]: Threat[] } = { "WHITE": [], "BLACK": [] };

    /** PlayerColor: [queenside, kingside] */
    castle_rights: { [key: string]: boolean[] } = {
        "WHITE": [true, true],
        "BLACK": [true, true],
    };

    
    constructor () {
        this.load_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
    }




    gen () {
        // iterer over brikker hvis tur det er.

        for (let _i in this.board.pieces) {
            let i = Number(_i);

            
            if (this.board.colors[i] != helpers.convert_player_color_to_color (this.turn)) {
                continue;
            }

            // Herfra er det udelukkende mine egne brikker vi kan se!

            let piece = this.board.pieces[i];

            
            switch (piece) {
                case "PAWN":
                    this.calc_pawn_moves(i);
                    break;
                default:
                    // nothing;
                    break;
            }
        }
    }


    private calc_pawn_moves (index: x8_index) {
        let square = squares[index];
        let limit = 1;
        limit = this.turn == "WHITE" && square[1] == "2" ? 2:limit;
        limit = this.turn == "BLACK" && square[1] == "7" ? 2:limit;

        let direction = this.turn == "WHITE" ? -12:12;


        // forward
        this.calc_sliding_moves (index, [direction], limit, (move: MoveData) => {
            if (move.capture) {
                return false;
            }

            this.submit_move_data (move);
        });

        // captures
        this.calc_sliding_moves (index, [direction-1, direction+1], 1, (move: MoveData) => {
            if (!move.capture) {
                return false;
            }

            this.submit_move_data (move);
        });
    }


    calc_sliding_moves (index: x8_index, directions: x12_index[], limit=8, post?: Function) {
        this.board.update_ext_board ();
        const real_limit = limit;
        
        for (const dir of directions) {
            let offset: x12_index = dir;
            limit = real_limit;

            
            while (limit--) {
                let move = this.board.try_move (index, offset); // skal bare prøve trækket af og generer movedata.

                // hvis det er et gyldigt træk, kør videre.
                if (move) {
                    if (post) {
                        if (post (move, limit) === false) {
                            break;
                        }
                    }
                    else {
                        this.submit_move_data (move);
                    }
                }
                else {
                    break;
                }

                offset += dir;
            }
        }
    }


    submit_move_data (move: MoveData) {
        // Tjek om
        // 1. Om trækket bliver opgraderet. ✅
        // 1. Hvis ens egen konge er i skak efter trækket, så er trækket jo sjovt nok ikke gyldigt.


        // 1. tilføjer upgrade_to som metadata til trækket.

        if (move.piece == "PAWN" && move.upgrade_to === undefined) {
            if (["1", "8"].includes (move.to[1])) {
                for (const piece of ["QUEEN", "ROOK", "BISHOP", "KNIGHT"]) {
                    this.submit_move_data ({
                        from: move.from,
                        to: move.to,
                        piece: move.piece,
                        capture: move.capture,
                        is_check: move.is_check,
                        is_checkmate: move.is_checkmate,
                        upgrade_to: piece as Piece,
                    });
                }
                return false;
            }
        }


        // Tjek om min egen konge er sat i skak efter trækket.
        // Der er to ting der kan ske!
            // 1. Brikkens nye placering har efterladt et hul i forsvaret, som nu bruges.
            // 2. Der er allerede en trussel, men det nye træk gør ingenting for at forsvare kongen.
            //    Bare kig i den retning og se om vi støder ind i noget.

        let my_king_square = this.board.get_piece ("KING", helpers.convert_player_color_to_color (this.turn));
        let threats_detected = false;

        //* gå i gennem eksisterende trusler
        for (let threat of this.known_threats[this.turn]) {
            console.log("THREAT!", threat)
        }
        


        //* kig i mod hullet
        let dir = helpers.calc_dir_towards_square (my_king_square, move.from);
        
        this.calc_sliding_moves (squares.indexOf (my_king_square), [dir], 8, (move: MoveData, len: number) => {
            if (!move.capture) {
                return false;
            }

            console.log (move);
        });

        console.log ("--------------------------------\n")


        //* Tjek om trækket sætter modstanderen i skak.

        let opponent_king_square = this.board.get_piece ("KING", helpers.convert_player_color_to_color (this.turn));
        let dir_against_old = helpers.calc_dir_towards_square (opponent_king_square, move.from);
        let dir_against_new = helpers.calc_dir_towards_square (opponent_king_square, move.to);
        // TODO: tjek om en hest har sat kongen i skak.

        this.calc_sliding_moves (squares.indexOf (my_king_square), [dir], 8, (move: MoveData, len: number) => {
            if (!move.capture) {
                return false;
            }

            console.log (move);
        });



        // 2. tjekker om trækket sætter modstanderens konge i skak.
        // this.board.update_ext_board ();
        // this.board.ext_board?.move (move, this.turn);

        // let king_square = this.board.get_piece ("KING", this.turn == "WHITE" ? "BLACK" : "WHITE");

        // let king_i8 = squares.indexOf (king_square);
        // let from_i8 = squares.indexOf (move.from);
        // let to_i8 = squares.indexOf (move.to);

        // let towards_old_pos_i12 = 0;
        // let towards_new_pos_i12 = 0;

        // if (move.upgrade_to == "KNIGHT" || move.piece == "KNIGHT") {
        //     let knight_avail_squares: Square[] = this.board.calc_knight_available_squares (to_i8);

        //     // så kan hesten fra dens nye position få fat i kongen
        //     move.is_check = knight_avail_squares.includes (king_square);
        // }
        // else {
        //     this.calc_sliding_moves (king_i8, towards_new_pos_i12, (move: MoveData) => {

        //     })
        // }



        // this.board.ext_board?.log ();


        this.raw_moves.push (move);
    }



    concentrate_moves () {
        // key: to, val: Move
        const col_moves: { [ key: string ]: MoveData[] } = {}


        // Sorter efter brik
        for (const move of this.raw_moves) {
            if (move.piece in col_moves) {
                col_moves[move.piece].push(move)
            }
            else {
                col_moves[move.piece] = [move]
            }
        }


        for (const piece of Object.keys(col_moves)) {
            let moves = col_moves[piece];
            // Sorter efter destination.
            const to_col: { [ key: string ]: MoveData[] } = {};

            for (let move of moves) {
                if (move.to in to_col) {
                    to_col[move.to].push(move);
                }
                else {
                    to_col[move.to] = [move];
                }
            }

            // kører over alle moves sorteret efter destination
            for (let dest of Object.keys(to_col)) {
                let moves = to_col[dest];

                // -||-
                for (let move of moves) {
                    let from_prefix = moves.length == 1 ? "":move.from[0];
                    if (moves.length > 1) {
                        for (let _m of moves) {
                            if (_m.to == move.to && _m.from == _m.from) {
                                continue;
                            }

                            if (_m.from[0] == move.from[0] || _m.from[1] == move.from[1]) {
                                from_prefix = move.from;
                            }
                        }
                    }
                    let piece_name_prefix = piece == "PAWN" ? "" : piece == "KNIGHT" ? "N":piece[0];
                    
                    let move_name = `${from_prefix}${piece_name_prefix}${move.to}`;
                    this.available_moves[move_name] = move;
                }
            }
        }
    }



    moves () {
        // return Object.keys (this.available_moves);
        this.concentrate_moves ();
        return this.available_moves;
    }


    log () {
        this.board.log (this.turn, this.moves_made, Object.keys (this.available_moves).length, this.en_passant);
    }




    // ugly implementations

    load_fen (fen: string) {
        this.board.clear_board ();
        load_fen (fen, this);
        this.board.update_ext_board ();
    }
}


