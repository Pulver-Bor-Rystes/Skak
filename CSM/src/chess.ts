import { Board } from "./board";
import { diagonals, Helper, horse, lateral, Move, Piece, PlayerColor, Square, squares, x12_index } from "./helpers";

export class Chess {
    private board = new Board;
    private fake = new Board;

    turn: PlayerColor = "WHITE";
    pseudo_moves: Move[] = [];
    legal_moves: Move[] = [];
    compressed_moves: { [key: string]: Move } = {};

    en_passant: Square | "" = "";
    half_move_clock = 0;
    full_move_clock = 0;

    pgn_string: string = "";
    
    
    // castle rights
    castle_rights: { [key: string]: [boolean, boolean] } = {
        "WHITE": [false, false],
        "BLACK": [false, false],
    }
    

    on_win: ((pcolor: PlayerColor) => void) | undefined;
    on_draw: (() => void) | undefined;


    debiance = true;


    constructor (on_win?: (pcolor: PlayerColor) => void, on_draw?: () => void) {
        this.on_win = on_win;
        this.on_draw = on_draw;
    }




    load_fen (str: string) {
        // der er forskellige dele
        let diff_parts = str.split(" ");

        let p_placement = diff_parts[0] || "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
        let p_turn = diff_parts[1] || "w";
        let p_castle = diff_parts[2] || "QKqk";
        let p_en_passant = diff_parts[3] || "-";
        let p_half = diff_parts[4] || "0";
        let p_full = diff_parts[5] || "1";

        // 1. Piece placement
        let pieces = p_placement;

        let backslashes = 0;
        let x = 0;
        let i = 0;

        for (let piece of pieces) {

            if (piece == "/") {
                if (x < 7 || x > 8) {
                    throw "NOT ENOUGH PLACED BETWEEN SLASHES!";
                }

                x = 0;
                backslashes ++;
            }
            else {
                let num = Number (piece);
                if (String(num) != "NaN") {
                    x += num;
                    i += num;
                }
                else {
                    let i12 = Helper.convert_x8_to_x12 (i);
    
                    this.board.colors[i12] = piece == piece.toLowerCase () ? "DARK":"LIGHT";
                    this.board.pieces[i12] = Helper.convert_piece_letter_to_piece (piece);
                    
                    i ++;
                    x ++;
                }
            }
        }

        if (backslashes != 7) {
            throw "NOT ENOUGH ROWS!";
        }


        // 2. Active color
        if (["w", "b"].includes (p_turn)) {
            this.turn = p_turn == "w" ? "WHITE":"BLACK";
        }
        else {
            throw "NOT VALID TURN"
        }


        // 3. Castle rights
        if (p_castle != "-") {
            let convert_chart: { [key: string]: any } = {
                "iq": 0,
                "ik": 1,
                "Q": "WHITE",
                "K": "WHITE",
                "q": "BLACK",
                "k": "BLACK",
            }


            let ri = 0
            for (let right of p_castle) {
                let color = convert_chart[ right ];

                
                let king_square: Square = ri == 0 ? "e1":"e8";

                if (this.board.is_piece_on_square ("KING", king_square)) {
                    let _i = convert_chart[ "i"+right.toLowerCase() ];
    
                    const square = Helper.convert_castle_right_to_square (color, _i);
    
                    this.castle_rights[color][_i] = this.board.is_piece_on_square ("ROOK", square)
                }
                else {
                    this.castle_rights[color] = [false, false];
                }

                ri ++;
            }
        }
        else {
            this.castle_rights["WHITE"] = [false, false];
            this.castle_rights["BLACK"] = [false, false];
        }

        
        // 4. En passant
        if ((!squares.includes (p_en_passant as Square)) && p_en_passant != "-") {
            throw "NOT A VALID EN PASSANT SQUARE";
        }
        else {
            this.en_passant = (p_en_passant == "-" ? "" : p_en_passant) as Square;
        }


        // 5. Half move clock
        let half = Number (p_half);
        if (String (half) == "NaN") {
            throw "NOT A VALID HALF MOVE NUMBER";
        }
        else {
            this.half_move_clock = half;
        }


        let full = Number (p_full);
        if (String (full) == "NaN") {
            throw "NOT A VALID FULL MOVE NUMBER";
        }
        else {
            this.full_move_clock = full;
        }

        return this;
    }


    load_pgn (pgn_string: string) {
        let parts = pgn_string.split (". ");
        parts.shift()
        console.log(parts)
        this.load_fen ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");

        this.gen ();

        for (const moves of parts) {
            for (let move of moves.split (" ")) {
                if (move in this.compressed_moves) {
                    this.move (move);
                }
            }
        }

        return this;
    }


