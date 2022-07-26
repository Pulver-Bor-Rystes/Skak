import { Color, diagonals, Helper, horse, lateral, Move, Piece, PlayerColor, Square, squares, square_letters, x12_index, x12_valid_indexes } from "./helpers";



export class Board {
    colors = Helper.default_x () as Color[];
    pieces = Helper.default_x () as Piece[];
    
    
    copy_from_board (board: Board) {
        for (let i = 0; i < 12*12; i++) {
            this.colors[i] = board.colors[i];
            this.pieces[i] = board.pieces[i];
        }
    }
    
    
    get_all_pieces (type: Piece | "*", turn_color: Color | "*", callback: (square: Square, i: x12_index, piece: Piece) => void) {
        
        for (let i = 0; i < 12*12; i++) {
            let color = this.colors[i];
            let piece = this.pieces[i];
            
            if (turn_color != "EMPTY" && turn_color != color && turn_color != "*") {
                continue;
            }
            
            
            if (type != "EMPTY" && type != piece && type != "*") {
                continue;
            }
            
            let square = squares[Helper.convert_x12_to_x8 (i)];
            
            callback (square, i, piece);
        }
    }



    calc_moves_in_dirs (index: x12_index, directions?: x12_index | x12_index[], limit=8, callback?: (move: Move, dir: x12_index, travel_length: number) => boolean | void) {
        if (directions == undefined) {
            directions = lateral.concat (diagonals).concat (horse);
        }
        else if (typeof directions == "number") {
            directions = [directions];
        }


        const real_limit = limit;
        let pseudo_moves: Move[] = [];

        for (const dir of directions) {
            limit = horse.includes (dir) ? 1 : real_limit;

            let offset = dir;

            while (limit--) {
                const move = this.is_move_possible (index, offset);

                if (!move) {
                    break;
                }

                if (callback) {
                    let res = callback (move, dir, real_limit-limit);

                    if (res) {
                        break;
                    }
                }
                else {
                    pseudo_moves.push (move);
                }

                // Hvis vi fanger noget, så kan den jo ikke bare fortsætte
                if (move.capture) {
                    break;
                }


                offset += dir;
            }
        }

        return pseudo_moves;
    }




    move (move: Move, turn: PlayerColor) {
        // Fjerner den gamle brik
        this.colors [Helper.square_to_i12 (move.from)] = "EMPTY";
        this.pieces [Helper.square_to_i12 (move.from)] = "EMPTY";


        // Placerer brikken på dens nye position
        this.colors [Helper.square_to_i12 (move.to)] = Helper.convert_player_color_to_color (turn);
        this.pieces [Helper.square_to_i12 (move.to)] = move.piece;

        // returnér en passant
        if (move.piece == "PAWN") {
            if (move.from[1] == "2" && move.to[1] == "4") {
                return `${move.from[0]}3`;
            }
            else if (move.from[1] == "7" && move.to[1] == "5") {
                return `${move.from[0]}6`;
            }
        }


        if (move.extra_move) {
            this.move (move.extra_move, turn);
        }
    }


    is_move_possible (index: x12_index, offset: x12_index): Move | false {
        // Tjek om trækket lander udenfor brættet
        if (!x12_valid_indexes.includes (index + offset)) {
            return false;
        }

        // Tjek om destinationen er på en egen farve.
        if (this.colors[index] == this.colors[index + offset]) {
            return false;
        }
        

        // Ellers er det et gyldigt træk!
        const from = squares[Helper.convert_x12_to_x8 (index)]
        const to = squares[Helper.convert_x12_to_x8 (index + offset)]


        const pot_capture = this.pieces[index + offset];

        const move: Move = {
            from,
            to,
            capture: pot_capture == "EMPTY" ? undefined : pot_capture,
            piece: this.pieces[index], // TODO: dejligt for debug, men er vel egentlig lidt ligegyldigt metadata?
        }

        return move;
    }




    is_square_in_danger(index: x12_index) {
        let threat_detected = false;

        this.calc_moves_in_dirs (index, undefined, 8, (move, dir, travel_length) => {
            if (!move.capture) return;

            let opponent = move.capture;
            let convert_chart: {[key: string]: x12_index[]} = {
                "KNIGHT": horse,
                "ROOK": lateral,
                "BISHOP": diagonals,
                "PAWN": diagonals,
                "QUEEN": diagonals.concat (lateral),
                "KING": diagonals.concat (lateral),
            }


            let allowed_directions = convert_chart[opponent];
            if (allowed_directions.includes(dir)) {
                if (["PAWN", "KING", "KNIGHT"].includes (opponent)) {
                    if (travel_length != 1) {
                        return;
                    }
                }
            }
            else {
                return;
            }

            threat_detected = true;
        })

        return threat_detected;
    }
    
    
    
    is_empty (start_index: x12_index, offsets: x12_index[]) {
        for (const offset of offsets) {
            if (this.pieces[start_index + offset] != "EMPTY") {
                return false;
            }
        }

        return true;
    }

    get_pieces (piece: Piece, color: PlayerColor) {
        let pieces: [Square, number, Piece][] = [];
        this.get_all_pieces (piece, Helper.convert_player_color_to_color (color), (square, i, piece) => {
            pieces.push ([square, i, piece]);
        })

        return pieces;
    }


    is_piece_on_square (piece: Piece, square: Square) {
        return this.pieces[Helper.convert_square_to_i12 (square)] == piece;
    }


    log (mark_square?: Square) {
        let y = -1;
        let x = 8;
        
        for (let i = 0; i < 12*12; i++) {
            if (!x12_valid_indexes.includes (i)) {
                continue;
            }
            
            if (++x >= 8) {
                y ++;
                x = 0;
                process.stdout.write (`\n ${8-y}|`);
            }


            let piece_name = this.pieces[i] == "KNIGHT" ? "N":this.pieces[i][0];
            let correct_piece_name = this.colors[i] == "LIGHT" ? piece_name : piece_name.toLowerCase();

            if (mark_square == squares[Helper.convert_x12_to_x8(i)]) {
                correct_piece_name = "X";
            }
            process.stdout.write (` ${ correct_piece_name.toLowerCase() == "e" ? "*":correct_piece_name }`);
        }

        process.stdout.write ("\n    ---------------")
        process.stdout.write ("\n    a b c d e f g h")

        process.stdout.write ("\n");
    }
}