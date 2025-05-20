use crate::extra::{index_144_to_64, index_64_to_144, index_64_to_algebraic};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index144(i32);

pub enum BoardType { Regular, Large }


const INVALID_INDEXES: [i32; 80] = [9, 83, 84, 135, 141, 120, 138, 24, 48, 85, 123, 139, 7, 121, 6, 136, 25, 35, 142, 106, 128, 22, 70, 124, 34, 4, 5, 23, 71, 20, 127, 2, 37, 109, 129, 122, 143, 3, 107, 36, 140, 73, 0, 21, 125, 14, 18, 132, 72, 133, 96, 108, 19, 130, 12, 16, 1, 95, 15, 126, 59, 131, 97, 119, 58, 17, 10, 61, 118, 46, 94, 13, 137, 47, 82, 8, 60, 134, 49, 11 ];
const LAST_ROWS: [i32; 16] = [26, 27, 28, 29, 30, 31, 32, 32, 110, 111, 112, 113, 114, 115, 116, 117];


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

    pub fn new() -> Self {
        Self(0)
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

    pub fn is_valid(&self) -> bool {
        !INVALID_INDEXES.contains(&self.i12())
    }

    pub fn is_on_last_row(&self) -> bool {
        LAST_ROWS.contains(&self.0)
    }

    pub fn str(&self) -> String {
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



// Resources


// Components

#[derive(Clone)]
pub struct ChessBoard {
    pub pieces: [Option<Piece>; 144],
    pub en_passant: Option<EnPassant>,
    pub turn: ChessColor,

    pub valid_moves: Vec<Move>,
    pub move_history: Vec<Move>,
    
    // changes
    pub board_changed: bool,
    pub turn_changed: bool,
}


impl ChessBoard {
    pub fn change_turn(&mut self) {
        self.turn = match self.turn {
            ChessColor::White => ChessColor::Black,
            ChessColor::Black => ChessColor::White,
        };

        self.turn_changed = true;
    }

    pub fn tick(&mut self) {
        self.board_changed = false;
        self.turn_changed = false;
    }
}


#[derive(Debug, Clone, Copy)]
pub struct EnPassant {
    pub to_attack: Index144,
    pub to_remove: Index144,
}


// #[derive(Component)]
// pub struct Turn(pub ChessColor);

// #[derive(Component)]
// pub struct ChangeTurn;



// #[derive(Component, DerefMut, Deref, Debug)]
// pub struct ValidMoves(pub Vec<Move>);

// #[derive(Component, Debug, DerefMut, Deref)]
// pub struct MoveHistory(pub Vec<Move>);

// #[derive(Component, Debug, Clone, PartialEq)]
// pub struct Move {
//     pub from: Index144,
//     pub to: Index144,
//     pub promote: Option<Promotion>,
//     pub extra_move: Option<(Index144, Index144)>,
//     pub requires: Vec<MoveRequirement>,
// }

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


// #[derive(Debug)]
// pub struct ProposeMove {
//     pub from: Index144,
//     pub to: Index144,
// }


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