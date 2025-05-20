use bevy::{platform::collections::HashSet, prelude::*};
use crate::extra::{algebraic_to_index_144, index_64_to_algebraic, iter_len};


pub mod chess_types;
pub mod run_ifs;
use chess_types::*;



pub struct ChessPlugin;
impl Plugin for ChessPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup, setup_chessboard))
            .add_systems(Update, (
                react_on_move,
                change_turn,
                calc_valid_moves,
            ))
        ;
    }
}



fn setup_chessboard(mut commands: Commands) {
    commands.spawn((
        ChessBoard::default(),
        MoveHistory(Vec::new()),
        Turn(ChessColor::White),
        ValidMoves(Vec::new()),
    ));
}


fn change_turn(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Turn, Option<&ChangeTurn>)>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let mut go_ahead = false;
    if keys.just_pressed(KeyCode::KeyT) {
        go_ahead = true;
    }

    if let Ok((entity, mut turn, o_ct)) = query.single_mut() {
        if o_ct.is_some() {
            commands.entity(entity).remove::<ChangeTurn>();
            go_ahead = true;
        }

        if go_ahead {
            turn.0 = match turn.0 {
                ChessColor::White => ChessColor::Black,
                ChessColor::Black => ChessColor::White,
            };
            info!("turn change: {:?}", turn.0);
        }
    }
}



// fn list_new_moves(moves: Query<(&ValidMoves, &Turn), Changed<ValidMoves>>) {
//     if let Ok((vm, turn)) = moves.single() {
//         if !vm.0.is_empty() {
//             info!("\n------- {:?}: Valid Moves -------", turn.0);
//         }
        
//         for mm in &vm.0 {
//             // let from_i64 = index_144_to_64(mm.from as i32);
//             // let to_i64 = index_144_to_64(mm.to as i32);
            
//             info!("{} -> {}", index_64_to_algebraic(mm.from()), index_64_to_algebraic(mm.to()))
//         }
//     }
// }


