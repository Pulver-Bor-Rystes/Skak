use std::slice::SliceIndex;

use crate::Castling;

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub kind: PieceType,
    pub color: Color,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PieceType {
    Pawn,
    Bishop,
    Knight,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn as_index(&self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 1,
        }
    }

    pub fn as_num(&self) -> i32 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }

    pub fn opposite(&self) -> Color {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}

// impl SliceIndex<[Castling]> for Color {
//     type Output = usize;

//     fn index(&self) -> Self::Output {
//         0
//     }
// }

pub enum ConversionError {
    InvalidString,
}

impl Piece {
    pub fn to_string(&self) -> String {
        let piece_letter = match self.kind {
            PieceType::Pawn => "p",
            PieceType::Bishop => "b",
            PieceType::Knight => "n",
            PieceType::Rook => "r",
            PieceType::Queen => "q",
            PieceType::King => "k",
        };

        if self.color == Color::White {
            return piece_letter.to_uppercase();
        }

        piece_letter.to_string()
    }

    pub fn try_from_str(str: &str) -> Result<Piece, ConversionError> {
        let kind = match str.to_lowercase().as_str() {
            "p" => PieceType::Pawn,
            "r" => PieceType::Rook,
            "n" => PieceType::Knight,
            "b" => PieceType::Bishop,
            "q" => PieceType::Queen,
            "k" => PieceType::King,
            _ => return Err(ConversionError::InvalidString),
        };

        let color = if str == str.to_lowercase() {
            Color::Black
        } else {
            Color::White
        };

        Ok(Piece { kind, color })
    }
}

impl PieceType {
    pub fn to_string(&self) -> &'static str {
        match self {
            PieceType::Pawn => "Pawn",
            PieceType::Bishop => "Bishop",
            PieceType::Knight => "Knight",
            PieceType::Rook => "Rook",
            PieceType::Queen => "Queen",
            PieceType::King => "King",
        }
    }
}

impl Color {
    pub fn to_string(&self) -> &'static str {
        match self {
            Color::White => "White",
            Color::Black => "Black",
        }
    }
}