    gen () {
        this.gen_pseudo ();
        this.pick_legal ();


        this.remove_undefined_properties (this.pseudo_moves);
        this.concentrate_moves ();

        //! DEBUG
        // console.log (this.legal_moves);
        // console.log (this.legal_moves.length);

        return this;
    }

    


    gen_pseudo (piece_filter: Piece | '*'='*') {
        this.fake.copy_from_board (this.board);


        // gå igennem alle brikker
        this.board.get_all_pieces (
            piece_filter, 
            Helper.convert_player_color_to_color (this.turn), 
            (
                square: Square, 
                index: x12_index, 
                piece: Piece
            ) => {
                let directions: x12_index[] = [];
                if (piece == "KNIGHT") directions = horse;
                if (piece == "ROOK") directions = lateral;
                if (piece == "BISHOP") directions = diagonals;
                if (piece == "QUEEN") directions = diagonals.concat (lateral);

                switch (piece) {
                    case "PAWN":
                        let limit = 1;
                        limit = this.turn == "WHITE" && square[1] == "2" ? 2:limit;
                        limit = this.turn == "BLACK" && square[1] == "7" ? 2:limit;
            
                        let direction = this.turn == "WHITE" ? -12:12;
            
            
                        // Forward
                        this.fake.calc_moves_in_dirs (index, [direction], limit, (move: Move) => {
                            if (move.capture) {
                                return true; // Stop med at søge!
                            }
            
                            this.upgrade_logic (move);
                        })
                        
            
                        // Capture
                        this.fake.calc_moves_in_dirs (index, [direction-1, direction+1], 1, (move: Move) => {
                            if (!move.capture) {
                                if (move.to == this.en_passant) {
                                    move.capture = "PAWN";
                                    move.extra_move = {
                                        from: Helper.get_left_over_en_passant_square (this.en_passant),
                                        to: Helper.get_left_over_en_passant_square (this.en_passant),
                                        piece: "EMPTY",
                                    }
                                }
                                else {
                                    return; // Fortsæt med at søge
                                }
                            }

                            this.upgrade_logic (move);
                        });
                        break;
                
                    case "KING":
                        this.fake.calc_moves_in_dirs (index, lateral.concat (diagonals), 1, (move: Move) => {
                            this.pseudo_moves.push (move);
                        })

                        // KONGE ROKADE!
                        let rights = this.castle_rights[this.turn];

                        
                        let spaces = [
                            [1, 2],
                            [-1, -2, -3],
                        ]
                        
                        let _i = 0;
                        for (let space of spaces) {
                            if (rights[_i]) {
                                if (this.board.is_empty (index, space)) {
                                    const move: Move = {
                                        from: square,
                                        to: Helper.get_square (index + space[1]),
                                        piece: "KING",
                                        extra_move: {
                                            from: Helper.get_square (index + ( Math.abs(space[space.length-1]) + 1 )*space[0] ),
                                            to: Helper.get_square (index + space[0]),
                                            piece: "ROOK",
                                        }
                                    }

                                    this.pseudo_moves.push (move);
                                }
                            }
                                
                            _i ++;
                        }

                        break;


                    default:
                        this.pseudo_moves = this.pseudo_moves.concat (
                            this.fake.calc_moves_in_dirs (index, directions)
                        )
                        break;
                }
            }
        )

        return this;
    }


    pick_legal () {
        // jeg vil gå igennem alle træk og prøve dem af på brættet for at se om de kan lade sig gøre

        for (const move of this.pseudo_moves) {
            if (!this.debiance) {
                // this.fake.log()
                // console.log (move)
            }


            this.fake.copy_from_board (this.board);
            this.fake.move (move, this.turn);

            if (!this.debiance) {
                // this.fake.log()
            }

            // 1. først og fremmest tjek om trækket sætter ens egen konge i skak!

            let [_, mki, __] = this.fake.get_pieces ("KING", this.turn)[0];
            let is_in_danger = this.fake.is_square_in_danger (mki);
            if (is_in_danger) continue;

            // a. Tjek om tårnet i kongerokaden er sat i skak
            if (move.extra_move?.piece == "ROOK") {
                let i = Helper.convert_x8_to_x12 (squares.indexOf (move.extra_move.to));
                let is_rook_in_danger = this.fake.is_square_in_danger (i);

                if (is_rook_in_danger) continue;
            }

            // 2. tjekker om trækket sætter modstanderen i skak. Udelukkende pga meta data!

            

            let [___, oki, ____] = this.fake.get_pieces ("KING", Helper.opposite_turn (this.turn))[0];
            let opp_is_in_danger = this.fake.is_square_in_danger (oki);
            
            
            move.is_check = opp_is_in_danger;

            if (opp_is_in_danger && this.debiance) {
                this.debiance = false;
                // TODO: Så skal vi tjekke om der er skakmat!

                //* Save
                const cp_pseudo = Helper.copy_into (this.pseudo_moves);
                const cp_legal = Helper.copy_into (this.legal_moves);
                const cp_board = new Board;
                cp_board.copy_from_board (this.board);
                this.pseudo_moves = [];
                this.legal_moves = [];

                this.board.move (move, this.turn);

                // Reverse
                this.turn = Helper.opposite_turn (this.turn);
                

                //* Action!
                this.gen ();
                if (this.legal_moves.length == 0) {
                    move.is_checkmate = true;
                }


                //* Load
                this.pseudo_moves = cp_pseudo;
                this.legal_moves = cp_legal;
                this.board.copy_from_board (cp_board);
                this.turn = Helper.opposite_turn (this.turn);
                this.debiance = true;
            }

            this.legal_moves.push (move);
        }

        return this;
    }

    


