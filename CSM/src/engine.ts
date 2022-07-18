import { Chess } from "./board";




class Engine {
    chess = new Chess;

    constructor(fen: string = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR") {
        this.chess.load_fen(fen);
    }
}













if (process.argv[2] == "test") {
    // let { chess } = new Engine(); // normal
    // let { chess } = new Engine("r3k2r/8/8/8/8/3p4/8/R3K2R b") //PPPPPPP/RNBQKBNR"); // hvid, ingen modstander
    let { chess } = new Engine("r3k2r/8/b7/8/8/3p4/8/R4K1R b")
    // let { chess } = new Engine(); // sort, ingen modstander
    // let { chess } = new Engine(); // normal
    // let { chess } = new Engine(); // normal
    // let { chess } = new Engine(); // normal

    chess.log();
    chess.gen();
    console.log( chess.moves() )
}