pub mod chess_types;
use chess_types::*;



impl ChessBoard {
    pub fn default() -> Self {
        let mut s = Self {
            pieces: [
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, Some(Piece { color: ChessColor::Black, kind: PieceType::Rook, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Knight, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Bishop, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Queen, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::King, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Bishop, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Knight, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Rook, has_moved: false }), None, None,
                None, None, Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, has_moved: false }), None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, has_moved: false }), None, None,
                None, None, Some(Piece { color: ChessColor::White, kind: PieceType::Rook, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Knight, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Bishop, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Queen, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::King, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Bishop, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Knight, has_moved: false }), Some(Piece { color: ChessColor::White, kind: PieceType::Rook, has_moved: false }), None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            en_passant: None,
            valid_moves: Vec::new(),
            move_history: Vec::new(),
            turn: ChessColor::White,

            board_changed: false,
            turn_changed: false,
        };

        s.calc_valid_moves();

        s
    }


    fn get(&self, index: Index144) -> Option<&Piece> {
        self.pieces[index.u12()].as_ref()
    }

    fn get_mut(&mut self, index: Index144) -> Option<&mut Piece> {
        self.pieces[index.u12()].as_mut()
    }


    pub fn play_move(&mut self, chess_move: &Move) {
        self.move_history.push(chess_move.clone());
        self.apply_move_to_chessboard(chess_move);
        self.change_turn();
        self.board_changed = true;
        self.turn_changed = true;


        self.calc_valid_moves();
    }

    fn calc_valid_moves(&mut self) {

        self.valid_moves.clear();
        
        // basically regn alle pseudo træk ud.
        // derefter lav kør alle trækkene igennem og tjek om målet er en konge, hvis det er, så marker den som check.
        // for hvert træk prøv at regn ud om vores konge nu er i fare.


        // step 1. regn alle pseudo træk ud
        let mut pseudo_moves = Vec::new();
        self.calculate_pseudo_moves(&mut pseudo_moves);

        // tilføj check data
        let _ = check_for_check(&mut pseudo_moves, self);


        // step 2. prøv trækket og se om det resultere i at vores konge er sat i skak...
        for pm in &mut pseudo_moves {
            let mut new_chessboard = self.clone();
            new_chessboard.apply_move_to_chessboard(pm);

            let mut new_pseudo_moves = Vec::new();
            new_chessboard.change_turn();
            new_chessboard.calculate_pseudo_moves(&mut new_pseudo_moves);

            // hvis kongen bliver sat i skak, så gælder den ikke
            if check_for_check(&mut new_pseudo_moves, &new_chessboard) { continue }


            let cant_continue: bool = match pm.information {
                MoveInformation::CastleKingSide => {
                    let squares_that_cannot_be_under_attack = vec![pm.from(), *pm.from().add(1), *pm.from().add(2)];

                    let mut attack_found = false;
                    
                    for npm in &new_pseudo_moves {
                        if squares_that_cannot_be_under_attack.contains(&npm.to()) {
                            attack_found = true;
                        }
                    }

                    attack_found
                },
                MoveInformation::CastleQueenSide => {
                    let squares_that_cannot_be_under_attack = vec![pm.from(), *pm.from().add(-1), *pm.from().add(-2)];

                    let mut attack_found = false;
                    
                    for npm in &new_pseudo_moves {
                        if squares_that_cannot_be_under_attack.contains(&npm.to()) {
                            attack_found = true;
                        }
                    }

                    attack_found
                }
                _ => false,
            };

            if cant_continue { continue }
            self.valid_moves.push(pm.clone());
        }


        println!("calculated {} actual moves", self.valid_moves.len());
    }


    fn calculate_pseudo_moves(&self, pseudo_moves: &mut Vec<chess_types::Move>) {
        let mut proposed_moves: Vec<chess_types::ProposeMove> = Vec::new();

        
        let mut index: Index144 = Index144::from12(-1);
        for piece in self.pieces {
            index.inc(BoardType::Large);
            

            
            if let Some(piece) = piece {
                let Piece { color, kind, has_moved: _ } = piece;
                if color != self.turn { continue }

                use MoveRequirement::*;             



                match kind {
                    PieceType::Pawn => {
                        let direction = match color {
                            ChessColor::White => -1,
                            ChessColor::Black => 1,
                        };

                        

                        // normalt frem og dobbelt ryk frem
                        proposed_moves.push(ProposeMove { movement: (index, *index.clone().up(direction)).into(), requires: [Pacifist].into(), information: MoveInformation::None });
                        proposed_moves.push(ProposeMove { movement: (index, *index.clone().up(direction).up(direction)).into(), requires: [Pacifist, FirstTime, IsFree(*index.clone().up(direction))].into(), information: MoveInformation::PawnDoubleMove(index.clone().up(direction).clone()) });

                        // angrib til hver side
                        proposed_moves.push(ProposeMove { movement: (index, *index.clone().up(direction).dec(BoardType::Large)).into(), requires: [HasToAttack].into(), information: MoveInformation::None });
                        proposed_moves.push(ProposeMove { movement: (index, *index.clone().up(direction).inc(BoardType::Large)).into(), requires: [HasToAttack].into(), information: MoveInformation::None });

                        // // tjek for en passant
                        proposed_moves.push(ProposeMove { movement: (index, *index.clone().up(direction).dec(BoardType::Large)).into(), requires: [EnPassant].into(), information: MoveInformation::EnPassant });
                        proposed_moves.push(ProposeMove { movement: (index, *index.clone().up(direction).inc(BoardType::Large)).into(), requires: [EnPassant].into(), information: MoveInformation::EnPassant });
                    },
                    PieceType::Knight => {
                        let directions: [i32; 8] = [24-1, 24+1, -24-1, -24+1, 2-12, 2+12, -2-12, -2+12];

                        for dir in directions {
                            proposed_moves.push(ProposeMove { movement: (index, *index.clone().add(dir)).into(), requires: [].into(), information: MoveInformation::None });
                        }
                    },
                    PieceType::King => {
                        let directions = vec![1, -1, 12, -12, 12-1, 12+1, -12-1, -12+1];

                        for dir in directions {
                            proposed_moves.push(ProposeMove { movement: (index, *index.clone().add(dir)).into(), requires: [].into(), information: MoveInformation::None });
                        }


                        if let Some(king) = self.get(index) {
                            if king.has_moved {
                                continue;
                            }
                        }


                        // king side
                        let empty_space1 = self.get(*index.clone().add(1));
                        let empty_space2 = self.get(*index.clone().add(2));
                        let rook_king_side = self.get(*index.clone().add(3));

                        if empty_space1.is_none() && empty_space2.is_none() {
                            if let Some(rook) = rook_king_side {
                                if !rook.has_moved {
                                    proposed_moves.push(ProposeMove { movement: (index, *index.clone().add(2)).into(), requires: [Pacifist].into(), information: MoveInformation::CastleKingSide });
                                }
                            }
                        }


                        // queen side
                        let empty_space1 = self.get(*index.clone().add(-1));
                        let empty_space2 = self.get(*index.clone().add(-2));
                        let empty_space3 = self.get(*index.clone().add(-3));
                        let rook_queen_side = self.get(*index.clone().add(-4));

                        if empty_space1.is_none() && empty_space2.is_none() && empty_space3.is_none() {
                            if let Some(rook) = rook_queen_side {
                                if !rook.has_moved {
                                    proposed_moves.push(ProposeMove { movement: (index, *index.clone().add(-2)).into(), requires: [Pacifist].into(), information: MoveInformation::CastleQueenSide });
                                }
                            }
                        }
                    },
                    _ => {
                        let directions = match kind {
                            PieceType::Rook => vec![1, -1, 12, -12],
                            PieceType::Bishop => vec![12-1, 12+1, -12-1, -12+1],
                            PieceType::Queen => vec![1, -1, 12, -12, 12-1, 12+1, -12-1, -12+1],
                            _ => vec![],
                        };

                        for dir in directions {
                            let mut target_index = index.clone();
                            target_index.add(dir);

                            while target_index.is_valid() {
                                let mut break_after = false;
                                
                                if let Some(target_piece) = self.get(target_index) {
                                    if target_piece.color == color {
                                        break;
                                    }
                                    else {
                                        break_after = true;
                                    }
                                }
                                proposed_moves.push(ProposeMove { movement: (index, target_index).into(), requires: [].into(), information: MoveInformation::None });
                                
                                target_index.add(dir);

                                if break_after { break }
                            }
                        }
                    },
                }
            }            
        }


        for proposal in &proposed_moves {
            
            // standard, må ikke dræbe ens egne
            let mut not_worthy = if let Some(target_piece) = self.get(proposal.movement.to) {
                target_piece.color == self.get(proposal.movement.from).unwrap().color
            } else {
                false
            };

            for req in &proposal.requires {
                not_worthy = not_worthy || match *req {
                    MoveRequirement::FirstTime => self.get(proposal.movement.from).unwrap().has_moved,
                    MoveRequirement::HasToAttack => {
                        let o_target = self.get(proposal.movement.to);

                        if let Some(target) = o_target {
                            target.color == self.turn
                        }
                        else {
                            true
                        }
                    },
                    MoveRequirement::IsFree(req_index) => self.get(req_index).is_some(),
                    MoveRequirement::Pacifist => self.pieces[proposal.movement.to.u12()].is_some(),
                    MoveRequirement::EnPassant => {
                        if let Some(en_passant_index) = &self.en_passant {
                            en_passant_index.to_attack != proposal.movement.to
                        }
                        else {
                            true
                        }
                    },
                };
            }
            

            if !not_worthy && proposal.movement.to.is_valid() {
                if proposal.movement.to.is_on_last_row() && self.get(proposal.movement.from).unwrap().kind == PieceType::Pawn {
                    // promotion
                    pseudo_moves.push( proposal.into_move().set_promotion(Promotion::Rook) );
                    pseudo_moves.push( proposal.into_move().set_promotion(Promotion::Bishop) );
                    pseudo_moves.push( proposal.into_move().set_promotion(Promotion::Knight) );
                    pseudo_moves.push( proposal.into_move().set_promotion(Promotion::Queen) );
                }
                else {
                    pseudo_moves.push( proposal.into_move() );
                }
            }
        }
    }    


    fn apply_move_to_chessboard(&mut self, chess_move: &chess_types::Move) {
        if let Some(piece) = self.get_mut(chess_move.from()) {
            piece.has_moved = true;
        }


        // ryk brikken
        self.pieces[chess_move.to().u12()] = self.pieces[chess_move.from().u12()];
        self.pieces[chess_move.from().u12()] = None;

        if let Some(promote) = chess_move.promote {
            let new_piece_kind = match promote {
                Promotion::Queen => PieceType::Queen,
                Promotion::Rook => PieceType::Rook,
                Promotion::Bishop => PieceType::Bishop,
                Promotion::Knight => PieceType::Knight,
            };

            if let Some(piece) = &mut self.pieces[chess_move.to().u12()] {
                piece.kind = new_piece_kind;
            }
        }

        match chess_move.information {
            MoveInformation::None => {},
            MoveInformation::PawnDoubleMove(to_attack) => self.en_passant = Some(EnPassant {
                to_attack,
                to_remove: chess_move.to(),
            }),
            MoveInformation::EnPassant => {
                let en_passant = self.en_passant.clone().unwrap();
                
                self.pieces[en_passant.to_remove.u12()] = None;
            },
            MoveInformation::CastleKingSide => {
                // move the rook
                let rook_index = chess_move.movement.from.clone().add(3).clone();
                let new_rook_index = chess_move.movement.from.clone().add(1).clone();

                self.pieces[new_rook_index.u12()] = self.pieces[rook_index.u12()];
                self.get_mut(new_rook_index).unwrap().has_moved = true;
                self.pieces[rook_index.u12()] = None;
            },
            MoveInformation::CastleQueenSide => {
                // move the rook
                let rook_index = chess_move.movement.from.clone().add(-4).clone();
                let new_rook_index = chess_move.movement.from.clone().add(-1).clone();

                self.pieces[new_rook_index.u12()] = self.pieces[rook_index.u12()];
                self.get_mut(new_rook_index).unwrap().has_moved = true;
                self.pieces[rook_index.u12()] = None;
            },
        }


        // remove en passant no matter what
        match chess_move.information {
            MoveInformation::PawnDoubleMove(_) => {},
            _ => self.en_passant = None,
        }
    }


}



