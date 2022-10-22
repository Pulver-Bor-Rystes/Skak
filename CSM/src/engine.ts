import { Chess } from "./chess";

console.clear ();
console.log ("\n\n  -----------------------------------------------\n\n")

const chess = new Chess;

// chess.load_pgn ("r3k2r/8/b7/8/8/3n4/4K3/R41R1 b");

// chess.load_fen ("rk1K3R/8/8/8/8/8/8/8");

chess.load_fen ("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
// chess.load_fen ("p2qkbnr/1P1ppppp/8/8/8/8/PPPPPPPP/RNBQKBNR");
// chess.load_fen ("1r2kbnr/4pppp/8/8/8/8/4PPPP/R3KBNR");
// chess.load_fen ("6kr/5ppp/8/8/8/8/5PPP/R4RK1 w - - 1 1")


// chess.load_pgn("1. a4 a5 2. b4 b5 3. c4 Nc6 4. cxb5 Nxb4 5. Ra2");

chess.gen()
    .move("a4")
    .move("Nf6")
    
// console.log (chess.legal_moves.filter ((m) => m.piece == "BISHOP"))

chess.log ();
// console.log(chess.legal_moves)
console.log (chess.moves)
