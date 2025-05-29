use std::time::{Duration, Instant};

use crate::extra::{index_144_to_64, index_64_to_144, index_64_to_algebraic};


pub mod piece_data;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index144(i32);

pub enum BoardType { Regular, Large }

#[derive(Clone)]
pub enum NamingConvention { Standard, LongAlgebraicNotation }


const INVALID_INDEXES: [i32; 80] = [9, 83, 84, 135, 141, 120, 138, 24, 48, 85, 123, 139, 7, 121, 6, 136, 25, 35, 142, 106, 128, 22, 70, 124, 34, 4, 5, 23, 71, 20, 127, 2, 37, 109, 129, 122, 143, 3, 107, 36, 140, 73, 0, 21, 125, 14, 18, 132, 72, 133, 96, 108, 19, 130, 12, 16, 1, 95, 15, 126, 59, 131, 97, 119, 58, 17, 10, 61, 118, 46, 94, 13, 137, 47, 82, 8, 60, 134, 49, 11 ];
const LAST_ROWS: [i32; 16] = [26, 27, 28, 29, 30, 31, 32, 33, 110, 111, 112, 113, 114, 115, 116, 117];


impl Index144 {
    // Default

    pub fn from_minus_one() -> Self {
        let mut i = Self(0);
        i.set_8(0);
        i.0 -= 1;
        return i;
    }

    pub fn from12(v: i32) -> Self {
        Self(v)
    }

    pub fn from8(v: i32) -> Self {
        Self(index_64_to_144(v))
    }


    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_algebraic(square: &str) -> Self {
        // let bytes = square.as_bytes();
        // let file = bytes[0];
        // let rank = bytes[1];

        // if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
        //     panic!("Invalid algebraic notation");
        // }

        // let file_index = (file - b'a') as i32;
        // let rank_index = 7 - (rank - b'1') as i32;

        // let index_64 = rank_index * 8 + file_index;
        // Self(index_64_to_144(index_64))
        if square.len() != 2 {
            panic!("shiiiit");
        }

        let bytes = square.as_bytes();
        let file = bytes[0];
        let rank = bytes[1];

        if !(b'a'..=b'h').contains(&file) || !(b'1'..=b'8').contains(&rank) {
            panic!("fuuuck");
        }

        let file_index = (file - b'a') as i32;
        let rank_index = 7-(rank - b'1') as i32;

        let index_64 = rank_index * 8 + file_index;

        Self(index_64_to_144(index_64))
    }

    // Setters
    
    pub fn set_12(&mut self, v: i32) -> &mut Self {
        self.0 = v;
        self
    }

    pub fn set_8(&mut self, v: i32) -> &mut Self {
        self.0 = index_64_to_144(v);
        self
    }

    // Incrementers

    pub fn add(&mut self, val: i32) -> &mut Self {
        self.0 += val;
        self
    }

    pub fn up(&mut self, direction: i32) -> &mut Self {
        self.0 += 12 * direction;
        self
    }


    pub fn inc(&mut self, board_type: BoardType) -> &mut Self {
        match board_type {
            BoardType::Regular => {
                self.0 += 1;
                while !self.is_valid() && self.0 < 143 {
                    self.0 += 1;
                }
                self
            },
            BoardType::Large => { self.0 += 1; self },
        }
    }

    pub fn dec(&mut self, board_type: BoardType) -> &mut Self {
        match board_type {
            BoardType::Regular => {
                self.0 -= 1;
                while !self.is_valid() && self.0 > 0 {
                    self.0 -= 1;
                }
                self
            },
            BoardType::Large => { self.0 -= 1; self },
        }
    }


    // Getters

    
    fn file_and_rank(&self) -> (String, String) {
        let from = self.to_str();
        let first = &from[0..1];
        let last = &from[from.len() - 1..];

        (first.to_string(), last.to_string())
    }

    pub fn file(&self) -> String {
        self.file_and_rank().0
    }

