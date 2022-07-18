import { Color, Piece, Square, squares } from "./board";

export function load_fen(fen: string, THIS: any) {
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
        THIS.turn = parts_space[1].toLowerCase() == "b" ? "BLACK":"WHITE";
    }

    if (parts_space[2]) {
        let rights = parts_space[2];

        if (rights == "-") {
            for (let i of [0, 1]) {
                for (let key of Object.keys(THIS.has_moved)) {
                    // @ts-ignore
                    THIS.has_moved[key][i] = true;
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
                THIS.has_moved[key][i] = false;
            }
        }
    }

    if (parts_space[3]) {
        if (parts_space[3] != "-") {               
            THIS.en_passant = parts_space[3].toLowerCase();
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
                        
                        THIS.board.colors[index] = color;
                        THIS.board.pieces[index] = piece;
                        


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