use bevy::{platform::collections::HashSet, prelude::*};
use crate::extra::{algebraic_to_index_144, index_64_to_algebraic};


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
                list_new_moves,
                change_turn,
                calc_valid_moves_on_turn_change,
                play_move,
            ))
        ;
    }
}



fn play_move(
    mut commands: Commands,
    mut query: Query<(Entity, &mut PlayMove, &mut MoveHistory, &ValidMoves, &mut Turn), Changed<PlayMove>>,
) {
    if query.is_empty() { return }
    let (entity, mut play_move, mut history, valid_moves, mut turn) = query.single_mut().unwrap();
    

    if let Some(mv) = &play_move.0 {
        if valid_moves.0.contains(&mv) {
            println!("playing move: {:?}", mv);
            history.push(mv.clone());
    
            play_move.0 = None;
            commands.entity(entity).insert(ChangeTurn);
        }
    }
}



fn setup_chessboard(mut commands: Commands) {
    commands.spawn((
        ChessBoard::default(),
        MoveHistory(Vec::new()),
        Turn(ChessColor::White),
        ValidMoves(Vec::new()),
        PlayMove(None),
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



fn list_new_moves(moves: Query<(&ValidMoves, &Turn), Changed<ValidMoves>>) {
    if let Ok((vm, turn)) = moves.single() {
        if !vm.0.is_empty() {
            info!("\n------- {:?}: Valid Moves -------", turn.0);
        }
        
        for mm in &vm.0 {
            // let from_i64 = index_144_to_64(mm.from as i32);
            // let to_i64 = index_144_to_64(mm.to as i32);
            
            info!("{} -> {}", index_64_to_algebraic(mm.from), index_64_to_algebraic(mm.to))
        }
    }
}





fn calc_valid_moves_on_turn_change(
    mut valid_moves: Query<&mut ValidMoves>,
    turn: Query<&Turn, Changed<Turn>>,
    chessboard: Query<&ChessBoard>,
    invalid_indexes: Res<InvalidIndexes>,
) {
    if let Ok(turn) = turn.single() {
        info!("Calculating valid moves...");
        
        
        
        let mut valid_moves = valid_moves.single_mut().expect("");
        let chessboard = chessboard.single().expect("");

        valid_moves.clear();

        let mut proposed_moves: Vec<ProposeMove> = Vec::new();

        
        let mut index: Index144 = Index144::from12(-1);
        for piece in &chessboard.pieces {
            index.inc(BoardType::Large);

            
            if let Some(piece) = piece {
                let Piece { color, kind, has_moved } = piece;
                if *color != turn.0 { continue }                

                match kind {
                    PieceType::Pawn => {
                        let direction = match *color {
                            ChessColor::White => -1,
                            ChessColor::Black => 1,
                        };

                        use MoveRequirement::*;

                        proposed_moves.push(ProposeMove { from: index, to: index.clone().up(direction).clone(), requires: [Pacifist].into() });
                        proposed_moves.push(ProposeMove { from: index, to: index.clone().up(direction).up(direction).clone(), requires: [Pacifist, FirstTime, IsFree(index.clone().up(direction).s())].into() });

                        proposed_moves.push(ProposeMove { from: index, to: index.clone().up(direction).dec(BoardType::Large).s(), requires: [HasToAttack].into() });
                        proposed_moves.push(ProposeMove { from: index, to: index.clone().up(direction).inc(BoardType::Large).s(), requires: [HasToAttack].into() });

                        // if chessboard.0[new_index].is_none() {
                        //     valid_moves.push(types::Move {
                        //         from: index as usize,
                        //         to: new_index,
                        //     })
                        // }
                    },
                    _ => {}
                }
            }            
        }


        for proposal in &proposed_moves {
            let mut not_worthy = false;

            for req in &proposal.requires {
                not_worthy = not_worthy || match *req {
                    MoveRequirement::FirstTime => chessboard.get(proposal.from).unwrap().has_moved,
                    MoveRequirement::HasToAttack => {
                        let o_target = chessboard.get(proposal.to);

                        if let Some(target) = o_target {
                            target.color == turn.0
                        }
                        else {
                            true
                        }
                    },
                    MoveRequirement::IsFree(req_index) => chessboard.get(req_index).is_some(),
                    MoveRequirement::Pacifist => chessboard.pieces[proposal.to.u12()].is_some(),
                };
            }
            
            if !not_worthy && !invalid_indexes.contains(&(proposal.to)) {
                valid_moves.push(chess_types::Move { from: proposal.from, to: proposal.to });
            }
        }

        info!("New Moves: {:?}", valid_moves);
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
                None, None, None, None, None, Some(Piece { color: ChessColor::Black, kind: PieceType::Pawn, ..default() }), None, None, None, None, None, None,
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



/// TODO! Skal laves om! for langsomt
fn react_on_move(
    mut chessboard: Query<&mut ChessBoard>,
    moves: Query<&MoveHistory, Changed<MoveHistory>>
) {
    let mut chessboard = chessboard.single_mut().expect("");
    if let Ok(moves) = moves.single() {
        if let Some(mv) = moves.0.last() {
            if let Some(piece) = chessboard.get_mut(mv.from) {
                piece.has_moved = true;
            }

            chessboard.pieces[mv.to.u12()] = chessboard.pieces[mv.from.u12()];
            chessboard.pieces[mv.from.u12()] = None;
        }
    }
}






