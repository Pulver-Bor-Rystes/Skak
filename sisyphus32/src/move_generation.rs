use crate::{BitMove, Move, Bitboard, Color, MoveFlag, MoveList, MoveMasks, Piece, Position, Rank, Square};

pub struct MoveGeneration;

impl MoveGeneration {
    #[inline]
    pub fn generate_moves<T: Move, F: Filter>(position: &Position) -> MoveList<T> {
        let mut move_list = MoveList::new();

        let ([pawn, knight, bishop, rook, queen, king], inv_own_occupancies) = match position.side {
            Color::White => (Piece::WHITE_PIECES, !position.white_occupancy),
            Color::Black => (Piece::BLACK_PIECES, !position.black_occupancy),
        };

        {
            /*------------------------------*\ 
                        Pawn moves
            \*------------------------------*/
            let (pawn_promotion_rank, pawn_starting_rank, en_passant_rank, pawn_double_push_rank, double_pawn_flag, en_passant_flag, enemy_occupancies) = match position.side {
                Color::White => (Rank::R7, Rank::R2, Rank::R5, Rank::R4, Some(MoveFlag::WDoublePawn), Some(MoveFlag::WEnPassant), position.black_occupancy),
                Color::Black => (Rank::R2, Rank::R7, Rank::R4, Rank::R5, Some(MoveFlag::BDoublePawn), Some(MoveFlag::BEnPassant), position.white_occupancy),
            };

            let mut pawn_bb = position.bitboards[pawn];
            while pawn_bb.is_not_empty() {
                let source = pawn_bb.pop_lsb();
                let source_rank = source.rank();

                // Captures
                let mut capture_mask = MoveMasks::get_pawn_capture_mask(position.side, source) & enemy_occupancies;
                while capture_mask.is_not_empty() {
                    let target = capture_mask.pop_lsb();

                    #[cfg(feature = "bb")]
                    let capture_option = position.get_piece_option(target);

                    if source_rank == pawn_promotion_rank {
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, capture_option, Some(MoveFlag::PromoN)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoN)));
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, capture_option, Some(MoveFlag::PromoB)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoB)));
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, capture_option, Some(MoveFlag::PromoR)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoR)));
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, capture_option, Some(MoveFlag::PromoQ)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoQ)));
                    } else {
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, capture_option, None));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                    }
                }

                // Quiet moves
                let mut quiet_mask = MoveMasks::get_pawn_quiet_mask(position.side, source) & !position.all_occupancy;
                while quiet_mask.is_not_empty() {
                    let target = quiet_mask.pop_lsb();
                    
                    if source_rank == pawn_starting_rank && target.rank() == pawn_double_push_rank {
                        // Making sure both squares in front of the pawn are empty
                        if (MoveMasks::get_pawn_quiet_mask(position.side, source) & position.all_occupancy).is_empty() {
                            
                            #[cfg(feature = "bb")]
                            Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, None, double_pawn_flag));

                            #[cfg(feature = "bb_array")]
                                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, double_pawn_flag));
                        } 
                    } else if source_rank == pawn_promotion_rank {
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, None, Some(MoveFlag::PromoN)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoN)));
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, None, Some(MoveFlag::PromoB)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoB)));
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, None, Some(MoveFlag::PromoR)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoR)));
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, None, Some(MoveFlag::PromoQ)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoQ)));
                    } else {
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, None, None));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                    }
                }
                
                // En-passant
                if let Some(en_passant_sq) = position.en_passant_option {
                    if source_rank == en_passant_rank {
                        let mut en_passant_mask = MoveMasks::get_pawn_capture_mask(position.side, source);
                        while en_passant_mask.is_not_empty() {
                            let target = en_passant_mask.pop_lsb();
                            if target == en_passant_sq {
                                #[cfg(feature = "bb")]
                                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, None, en_passant_flag));

                                #[cfg(feature = "bb_array")]
                                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, en_passant_flag));
                            }
                        }
                    }
                }
            }
        }

        {
            /*------------------------------*\ 
                    Knight moves
            \*------------------------------*/
            let mut knight_bb = position.bitboards[knight];
            while knight_bb.is_not_empty() {
                let source = knight_bb.pop_lsb();
                
                let mut move_mask = MoveMasks::get_knight_mask(source) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "bb")]
                    let capture_option = position.get_piece_option(target);
                    
                    #[cfg(feature = "bb")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, knight, capture_option, None));

                    #[cfg(feature = "bb_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        King moves
            \*------------------------------*/
            let (
                king_side_castling_flag, queen_side_castling_flag,
                king_side_castling_mask, queen_side_castling_mask,
                king_side_castling_right, queen_side_castling_right,
                castling_square_c, castling_square_d, castling_square_e, castling_square_f, castling_square_g
            ) = match position.side {
                Color::White => (
                    Some(MoveFlag::WKCastle), Some(MoveFlag::WQCastle),
                    Bitboard::W_KING_SIDE_MASK, Bitboard::W_QUEEN_SIDE_MASK,
                    position.castling_rights.wk(), position.castling_rights.wq(),
                    Square::C1, Square::D1, Square::E1, Square::F1, Square::G1
                ),
                Color::Black => (
                    Some(MoveFlag::BKCastle), Some(MoveFlag::BQCastle),
                    Bitboard::B_KING_SIDE_MASK, Bitboard::B_QUEEN_SIDE_MASK,
                    position.castling_rights.bk(), position.castling_rights.bq(),
                    Square::C8, Square::D8, Square::E8, Square::F8, Square::G8
                ),
            };

            let mut king_bb = position.bitboards[king];
            let source = king_bb.pop_lsb();
            let mut move_mask = MoveMasks::get_king_mask(source) & inv_own_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();

                #[cfg(feature = "bb")]
                let capture_option = position.get_piece_option(target);
                
                #[cfg(feature = "bb")]
                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, king, capture_option, None));

                #[cfg(feature = "bb_array")]
                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
            }

            // Kingside Castling
            #[allow(clippy::collapsible_if)]
            if king_side_castling_right && (position.all_occupancy & king_side_castling_mask).is_empty() {
                if !position.is_square_attacked(position.side, castling_square_e) &&
                !position.is_square_attacked(position.side, castling_square_f) &&
                !position.is_square_attacked(position.side, castling_square_g)
                {
                    
                    #[cfg(feature = "bb")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, castling_square_g, king, None, king_side_castling_flag));

                    #[cfg(feature = "bb_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, castling_square_g, king_side_castling_flag));
                }
            }

            // Queenside Castling
            #[allow(clippy::collapsible_if)]
            if queen_side_castling_right && (position.all_occupancy & queen_side_castling_mask).is_empty() {
                if !position.is_square_attacked(position.side, castling_square_e) &&
                !position.is_square_attacked(position.side, castling_square_d) &&
                !position.is_square_attacked(position.side, castling_square_c)
                {
                    
                    #[cfg(feature = "bb")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, castling_square_c, king, None, queen_side_castling_flag));

                    #[cfg(feature = "bb_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, castling_square_c, queen_side_castling_flag));
                }
            }
        }

        {
            /*------------------------------*\ 
                    Bishop moves
            \*------------------------------*/
            let mut bishop_bb = position.bitboards[bishop];
            while bishop_bb.is_not_empty() {
                let source = bishop_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_bishop_mask(source, position.all_occupancy) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "bb")]
                    let capture_option = position.get_piece_option(target);
                    
                    #[cfg(feature = "bb")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, bishop, capture_option, None));

                    #[cfg(feature = "bb_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        Rook moves
            \*------------------------------*/
            let mut rook_bb = position.bitboards[rook];
            while rook_bb.is_not_empty() {
                let source = rook_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_rook_mask(source, position.all_occupancy) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "bb")]
                    let capture_option = position.get_piece_option(target);
                    
                    #[cfg(feature = "bb")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, rook, capture_option, None));

                    #[cfg(feature = "bb_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                }
            }
        }

        {
            /*------------------------------*\ 
                    Queen moves
            \*------------------------------*/
            let mut queen_bb = position.bitboards[queen];
            while queen_bb.is_not_empty() {
                let source = queen_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_queen_mask(source, position.all_occupancy) & inv_own_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "bb")]
                    let capture_option = position.get_piece_option(target);
                    
                    #[cfg(feature = "bb")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, queen, capture_option, None));

                    #[cfg(feature = "bb_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                }
            }
        }
        
        move_list
    }

    fn add_move<T: Move, F: Filter>(position: &Position, move_list: &mut MoveList<T>, bit_move: BitMove) {
        if F::should_add(position, bit_move) {
            move_list.add(T::new(position, bit_move));
        }
    }

    #[inline]
    pub fn generate_captures<T: Move, F: Filter>(position: &Position) -> MoveList<T> {
        let mut move_list = MoveList::new();
        
        let ([pawn, knight, bishop, rook, queen, king], enemy_occupancies) = match position.side {
            Color::White => (Piece::WHITE_PIECES, position.black_occupancy),
            Color::Black => (Piece::BLACK_PIECES, position.white_occupancy),
        };

        {
            /*------------------------------*\ 
                        Pawn moves
            \*------------------------------*/
            let (pawn_promotion_rank, en_passant_rank, en_passant_flag) = match position.side {
                Color::White => (Rank::R7, Rank::R5, Some(MoveFlag::WEnPassant)),
                Color::Black => (Rank::R2, Rank::R4, Some(MoveFlag::BEnPassant)),
            };

            let mut pawn_bb = position.bitboards[pawn];
            while pawn_bb.is_not_empty() {
                let source = pawn_bb.pop_lsb();
                let source_rank = source.rank();

                // Captures
                let mut capture_mask = MoveMasks::get_pawn_capture_mask(position.side, source) & enemy_occupancies;
                while capture_mask.is_not_empty() {
                    let target = capture_mask.pop_lsb();

                    #[cfg(feature = "bb")]
                    let capture_option = position.get_piece_option(target);

                    if source_rank == pawn_promotion_rank {
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, capture_option, Some(MoveFlag::PromoN)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoN)));
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, capture_option, Some(MoveFlag::PromoB)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoB)));
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, capture_option, Some(MoveFlag::PromoR)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoR)));
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, capture_option, Some(MoveFlag::PromoQ)));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, Some(MoveFlag::PromoQ)));
                    } else {
                        
                        #[cfg(feature = "bb")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, capture_option, None));

                        #[cfg(feature = "bb_array")]
                        Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                    }
                }

                #[cfg(feature = "quiescence_en_passant")]
                // En-passant
                if let Some(en_passant_sq) = position.en_passant_option {
                    if source_rank == en_passant_rank {
                        let mut en_passant_mask = MoveMasks::get_pawn_capture_mask(position.side, source);
                        while en_passant_mask.is_not_empty() {
                            let target = en_passant_mask.pop_lsb();
                            if target == en_passant_sq {
                                #[cfg(feature = "bb")]
                                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, pawn, None, en_passant_flag));

                                #[cfg(feature = "bb_array")]
                                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, en_passant_flag));
                            }
                        }
                    }
                }
            }
        }

        {
            /*------------------------------*\ 
                    Knight moves
            \*------------------------------*/
            let mut knight_bb = position.bitboards[knight];
            while knight_bb.is_not_empty() {
                let source = knight_bb.pop_lsb();
                
                let mut move_mask = MoveMasks::get_knight_mask(source) & enemy_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "bb")]
                    let capture_option = position.get_piece_option(target);
                    
                    #[cfg(feature = "bb")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, knight, capture_option, None));

                    #[cfg(feature = "bb_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        King moves
            \*------------------------------*/
            let mut king_bb = position.bitboards[king];
            let source = king_bb.pop_lsb();
            let mut move_mask = MoveMasks::get_king_mask(source) & enemy_occupancies;
            while move_mask.is_not_empty() {
                let target = move_mask.pop_lsb();

                #[cfg(feature = "bb")]
                let capture_option = position.get_piece_option(target);
                
                #[cfg(feature = "bb")]
                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, king, capture_option, None));

                #[cfg(feature = "bb_array")]
                Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
            }
        }

        {
            /*------------------------------*\ 
                    Bishop moves
            \*------------------------------*/
            let mut bishop_bb = position.bitboards[bishop];
            while bishop_bb.is_not_empty() {
                let source = bishop_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_bishop_mask(source, position.all_occupancy) & enemy_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "bb")]
                    let capture_option = position.get_piece_option(target);
                    
                    #[cfg(feature = "bb")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, bishop, capture_option, None));

                    #[cfg(feature = "bb_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                }
            }
        }

        {
            /*------------------------------*\ 
                        Rook moves
            \*------------------------------*/
            let mut rook_bb = position.bitboards[rook];
            while rook_bb.is_not_empty() {
                let source = rook_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_rook_mask(source, position.all_occupancy) & enemy_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "bb")]
                    let capture_option = position.get_piece_option(target);
                    
                    #[cfg(feature = "bb")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, rook, capture_option, None));

                    #[cfg(feature = "bb_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                }
            }
        }

        {
            /*------------------------------*\ 
                    Queen moves
            \*------------------------------*/
            let mut queen_bb = position.bitboards[queen];
            while queen_bb.is_not_empty() {
                let source = queen_bb.pop_lsb();
                let mut move_mask = MoveMasks::get_queen_mask(source, position.all_occupancy) & enemy_occupancies;
                while move_mask.is_not_empty() {
                    let target = move_mask.pop_lsb();

                    #[cfg(feature = "bb")]
                    let capture_option = position.get_piece_option(target);
                    
                    #[cfg(feature = "bb")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, queen, capture_option, None));

                    #[cfg(feature = "bb_array")]
                    Self::add_move::<T, F>(position, &mut move_list, BitMove::encode(source, target, None));
                }
            }
        }
        
        move_list
    }
}

pub trait Filter {
    fn should_add(position: &Position, bit_move: BitMove) -> bool;
}

pub struct PseudoLegal;
impl Filter for PseudoLegal {
    #[inline(always)]
    fn should_add(_position: &Position, _bit_move: BitMove) -> bool {
        true
    }
}

pub struct Legal;
impl Filter for Legal {
    #[inline(always)]
    fn should_add(position: &Position, bit_move: BitMove) -> bool {
        let mut position_copy = position.clone();
        position_copy.make_move(bit_move.get_bit_move());
        !position_copy.in_check(position_copy.side.opposite())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn generate_pseudo_legal_moves_returns_unique_moves() {
        let move_list = MoveGeneration::generate_moves::<BitMove, PseudoLegal>(&Position::starting_position());
        let mut seen = HashSet::new();
        assert!(move_list.iter().all(|&m| seen.insert(m)));
    }
}
