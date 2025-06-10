use crate::{EvalMove, MoveFlag, Piece, Position, Score, Square};
use core::fmt;
use std::{cmp::Ordering, fmt::Display, hash::Hash, mem};

#[cfg(feature = "bb")]
const SOURCE_MASK: u32 =  0b0000_0000_0000_0000_0000_0000_0011_1111;
#[cfg(feature = "bb")]
const TARGET_MASK: u32 =  0b0000_0000_0000_0000_0000_1111_1100_0000;
#[cfg(feature = "bb")]
const PIECE_MASK: u32 =   0b0000_0000_0000_0000_1111_0000_0000_0000;
#[cfg(feature = "bb")]
const CAPTURE_MASK: u32 = 0b0000_0000_0000_1111_0000_0000_0000_0000;
#[cfg(feature = "bb")]
const FLAG_MASK: u32 =    0b0000_0000_1111_0000_0000_0000_0000_0000;

#[cfg(feature = "bb_array")]
const SOURCE_MASK: u16 =  0b0000_0000_0011_1111;
#[cfg(feature = "bb_array")]
const TARGET_MASK: u16 =  0b0000_1111_1100_0000;
#[cfg(feature = "bb_array")]
const FLAG_MASK: u16 =    0b1111_0000_0000_0000;

pub trait Move: Copy + Default + Eq + Hash {
    fn get_bit_move(self) -> BitMove;
    fn new(position: &Position, bit_move: BitMove) -> Self;
    fn to_row_string(self) -> String;
}

/*------------------------------*\ 
             BitMove
\*------------------------------*/
#[cfg(feature = "bb")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BitMove(u32);

// NOTE: Maintaining an array of piece positions allows moves to use only two bytes
#[cfg(feature = "bb_array")]
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct BitMove(u16);

impl Default for BitMove {
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Move for BitMove {
    #[inline(always)]
    fn get_bit_move(self) -> BitMove {
        self
    }
    
    fn new(_position: &Position, bit_move: BitMove) -> Self {
        bit_move
    }
    
    #[cfg(feature = "bb")]
    fn to_row_string(self) -> String {
        format!(
            "  | {:<8} | {:<8} | {:<8} | {:<8?} | {:<19?} |\n",
            self.source(),
            self.target(),
            self.piece(),
            self.capture_option(),
            self.flag_option()
        )
    }

    #[cfg(feature = "bb_array")]
    fn to_row_string(self) -> String {
        format!(
            "  | {:<8} | {:<8} | {:<8} | {:<8} | {:<19?} |\n",
            self.source(),
            self.target(),
            "",
            "",
            self.flag_option()
        )
    }
}

impl BitMove {
    pub const EMPTY: BitMove = unsafe { mem::zeroed() };

    #[inline(always)]
    pub fn source(&self) -> Square {
        Square::from((self.0 & SOURCE_MASK) as u8)
    }

    #[inline(always)]
    pub fn target(&self) -> Square {
        Square::from(((self.0 & TARGET_MASK) >> 6) as u8)
    }

    #[cfg(feature = "bb")]
    #[inline(always)]
    pub fn piece(&self) -> Piece {
        Piece::from(((self.0 & PIECE_MASK) >> 12) as u8)
    }

    #[cfg(feature = "bb")]
    #[inline(always)]
    pub fn capture_option(&self) -> Option<Piece> {
        unsafe { std::mem::transmute::<u8, Option<Piece>>(((self.0 & CAPTURE_MASK) >> 16) as u8) }
    }

    #[cfg(feature = "bb")]
    #[inline(always)]
    pub fn flag_option(&self) -> Option<MoveFlag> {
        unsafe { std::mem::transmute::<u8, Option<MoveFlag>>(((self.0 & FLAG_MASK) >> 20) as u8) }
    }

    // NOTE: For the array representation, the flag mask is offset by 12 instead of 20
    #[cfg(feature = "bb_array")]
    #[inline(always)]
    pub fn flag_option(&self) -> Option<MoveFlag> {
        unsafe { std::mem::transmute::<u8, Option<MoveFlag>>(((self.0 & FLAG_MASK) >> 12) as u8) }
    }

    #[cfg(feature = "bb")]
    #[inline(always)]
    pub fn encode(
        source: Square, 
        target: Square, 
        piece: Piece, 
        capture_option: Option<Piece>, 
        flag_option: Option<MoveFlag>
    ) -> BitMove {
        unsafe {
            BitMove(
                source as u32 | 
                (target as u32) << 6 | 
                (piece as u32) << 12 | 
                (std::mem::transmute::<Option<Piece>, u8>(capture_option) as u32) << 16 |
                (std::mem::transmute::<Option<MoveFlag>, u8>(flag_option) as u32) << 20
            )
        }
    }