    pub fn rank(&self) -> String {
        self.file_and_rank().1
    }
    pub fn is_valid(&self) -> bool {
        !INVALID_INDEXES.contains(&self.i12())
    }

    pub fn is_on_last_row(&self) -> bool {
        LAST_ROWS.contains(&self.0)
    }

    pub fn to_str(&self) -> String {
        if self.is_valid() {
            index_64_to_algebraic(*self)
        }
        else {
            format!("X0({})", self.0)
        }
    }

    pub fn u12(&self) -> usize {
        self.0 as usize
    }

    pub fn i12(&self) -> i32 {
        self.0 as i32
    }

    pub fn u8(&self) -> usize {
        index_144_to_64(self.i12()).unwrap() as usize
    }

    pub fn i8(&self) -> i32 {
        index_144_to_64(self.i12()).unwrap()
    }
}


impl From<i32> for Index144 {
    fn from(v: i32) -> Self {
        Self(v)
    }
}


// Resources
#[derive(Clone)]
pub struct ChessBoard {
    pub pieces: [Option<Piece>; 144],
    pub en_passant: Option<EnPassant>,
    pub turn: ChessColor,

    pub moves: Vec<Move>,
    pub move_history: Vec<Move>,
    
    pub halfmove_clock: i32,
    pub fullmove_number: i32,
    
    // CHANGE DETECTION
    pub board_changed: bool,
    pub turn_changed: bool,

    // META
    pub fen_str: String,
    pub real: bool,
    pub naming_convention: NamingConvention,

    pub winner: Option<Winner>,

    // Time
    pub clock: Clock,
}

#[derive(Clone, Debug)]
pub enum Winner { White, Black, Tie }


#[derive(Clone)]
pub struct Clock {
    pub white: Duration,
    pub black: Duration,

    pub increment: Duration,
    pub since_last_move: Instant,
}


#[derive(Debug, Clone, Copy)]
pub struct EnPassant {
    pub to_attack: Index144,
    pub to_remove: Index144,
}



#[derive(Debug, Clone, PartialEq)]
pub struct Move {
    pub movement: Movement,
    pub promote: Option<Promotion>,
    pub information: MoveInformation,
    pub check: bool,
    pub check_mate: bool,
    pub name: String,
}



impl Move {
    pub fn from(&self) -> Index144 {
        self.movement.from
    }

    pub fn to(&self) -> Index144 {
        self.movement.to
    }

    pub fn set_promotion(&self, promotion: Promotion) -> Self {
        let mut copy = self.clone();

        copy.promote = Some(promotion);
        return copy;
    }

    fn file_and_rank(&self) -> (String, String) {
        let from = self.from().to_str();
        let first = &from[0..1];
        let last = &from[from.len() - 1..];

        (first.to_string(), last.to_string())
    }

    pub fn file(&self) -> String {
        self.file_and_rank().0
    }

    pub fn rank(&self) -> String {
        self.file_and_rank().1
    }

    pub fn make_name(&mut self, chessboard: &ChessBoard, show_file: bool, show_rank: bool) {
        match chessboard.naming_convention {
            NamingConvention::Standard => self.make_name_standard(chessboard, show_file, show_rank),
            NamingConvention::LongAlgebraicNotation => self.make_name_lan(chessboard),
        }
    }

    fn make_name_lan(&mut self, chessboard: &ChessBoard) {
        let capture_flag = match chessboard.get(self.to()).is_some() {
            true => "",
            false => "",
        };

        let promotion_flag = match self.promote {
            Some(promotion) => match promotion {
                Promotion::Queen => "q",
                Promotion::Bishop => "b",
                Promotion::Knight => "n",
                Promotion::Rook => "r",
            },
            None => "",
        };

        // from .. capture? .. to .. promotion
        self.name = format!("{}{}{}{}", self.from().to_str(), capture_flag, self.to().to_str(), promotion_flag);
    }