fn calc_valid_moves(
    mut valid_moves: Query<&mut ValidMoves>,
    turn: Query<&Turn, Changed<Turn>>,
    chessboard: Query<&ChessBoard>,
) {
    if iter_len(turn.iter()) != 1 { return }
    if iter_len(chessboard.iter()) != 1 { return }
    if iter_len(valid_moves.iter()) != 1 { return }
    
    let turn = turn.single().unwrap();
    let chessboard = chessboard.single().unwrap();
    let mut valid_moves = valid_moves.single_mut().unwrap();


    valid_moves.clear();
    
    // basically regn alle pseudo træk ud.
    // derefter lav kør alle trækkene igennem og tjek om målet er en konge, hvis det er, så marker den som check.
    // for hvert træk prøv at regn ud om vores konge nu er i fare.


    // step 1. regn alle pseudo træk ud
    let mut pseudo_moves = Vec::new();
    calculate_pseudo_moves(&mut pseudo_moves, &turn.0, &chessboard);

    // tilføj check data
    let _ = check_for_check(&mut pseudo_moves, &chessboard);


    // step 2. prøv trækket og se om det resultere i at vores konge er sat i skak...
    for pm in &mut pseudo_moves {
        let mut new_chessboard = chessboard.clone();
        apply_move_to_chessboard(&mut new_chessboard, &pm);

        let mut new_pseudo_moves = Vec::new();
        let new_turn = match turn.0 {
            ChessColor::White => ChessColor::Black,
            ChessColor::Black => ChessColor::White,
        };
        calculate_pseudo_moves(&mut new_pseudo_moves, &new_turn, &new_chessboard);

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
        valid_moves.push(pm.clone());
    }


    info!("calculated {} actual moves", valid_moves.len());
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



fn calculate_pseudo_moves(
    // mut valid_moves: Query<&mut ValidMoves>,
    // turn: Query<&Turn, Changed<Turn>>,
    // chessboard: Query<&ChessBoard>,
    // invalid_indexes: Res<InvalidIndexes>,

    pseudo_moves: &mut Vec<chess_types::Move>,
    turn: &ChessColor,
    chessboard: &ChessBoard,
) {
    let mut proposed_moves: Vec<chess_types::ProposeMove> = Vec::new();

    
    let mut index: Index144 = Index144::from12(-1);
    for piece in &chessboard.pieces {
        index.inc(BoardType::Large);
        

        
        if let Some(piece) = piece {
            let Piece { color, kind, has_moved: _ } = piece;
            if color != turn { continue }

            use MoveRequirement::*;             



            match kind {
                PieceType::Pawn => {
                    let direction = match *color {
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


                    if let Some(king) = chessboard.get(index) {
                        if king.has_moved {
                            continue;
                        }
                    }


                    // king side
                    let empty_space1 = chessboard.get(*index.clone().add(1));
                    let empty_space2 = chessboard.get(*index.clone().add(2));
                    let rook_king_side = chessboard.get(*index.clone().add(3));

                    if empty_space1.is_none() && empty_space2.is_none() {
                        if let Some(rook) = rook_king_side {
                            if !rook.has_moved {
                                proposed_moves.push(ProposeMove { movement: (index, *index.clone().add(2)).into(), requires: [Pacifist].into(), information: MoveInformation::CastleKingSide });
                            }
                        }
                    }


                    // queen side
                    let empty_space1 = chessboard.get(*index.clone().add(-1));
                    let empty_space2 = chessboard.get(*index.clone().add(-2));
                    let empty_space3 = chessboard.get(*index.clone().add(-3));
                    let rook_queen_side = chessboard.get(*index.clone().add(-4));

                    if empty_space1.is_none() && empty_space2.is_none() && empty_space3.is_none() {
                        if let Some(rook) = rook_queen_side {
                            if !rook.has_moved {
                                proposed_moves.push(ProposeMove { movement: (index, *index.clone().add(-2)).into(), requires: [Pacifist].into(), information: MoveInformation::CastleQueenSide });
                            }
                        }
                    }
                },
                _ => {
                    let directions = match *kind {
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
                            
                            if let Some(target_piece) = chessboard.get(target_index) {
                                if target_piece.color == *color {
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
        let mut not_worthy = if let Some(target_piece) = chessboard.get(proposal.movement.to) {
            target_piece.color == chessboard.get(proposal.movement.from).unwrap().color
        } else {
            false
        };

        for req in &proposal.requires {
            not_worthy = not_worthy || match *req {
                MoveRequirement::FirstTime => chessboard.get(proposal.movement.from).unwrap().has_moved,
                MoveRequirement::HasToAttack => {
                    let o_target = chessboard.get(proposal.movement.to);

                    if let Some(target) = o_target {
                        target.color == *turn
                    }
                    else {
                        true
                    }
                },
                MoveRequirement::IsFree(req_index) => chessboard.get(req_index).is_some(),
                MoveRequirement::Pacifist => chessboard.pieces[proposal.movement.to.u12()].is_some(),
                MoveRequirement::EnPassant => {
                    if let Some(en_passant_index) = &chessboard.en_passant {
                        en_passant_index.to_attack != proposal.movement.to
                    }
                    else {
                        true
                    }
                },
            };
        }
        

        if !not_worthy && proposal.movement.to.is_valid() {
            if proposal.movement.to.is_on_last_row() && chessboard.get(proposal.movement.from).unwrap().kind == PieceType::Pawn {
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






fn setup(mut commands: Commands) {
    let mut invalid_fields = Vec::new();
    let mut invalid_indexes: HashSet<Index144> = HashSet::new();

    let mut index = 0;
    
    for y in 0..12 {
        for x in 0..12 {
            // Valid fields are in the 8x8 center: x in 2..10, y in 2..10
            if x < 2 || x >= 10 || y < 2 || y >= 10 {
                invalid_fields.push((x, y));
                
                invalid_indexes.insert(Index144::from12(index));
            }


            index += 1;
        }
    }

    
    commands.insert_resource(InvalidIndexes(invalid_indexes));
    commands.insert_resource(InvalidPositions(invalid_fields.into_iter().collect()));
}




impl ChessBoard {
    fn default() -> Self {
        Self {
            pieces: [
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, Some(Piece { color: ChessColor::Black, kind: PieceType::Rook, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Knight, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Bishop, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Queen, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::King, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Bishop, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Knight, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Rook, ..default() }), None, None,
                None, None, Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, ..default() }), None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Pawn, ..default() }), None, None,
                None, None, Some(Piece { color: ChessColor::White, kind: PieceType::Rook, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Knight, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Bishop, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Queen, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::King, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Bishop, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Knight, ..default() }), Some(Piece { color: ChessColor::White, kind: PieceType::Rook, ..default() }), None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
                None, None, None, None, None, None, None, None, None, None, None, None,
            ],
            en_passant: None,
        }
    }


    fn get(&self, index: Index144) -> Option<&Piece> {
        self.pieces[index.u12()].as_ref()
    }

    fn get_mut(&mut self, index: Index144) -> Option<&mut Piece> {
        self.pieces[index.u12()].as_mut()
    }
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



fn apply_move_to_chessboard(
    chessboard: &mut ChessBoard,
    chess_move: &chess_types::Move,
) {
    if let Some(piece) = chessboard.get_mut(chess_move.from()) {
        piece.has_moved = true;
    }


    // ryk brikken
    chessboard.pieces[chess_move.to().u12()] = chessboard.pieces[chess_move.from().u12()];
    chessboard.pieces[chess_move.from().u12()] = None;

    if let Some(promote) = chess_move.promote {
        let new_piece_kind = match promote {
            Promotion::Queen => PieceType::Queen,
            Promotion::Rook => PieceType::Rook,
            Promotion::Bishop => PieceType::Bishop,
            Promotion::Knight => PieceType::Knight,
        };

        if let Some(piece) = &mut chessboard.pieces[chess_move.to().u12()] {
            piece.kind = new_piece_kind;
        }
    }

    match chess_move.information {
        MoveInformation::None => {},
        MoveInformation::PawnDoubleMove(to_attack) => chessboard.en_passant = Some(EnPassant {
            to_attack,
            to_remove: chess_move.to(),
        }),
        MoveInformation::EnPassant => {
            let en_passant = chessboard.en_passant.clone().unwrap();
            
            chessboard.pieces[en_passant.to_remove.u12()] = None;
        },
        MoveInformation::CastleKingSide => {
            // move the rook
            let rook_index = chess_move.movement.from.clone().add(3).clone();
            let new_rook_index = chess_move.movement.from.clone().add(1).clone();

            chessboard.pieces[new_rook_index.u12()] = chessboard.pieces[rook_index.u12()];
            chessboard.get_mut(new_rook_index).unwrap().has_moved = true;
            chessboard.pieces[rook_index.u12()] = None;
        },
        MoveInformation::CastleQueenSide => {
            // move the rook
            let rook_index = chess_move.movement.from.clone().add(-4).clone();
            let new_rook_index = chess_move.movement.from.clone().add(-1).clone();

            chessboard.pieces[new_rook_index.u12()] = chessboard.pieces[rook_index.u12()];
            chessboard.get_mut(new_rook_index).unwrap().has_moved = true;
            chessboard.pieces[rook_index.u12()] = None;
        },
    }


    // remove en passant no matter what
    match chess_move.information {
        MoveInformation::PawnDoubleMove(_) => {},
        _ => chessboard.en_passant = None,
    }
}



/// TODO! Skal laves om! for langsomt
fn react_on_move(
    mut commands: Commands,
    mut chessboard: Query<&mut ChessBoard>,
    query: Query<(Entity, &MoveHistory), Changed<MoveHistory>>
) {
    let mut chessboard = chessboard.single_mut().expect("");
    if let Ok((entity, moves)) = query.single() {
        if let Some(mv) = moves.0.last() {
            apply_move_to_chessboard(&mut chessboard, mv);
            
            commands.entity(entity).insert(ChangeTurn);
        }
    }

}






