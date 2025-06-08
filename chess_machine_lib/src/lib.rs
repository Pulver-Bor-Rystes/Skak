use chess::chess_types::{ChessBoard, ChessColor, Index144, NamingConvention};
use wasm_bindgen::prelude::*;
use std::cell::RefCell;
pub mod chess;
pub mod extra;

thread_local! {
    static BOARDS: RefCell<Vec<chess::chess_types::ChessBoard>> = RefCell::new(Vec::new());
}


#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}



#[wasm_bindgen]
pub fn new_chessboard(fen_str: &str) -> usize {
    let mut index = 0;
    
    BOARDS.with(|boards| {
        let new_board = ChessBoard::default()
            .load_fen(fen_str)
            .set_naming_convention(NamingConvention::LongAlgebraicNotation)
            .to_owned();
        
        // gemmer skakbræt
        boards.borrow_mut().push(new_board);

        index = boards.borrow().len() - 1;
    });

    return index;
}

#[wasm_bindgen]
pub fn load_fen(board_id: usize, fen_str: &str) {
    BOARDS.with(|boards| {
        let board = &mut boards.borrow_mut()[board_id];

        board.load_fen(fen_str);
    });
}

#[wasm_bindgen]
pub fn play_move(board_id: usize, move_name: &str) {
    BOARDS.with(|boards| {
        let board = &mut boards.borrow_mut()[board_id];

        board.play_notation(move_name);
    });
}


#[wasm_bindgen]
pub fn get_pieces(board_id: usize) -> Vec<String> {
    let mut pieces = Vec::new();
    
    
    BOARDS.with(|boards| {
        let board = &boards.borrow()[board_id];


        for i in 0..64 {
            let piece = board.get(Index144::from8(i));

            match piece {
                None => pieces.push("*".to_string()),
                Some(piece) => pieces.push(piece.to_str_img_format()),
            }
        }
    });

    return pieces;
}


#[wasm_bindgen]
pub fn get_moves(board_id: usize) -> Vec<String> {
    let mut moves: Vec<String> = Vec::new();

    BOARDS.with(|boards| {
        let board = &boards.borrow()[board_id];

        for chess_move in &board.moves {
            moves.push(chess_move.name.clone());
        }
    });

    moves
}



#[wasm_bindgen]
pub fn get_destinations(board_id: usize, color: bool, from: i32) -> Vec<i32> {
    let mut moves: Vec<i32> = Vec::new();

    BOARDS.with(|boards| {
        let board = &boards.borrow()[board_id];

        let mut stop = false;

        match (board.turn, color) {
            (ChessColor::White, false) => stop = true,
            (ChessColor::Black, true) => stop = true,
            _ => {},
        }
        

        if !stop {
            for chess_move in &board.moves {
                if chess_move.from() != Index144::from8(from) { continue }
    
                moves.push(chess_move.to().i8());
            }
        }
    });

    moves
}

#[wasm_bindgen]
pub fn get_move_name(board_id: usize, from: i32, to: i32) -> String {
    let mut name = String::new();

    BOARDS.with(|boards| {
        let board = &boards.borrow()[board_id];

        for chess_move in &board.moves {
            if chess_move.from() != Index144::from8(from) { continue }
            if chess_move.to() != Index144::from8(to) { continue }

            name = chess_move.name.clone();
        }
    });

    name
}