    #[cfg(feature = "bb_array")]
    #[inline(always)]
    pub fn encode(
        source: Square, 
        target: Square, 
        flag_option: Option<MoveFlag>
    ) -> BitMove {
        unsafe {
            BitMove(
                source as u16 | 
                (target as u16) << 6 | 
                (std::mem::transmute::<Option<MoveFlag>, u8>(flag_option) as u16) << 12
            )
        }
    }

    #[cfg(feature = "bb")]
    #[inline(always)]
    pub fn decode(&self) -> (Square, Square, Piece, Option<Piece>, Option<MoveFlag>) {
        (self.source(), self.target(), self.piece(), self.capture_option(), self.flag_option())
    }

    #[cfg(feature = "bb_array")]
    #[inline(always)]
    pub fn decode(&self) -> (Square, Square, Option<MoveFlag>) {
        (self.source(), self.target(), self.flag_option())
    }

    #[inline(always)]
    pub fn is_capture(self, position: &Position) -> bool {
        position.get_piece_option(self.target()).is_some()
    }

    #[inline(always)]
    pub(crate) fn is_capture_or_promotion(self, position: &Position) -> bool {
        self.is_capture(position) || self.flag_option().is_some_and(|f| f.is_promotion())
    }

    #[inline(always)]
    pub(crate) fn is_pp_capture_or_castle(self, position: &Position) -> bool {
        let source_piece = position.get_piece_option(self.source());
        let target_piece = position.get_piece_option(self.target());
        source_piece == Some(Piece::WP) ||
        source_piece == Some(Piece::BP) ||
        target_piece.is_some() ||
        self.flag_option().is_some_and(|f| f.is_castle())
    }

    pub fn to_uci_string(self) -> String {
        format!(
            "{}{}{}",
            self.source(),
            self.target(),
            match self.flag_option() {
                Some(MoveFlag::PromoN) => "n",
                Some(MoveFlag::PromoB) => "b",
                Some(MoveFlag::PromoR) => "r",
                Some(MoveFlag::PromoQ) => "q",
                _ => "",
            }
        )
    }
}

#[cfg(feature = "bb")]
impl Display for BitMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!(
            "
  Raw move data: {:b}
  Source Square: {}
  Target Square: {}
  Piece Type:    {}
  Capture:       {:?}
  Move Flag:     {:?}\n",
            self.0,
            self.source(),
            self.target(),
            self.piece(),
            self.capture_option(),
            self.flag_option()
        ))
    }
}

#[cfg(feature = "bb_array")]
impl Display for BitMove {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad(&format!(
            "
  Raw move data: {:b}
  Source Square: {}
  Target Square: {}
  Move Flag:     {:?}\n",
            self.0,
            self.source(),
            self.target(),
            self.flag_option()
        ))
    }
}

/*------------------------------*\ 
           ScoringMove
\*------------------------------*/
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct ScoringMove {
    pub bit_move: BitMove,
    pub score: Score,
}

impl Default for ScoringMove {
    #[inline(always)]
    fn default() -> Self {
        Self::EMPTY
    }
}

impl Move for ScoringMove {
    #[inline(always)]
    fn get_bit_move(self) -> BitMove {
        self.bit_move
    }
    
    #[inline(always)]
    fn new(position: &Position, bit_move: BitMove) -> Self {
        let score = EvalMove::eval(position, bit_move);
        Self { bit_move, score }
    }
    
    fn to_row_string(self) -> String {
        format!(
            "  | {:<7} | {:<53} |\n",
            self.bit_move.to_uci_string(),
            self.score,
        )
    }
}

impl ScoringMove {
    const EMPTY: ScoringMove = unsafe { mem::zeroed() };

    #[inline(always)]
    pub fn blank(score: Score) -> Self {
        Self {
            bit_move: BitMove::EMPTY,
            score
        }
    }

    #[inline(always)]
    pub fn new(bit_move: BitMove, score: Score) -> Self {
        Self {
            bit_move,
            score
        }
    }
}

impl From<BitMove> for ScoringMove {
    #[inline(always)]
    fn from(bm: BitMove) -> Self {
        ScoringMove { bit_move: bm, score: Score::BLANK }
    }
}

impl Ord for ScoringMove {
    fn cmp(&self, other: &Self) -> Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for ScoringMove {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "bb")]
    fn encode_and_decode_works() {
        let bit_move = BitMove::encode(Square::A1, Square::B1, Piece::WP, None, None);
        let (source, target, piece, capture_option, flag_option) = bit_move.decode();

        assert_eq!(source, Square::A1);
        assert_eq!(target, Square::B1);
        assert_eq!(piece, Piece::WP);
        assert_eq!(capture_option, None);
        assert_eq!(flag_option, None);
    }

    #[test]
    #[cfg(feature = "bb_array")]
    fn encode_and_decode_works() {
        let bit_move = BitMove::encode(Square::A1, Square::B1, None);
        let (source, target, flag_option) = bit_move.decode();

        assert_eq!(source, Square::A1);
        assert_eq!(target, Square::B1);
        assert_eq!(flag_option, None);
    }
}
