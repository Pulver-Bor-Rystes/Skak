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
export const x12_to_x8: { [key: x12_index]: x8_index } = {26: 0, 27: 1, 28: 2, 29: 3, 30: 4, 31: 5, 32: 6, 33: 7, 38: 8, 39: 9, 40: 10, 41: 11, 42: 12, 43: 13, 44: 14, 45: 15, 50: 16, 51: 17, 52: 18, 53: 19, 54: 20, 55: 21, 56: 22, 57: 23, 62: 24, 63: 25, 64: 26, 65: 27, 66: 28, 67: 29, 68: 30, 69: 31, 74: 32, 75: 33, 76: 34, 77: 35, 78: 36, 79: 37, 80: 38, 81: 39, 86: 40, 87: 41, 88: 42, 89: 43, 90: 44, 91: 45, 92: 46, 93: 47, 98: 48, 99: 49, 100: 50, 101: 51, 102: 52, 103: 53, 104: 54, 105: 55, 110: 56, 111: 57, 112: 58, 113: 59, 114: 60, 115: 61, 116: 62, 117: 63 };


export const diagonals = [11, 13, -11, -13];
export const lateral = [12, -12, 1, -1];
export const horse = [25, 23, -25, -23, 14, 10, -14, -10];


export interface Move {
    from: Square
    to: Square
    piece: Piece
    capture?: Piece
    is_check?: boolean
    is_checkmate?: boolean
    upgrade_to?: Piece
    extra_move?: Move
}




export class Helper {

    static copy_into (from: any[]) {
        let to = [];
        for (let i = 0; i < from.length; i++) {
            to.push (from[i]);
        }
        return to;
    }

    static convert_square_to_i12 (square: Square) {
        let i8 = squares.indexOf (square);
        return Helper.convert_x8_to_x12 (i8);
    }


    static opposite_turn (turn: PlayerColor) {
        return turn == "WHITE" ? "BLACK" : "WHITE";
    }


    static convert_castle_right_to_square(color: PlayerColor, _i: 0 | 1) {
        const convert_chart: {[key: string]: Square} = {
            "WHITE0": "a1",
            "WHITE1": "h1",
            "BLACK0": "a8",
            "BLACK1": "h8",
        }

        return convert_chart[color+_i];
    }


    static get_left_over_en_passant_square(en_passant: Square): Square {
        let num = Number(en_passant[1]);
        let new_num = num == 3 ? 4:5;

        return (en_passant[0] + new_num) as Square;
    }

    static get_square (index: x12_index) {
        return squares[Helper.convert_x12_to_x8 (index)];
    }


    static square_to_i12 (square: Square) {
        let i8 = squares.indexOf(square);
        return Helper.convert_x8_to_x12 (i8);
    }


    static default_x (lim: number = 12): Color[] | Piece[] {
        const colors: Color[] = [];
        for (let i = 0; i < (lim*lim); i++) {
            colors.push("EMPTY");
        }
        return colors;
    }
    

    static convert_player_color_to_color (pl_col: PlayerColor, reverse = false) {
        if (reverse) {
            return pl_col == "WHITE" ? "DARK" : "LIGHT";
        }
        
        return pl_col == "WHITE" ? "LIGHT":"DARK";
    }


    static convert_piece_letter_to_piece (piece_letter: string): Piece {
        piece_letter = piece_letter.toLowerCase ();
        
        let convert_chart: { [key: string]: Piece } = {
            "n": "KNIGHT",
            "k": "KING",
            "q": "QUEEN",
            "b": "BISHOP",
            "r": "ROOK",
            "p": "PAWN",
            "*": "EMPTY",
        }

        if (!(piece_letter in convert_chart)) {
            throw "PIECE LETTER WAS NOT VALID!";
        }

        return convert_chart[piece_letter];
    }

    static convert_piece_to_piece_letter (piece: Piece): string {
        let convert_chart: { [key: string]: string } = {
           "KNIGHT": "n",
           "KING": "k",
           "QUEEN": "q",
           "BISHOP": "b",
           "ROOK": "r",
           "PAWN": "p",
           "EMPTY": "*",
        }

        if (!(piece in convert_chart)) {
            throw "PIECE WAS NOT VALID!";
        }

        return convert_chart[piece]; 
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

        dir += Number (towards[1]) > Number (from[1]) ? -1:1;
        dir += square_letters.indexOf (towards[0] as Square_Letter) > square_letters.indexOf (from[0] as Square_Letter) ? 12:-12;

        return dir;
    }
}