fn check_for_check(list: &mut Vec<chess_types::Move>, chessboard: &ChessBoard) -> bool {
    let mut check_found = false;

    for pm in list {        
        if let Some(target) = chessboard.get(pm.movement.to) {
            if target.kind == PieceType::King {
                pm.check = true;
                check_found = true;
            }
        }
    }

    check_found
}







impl Piece {
    pub fn as_letters(&self) -> String {
        let color = match self.color {
            ChessColor::White => "w",
            ChessColor::Black => "b",
        };

        let kind = match self.kind {
            PieceType::Bishop => "b",
            PieceType::Knight => "n",
            PieceType::King => "k",
            PieceType::Pawn => "p",
            PieceType::Queen => "q",
            PieceType::Rook => "r",
        };

        format!("{}{}", color, kind)
    }
}





// TODO! Skal laves om! for langsomt
// fn react_on_move(
//     mut commands: Commands,
//     mut chessboard: Query<&mut ChessBoard>,
//     // query: Query<(Entity, &MoveHistory), Changed<MoveHistory>>
// ) {
//     let mut chessboard = chessboard.single_mut().expect("");
//     if let Ok((entity, moves)) = query.single() {
//         if let Some(mv) = moves.0.last() {
//             apply_move_to_chessboard(&mut chessboard, mv);
            
//             commands.entity(entity).insert(ChangeTurn);
//         }
//     }

// }






