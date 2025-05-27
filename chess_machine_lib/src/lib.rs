use chess::chess_types::{ChessBoard, ChessColor, Index144, Move};
use wasm_bindgen::prelude::*;
pub mod chess;
pub mod extra;

// Make this function callable from JavaScript.
#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hvad så taber, {}!", name)
}


use std::cell::RefCell;

thread_local! {
    static BOARDS: RefCell<Vec<chess::chess_types::ChessBoard>> = RefCell::new(Vec::new());
}




#[wasm_bindgen]
pub fn new_chessboard(fen_str: &str) -> usize {
    let mut index = 0;
    
    BOARDS.with(|boards| {
        boards.borrow_mut().push(ChessBoard::from_fen(fen_str));

        index = boards.borrow().len() - 1;
    });

    return index;
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