    concentrate_moves () {
        // key: to, val: Move
        const col_moves: { [ key: string ]: Move[] } = {}


        // Sorter efter brik
        for (const move of this.legal_moves) {
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
            const to_col: { [ key: string ]: Move[] } = {};

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

                    if (move.piece == "PAWN" && move.capture && from_prefix == "") {
                        from_prefix = move.from[0];
                    }
                    
                    let piece_name_prefix = piece == "PAWN" ? "" : piece == "KNIGHT" ? "N":piece[0];
                    
                    let move_name = `${ from_prefix}${piece_name_prefix}${move.capture ? 'x':''}${move.to}`;

                    if (move.extra_move?.piece == "ROOK") {
                        move_name = move.to.includes("c") ? "O-O-O":"O-O";
                    }

                    if (move.is_check) {
                        move_name += move.is_checkmate ? '#':'+';
                    }

                    this.compressed_moves[move_name] = move;
                }
            }
        }

        return this;
    }


    
    
    log (fake=false) {
        if (fake) {
            this.fake.log ();
        }
        else {
            this.board.log ();
            console.log ("turn:", this.turn);
            console.log ("en_passant:", this.en_passant);
            // console.log ("castle rights: ", this.castle_rights);
            // console.log ("pgn:", this.pgn_string);
        }

        return this;
    }


    move (move_name: string) {
        if (!(move_name in this.compressed_moves)) {
            throw "NO SUCH MOVE";
        }

        const move = this.compressed_moves[move_name];

        let res = this.board.move (move, this.turn);

        // Opdaterer en passant
        if (res && squares.includes (res as Square)) {
            this.en_passant = res as Square;
        }
        else {
            this.en_passant = "";
        }


        if (!move.capture || move.piece != "PAWN") {
            this.half_move_clock ++;
            if (this.on_draw) {
                this.on_draw ();
            }
        }
        else {
            this.half_move_clock = 0;
        }

        if (move.extra_move?.piece == "ROOK" || move.piece == "KING") {
            this.castle_rights[this.turn][0] = false;
            this.castle_rights[this.turn][1] = false;
        }

        if (move.piece == "ROOK") {
            if (move.from.includes ("a")) {
                this.castle_rights[this.turn][0] = false;
            }
            if (move.from.includes ("h")) {
                this.castle_rights[this.turn][1] = false;
            }
        }


        // Notér pgn
        if (this.pgn_string == "" || this.turn == "WHITE") {
            this.pgn_string += ` ${this.full_move_clock}.`
        }

        this.pgn_string += ` ${move_name}`


        // Skift tur
        this.turn = Helper.opposite_turn (this.turn);
        if (this.turn == "WHITE") {
            this.full_move_clock ++;
        }


        this.pseudo_moves = [];
        this.legal_moves = [];
        this.compressed_moves = {};

        
        
        
        if (!move.is_checkmate) {
            this.gen ();
        }
        else {
            if (this.on_win) {
                this.on_win (this.turn);
            }
        }

        return this;
    }

    get pgn () {
        return this.pgn_string;
    }

    get moves () {
        return Object.keys (this.compressed_moves);
    }
    
    
    
    private remove_undefined_properties (objs: any[]) {
        for (let obj of objs) {
            for (const key of Object.keys (obj)) {
                if (!obj[key]) {
                    delete obj[key];
                }
            }
        }
    }

    private upgrade_logic (move: Move) {
        this.pseudo_moves.push (move);
        
        if (move.piece == "PAWN" && move.upgrade_to === undefined) {
            if (["1", "8"].includes (move.to[1])) {
                for (const piece of ["QUEEN", "ROOK", "BISHOP", "KNIGHT"]) {
                    this.pseudo_moves.push ({
                        from: move.from,
                        to: move.to,
                        piece: move.piece,
                        capture: move.capture,
                        is_check: move.is_check,
                        is_checkmate: move.is_checkmate,
                        upgrade_to: piece as Piece,
                    });
                }
                return true;
            }
        }
    }
}