    fn make_name_standard(&mut self, chessboard: &ChessBoard, show_file: bool, show_rank: bool) {
        // De forskellige flags
        // 1. PieceType
        let piece = chessboard.get(self.from()).unwrap();
        let piece_name = piece.kind.to_str_move_name_format();
        
        // 2. Identifier flag in case of multiple.
        let mut from_flag = match (show_file, show_rank) {
            (true, true) => self.from().to_str(),
            (true, false) => self.file(),
            (false, true) => self.rank(),
            (false, false) => "".to_string(),
        };
        
        // 2. Capture flag
        let mut capture_flag = match chessboard.get(self.to()).is_some() {
            true => "x",
            false => "",
        };

        match chessboard.en_passant {
            Some(EnPassant { to_attack, to_remove: _ }) => {
                if self.to() == to_attack {
                    capture_flag = "x";
                    if &from_flag == "" {
                        from_flag = self.file();
                    }
                }
            },
            None => {},
        }
        
        // 3. Destination
        let destination_flag = self.to().to_str();
        
        // 4. Promotion
        let promotion_flag = match self.promote {
            Some(promotion) => match promotion {
                Promotion::Queen => "=Q",
                Promotion::Bishop => "=B",
                Promotion::Knight => "=N",
                Promotion::Rook => "=R",
            },
            None => "",
        };
        
        // 5. Check og checkmate flag
        let check_flag = match (self.check, self.check_mate) {
            (_, true) => "#",
            (true, false) => "+",
            _ => "",
        };


        // hvad hvis det er casling?

        if piece.kind == PieceType::King {           
            // black king side
            if self.to() == 32.into() {
                self.name = "o-o".to_string();
                return;
            }

            // black queen side
            if self.to() == 28.into() {
                self.name = "o-o-o".to_string();
                return;
            }

            // white king side
            if self.to() == 116.into() {
                self.name = "O-O".to_string();
                return;
            }

            // white queen side
            if self.to() == 112.into() {
                self.name = "O-O-O".to_string();
                return;
            }
        }

        self.name = format!("{}{}{}{}{}{}", piece_name, from_flag, capture_flag, destination_flag, promotion_flag, check_flag);
    }
}





#[derive(Debug, Clone, PartialEq)]
pub struct Movement {
    pub from: Index144,
    pub to: Index144,
}

impl From<(Index144, Index144)> for Movement {
    fn from((from, to): (Index144, Index144)) -> Self {
        Movement { from, to }
    }
}


/// Pawn double move holder på feltet som blev sprunget over
#[derive(Debug, Clone, PartialEq)]
pub enum MoveInformation { None, PawnDoubleMove(Index144), EnPassant, CastleKingSide, CastleQueenSide }



#[derive(Debug, Clone, PartialEq)]
pub struct ProposeMove {
    pub movement: Movement,
    pub requires: Vec<MoveRequirement>,
    pub information: MoveInformation,
}


impl ProposeMove {
    pub fn into_move(&self) -> Move {
        Move {
            movement: self.movement.clone(),
            promote: None,
            information: self.information.clone(),
            check: false,
            check_mate: false,
            name: "".to_string(),
        }
    }
}


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Promotion { Rook, Bishop, Queen, Knight }


#[derive(Debug, Clone, PartialEq, Copy)]
pub enum MoveRequirement { HasToAttack, Pacifist, FirstTime, IsFree(Index144), EnPassant }



#[derive(Clone, Copy, Default, Debug)]
pub struct Piece {
    pub kind: PieceType,
    pub color: ChessColor,
    pub has_moved: bool,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum PieceType { #[default] Pawn, Rook, Knight, Bishop, Queen, King }

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ChessColor { #[default] White, Black }


impl ChessColor {
    pub fn to_str_fen_format(&self) -> String {
        match self {
            ChessColor::White => "w",
            ChessColor::Black => "b",
        }.to_string()
    }
}