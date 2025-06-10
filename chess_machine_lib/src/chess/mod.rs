pub mod chess_types;
use core::panic;
use std::{collections::HashMap, time::Duration};
use chess_types::*;

#[cfg(feature = "time")]
use std::time::Instant;




impl ChessBoard {
    fn fake(&mut self) -> Self {
        let mut clone = self.clone();
        clone.real = false;
        clone
    }

    pub fn set_naming_convention(&mut self, naming_convention: NamingConvention) -> &mut Self {
        self.naming_convention = naming_convention;
        self.calc_valid_moves(false);
        self
    }

    fn empty() -> Self {
        Self {
            pieces: [None; 144],
            en_passant: None,
            turn: ChessColor::White,

            moves: Vec::new(),
            move_history: Vec::new(),
            
            halfmove_clock: 0,
            fullmove_number: 0,
            
            // changes
            board_changed: false,
            turn_changed: false,

            // meta
            fen_str: String::new(),
            real: true,
            naming_convention: NamingConvention::Standard,
            winner: None,

            // time
            clock: Clock {
                white: Duration::from_secs(60 * 10),
                black: Duration::from_secs(60 * 10),
                increment: Duration::from_secs(0),
                #[cfg(feature = "time")]
                since_last_move: Instant::now(),
            },
        }
    }
    
    
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
            moves: Vec::new(),
            move_history: Vec::new(),
            turn: ChessColor::White,
            fullmove_number: 0,
            halfmove_clock: 0,

            board_changed: false,
            turn_changed: false,

            fen_str: String::new(),
            real: true,
            naming_convention: NamingConvention::Standard,
            winner: None,

            // time
            clock: Clock {
                white: Duration::from_secs(60 * 10),
                black: Duration::from_secs(60 * 10),
                increment: Duration::from_secs(0),
                #[cfg(feature = "time")]
                since_last_move: Instant::now(),
            },
        };

        s.calc_valid_moves(false);

        s
    }


    pub fn from_fen(fen_str: &str) -> Self {
        let mut chessboard = ChessBoard::empty();
        chessboard.load_fen(fen_str);
        chessboard
    }


    pub fn load_fen(&mut self, fen_str: &str) -> &mut Self {

        // empty all the pieces
        for i in 0..64 {
            self.pieces[Index144::from8(i).u12()] = None;
        }
        
        self.fen_str = fen_str.to_string();
        // fen er delt ind i 6 dele. Lad os dele dem op

        // counteren for lov at starte på 1, så er det lidt nemmere at følge med på wikipedia siden
        let mut counter = 0;
        let mut index: Index144 = Index144::new();
        
        for part in fen_str.split_whitespace() {
            counter += 1;

            // 1. piece placement
            if counter == 1 {
                for letter in part.split("") {
                    if letter == "" { continue }
                    if letter == "/" { continue }
                    
                    index.inc(BoardType::Regular);
    
                    let piece = Piece::from_str(letter);
                    
                    if let Some(piece) = piece {
                        // println!("placing {} at {}", piece.kind.to_str_name(), index.to_str());
                        self.pieces[index.u12()] = Some(piece);
                    }
                    else {
                        // check if letter is a number and store the value
                        if let Ok(num) = letter.parse::<i32>() {
                            index.add(num-1);
                        }
                        else {
                            panic!("Noget gik galt da vi forsøgte at parse: {:?}", letter);
                        }
                    }       
                }
            }


            // 2. active color
            if counter == 2 {
                self.turn = match part {
                    "w" => ChessColor::White,
                    "b" => ChessColor::Black,
                    _ => panic!("Invalid turn: {}", part),
                };
            }

            // 3. castling availability
            if counter == 3 {
                // disable by default

                // black
                self.change_index_has_moved(30, true);
                self.change_index_has_moved(30-4, true);
                self.change_index_has_moved(30+3, true);


                // white
                self.change_index_has_moved(114, true);
                self.change_index_has_moved(114-4, true);
                self.change_index_has_moved(114+3, true);
                
                
                // castling
                for letter in part.split("") {
                    if letter == "" { continue }
                    
                    match letter {
                        "K" => {
                            self.change_index_has_moved(114, false);
                            self.change_index_has_moved(114+3, false);
                            // println!("White can castle king side");
                        },
                        "Q" => {
                            self.change_index_has_moved(114, false);
                            self.change_index_has_moved(114-4, false);
                            // println!("White can castle queen side");
                        },
                        "k" => {
                            self.change_index_has_moved(30, false);
                            self.change_index_has_moved(30+3, false);
                            // println!("Black can castle king side");
                        },
                        "q" => {
                            self.change_index_has_moved(30, false);
                            self.change_index_has_moved(30-4, false);
                            // println!("Black can castle queen side");
                        },
                        "-" => {},
                        _ => panic!("Invalid castling: {}", letter),
                    }
                }
            }


            // 4. En passant
            if counter == 4 {
                // en passant
                if part != "-" {
                    let to_attack = Index144::from_algebraic(part);

                    let to_remove = if to_attack.rank() == "3" {
                        to_attack.clone().add(12).clone()
                    }
                    else {
                        to_attack.clone().add(-12).clone()
                    };

                    self.en_passant = Some(EnPassant {
                        to_attack,
                        to_remove,
                    });
                }
            }

            // 5. halfmove clock
            if counter == 5 {
                if let Ok(halfmove_clock) = part.parse::<i32>() {
                    self.halfmove_clock = halfmove_clock;
                }
                else {
                    panic!("Invalid halfmove clock: {}", part);
                }
            }

            // 6. fullmove number
            if counter == 6 {
                if let Ok(fullmove_number) = part.parse::<i32>() {
                    self.fullmove_number = fullmove_number;
                }
                else {
                    panic!("Invalid fullmove number: {}", part);
                }
            }

        }


        self.calc_valid_moves(false);
        
        
        self
    }


    fn change_index_has_moved(&mut self, index: impl Into<Index144>, has_moved: bool) {
        if let Some(piece) = self.get_mut(index.into()) {
            piece.has_moved = has_moved;
        }
    }


    pub fn to_fen(&self) -> String {
        let mut fen = String::new();

        // 1. piece placement

        let mut counter = 0;
        let mut empty_space = 0;

        for i in 0..64 {
            counter += 1;
            let index = Index144::from8(i);

            if let Some(piece) = self.get(index) {
                if empty_space > 0 {
                    fen += empty_space.to_string().as_str();
                    empty_space = 0;
                }

                fen += &piece.to_str_fen_format();
            }
            else {
                empty_space += 1;
            }

            if counter == 8 {
                if empty_space > 0 {
                    fen += empty_space.to_string().as_str();
                }


                if i != 63 {
                    fen += "/";
                    
                    counter = 0;
                    empty_space = 0;
                }
            }
        }

        // 2. active color
        fen += " ";
        fen += &self.turn.to_str_fen_format();

        // 3. castling availability
        fen += " ";

        let mut castling = String::new();

        // hvid
        if let Some(Piece { kind: PieceType::King, color: _, has_moved: false }) = self.get(114) {
            // tjek det king side
            if let Some(Piece { kind: PieceType::Rook, color: _, has_moved: false }) = self.get(114+3) {
                castling += "K";
            }
            if let Some(Piece { kind: PieceType::Rook, color: _, has_moved: false }) = self.get(114-4) {
                castling += "Q";
            }
        }

        // sort
        if let Some(Piece { kind: PieceType::King, color: _, has_moved: false }) = self.get(30) {
            // tjek det king side
            if let Some(Piece { kind: PieceType::Rook, color: _, has_moved: false }) = self.get(30+3) {
                castling += "k";
            }
            if let Some(Piece { kind: PieceType::Rook, color: _, has_moved: false }) = self.get(30-4) {
                castling += "q";
            }
        }

        match castling == "" {
            true => fen += "-",
            false => fen += &castling,
        }


        // 4. en passant
        fen += " ";
        if let Some(en_passant) = self.en_passant {
            fen += &en_passant.to_attack.to_str();
        }
        else {
            fen += "-";
        }


        // 5. halfmove
        fen += " ";
        fen += self.halfmove_clock.to_string().as_str();

        // 6. fullmove
        fen += " ";
        fen += self.fullmove_number.to_string().as_str();
        
        
        // check
        // println!("\n\nFen Produced: {}", fen);
        // println!("Correct Fen:  {}", self.fen_str);
        // println!(" -> Fen parsing went ok? {}", self.fen_str.contains(&fen));

        fen
    }



    fn change_turn(&mut self) {
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


    pub fn get(&self, index: impl Into<Index144>) -> Option<&Piece> {
        let index: Index144 = index.into();
        self.pieces[index.u12()].as_ref()
    }

    fn get_mut(&mut self, index: Index144) -> Option<&mut Piece> {
        self.pieces[index.u12()].as_mut()
    }


    pub fn play_fromto(&mut self, from: Index144, to: Index144) {
        for chess_move in &self.moves.clone() {
            if chess_move.from() != from || chess_move.to() != to { continue }

            self.play(chess_move);

            break;
        }
    }


    pub fn play_notation(&mut self, move_name: &str) {
        for chess_move in &self.moves.clone() {
            if chess_move.name != move_name { continue }

            self.play(chess_move);
            
            break;
        }
    }


    pub fn is_move_name_valid(&self, move_name: &str) -> bool {
        for mv in &self.moves {
            if mv.name == move_name {
                return true;
            }
        }

        false
    }


    pub fn play(&mut self, chess_move: &Move) {
        if self.winner.is_some() { return }
        
        #[cfg(feature = "time")]
        match self.turn {
            ChessColor::White => {
                let new_clock = self.clock.white.checked_sub(self.clock.since_last_move.elapsed() - self.clock.increment);
                if let Some(new_clock) = new_clock {
                    self.clock.white = new_clock;
                    println!("white has {:?} left", self.clock.white);
                }
                else {
                    self.winner = Some(Winner::Black);
                    println!("Vi har fundet en vinder!\nTillykke {:?}", self.winner);
                    return;
                }
            },
            ChessColor::Black => {
                let new_clock = self.clock.black.checked_sub(self.clock.since_last_move.elapsed() - self.clock.increment);
                if let Some(new_clock) = new_clock {
                    self.clock.black = new_clock;
                    println!("black has {:?} left", self.clock.black);
                }
                else {
                    self.winner = Some(Winner::White);
                    println!("Vi har fundet en vinder!\nTillykke {:?}", self.winner);
                    return;
                }
            },
        }
        
        
        let was_capture = self.get(chess_move.to()).is_some();
        let was_pawn_advance = if let Some(Piece { kind: PieceType::Pawn, color: _, has_moved: _ }) = self.get(chess_move.from()) { true } else { false };
        
        // actual changes
        self.move_history.push(chess_move.clone());
        self.apply_move_to_chessboard(chess_move);
        self.change_turn();
        self.board_changed = true;
        self.turn_changed = true;


        if self.turn == ChessColor::White {
            self.fullmove_number += 1;
        }

        if was_capture || was_pawn_advance {
            self.halfmove_clock = 0;
        }
        else {
            self.halfmove_clock += 1;
        }


        self.calc_valid_moves(false);


        if self.moves.len() == 0 {
            self.winner = match self.turn {
                ChessColor::White => Winner::Black,
                ChessColor::Black => Winner::White
            }.into();

            println!("Vi har sgu en vinder! {:?}", self.winner);
        }

        if self.halfmove_clock >= 50 {
            self.winner = Winner::Tie.into();
            println!("Det blev uafgjort!");
        }
        
        
        #[cfg(feature = "time")]
        {
            self.clock.since_last_move = Instant::now();
        }
    }

    fn calc_valid_moves(&mut self, only_first_two_steps: bool) {
        self.moves.clear();
        
        // basically regn alle pseudo træk ud.
        // derefter lav kør alle trækkene igennem og tjek om målet er en konge, hvis det er, så marker den som check.
        // for hvert træk prøv at regn ud om vores konge nu er i fare.


        // step 1. regn alle pseudo træk ud
        let mut pseudo_moves = Vec::new();
        self.calculate_pseudo_moves(&mut pseudo_moves);

        // step 2. sorter alt fra som er farligt for vores konge!
        for pm in &mut pseudo_moves {
            let mut new_chessboard = self.fake();
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
            self.moves.push(pm.clone());
        }
        
        // springer resten over, så vi undgår et uendeligt loop!
        if only_first_two_steps { return }


        // step 3. gå alle træk igennem og tjek hvorvidt vi har sat kongen i skak eller skakmat.
        let new_chessboard = self.fake();
        
        for mv in &mut self.moves {
            // skakmat
            let mut cb1 = new_chessboard.clone();            
            cb1.apply_move_to_chessboard(mv);
            cb1.change_turn();
            cb1.calc_valid_moves(true);
            
            if cb1.moves.len() == 0 {
                mv.check_mate = true;
            }
            
            // tjek skak
            let mut cb2 = new_chessboard.clone();
            cb2.apply_move_to_chessboard(mv);
            cb2.calc_valid_moves(true);
            for new_move in cb2.moves.iter() {
                if let Some(Piece { kind: PieceType::King, color: _, has_moved: _ }) = cb2.get(new_move.to()) {
                    mv.check = true;
                }
            }
        }
        

        self.generate_name_for_each_move();
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

                        // double pawn move
                        if index.rank() == "2" || index.rank() == "7" {
                            proposed_moves.push(ProposeMove { movement: (index, *index.clone().up(direction).up(direction)).into(), requires: [Pacifist, FirstTime].into(), information: MoveInformation::PawnDoubleMove(*index.clone().up(direction)) });
                        }

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
                    pseudo_moves.push( proposal.into_move().set_promotion(Promotion::Queen) );
                    pseudo_moves.push( proposal.into_move().set_promotion(Promotion::Rook) );
                    pseudo_moves.push( proposal.into_move().set_promotion(Promotion::Bishop) );
                    pseudo_moves.push( proposal.into_move().set_promotion(Promotion::Knight) );
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


    fn generate_name_for_each_move(&mut self) {        
        let mut map: HashMap<String, HashMap<Index144, Vec<Move>>> = HashMap::new();

        for mv in &self.moves {
            let piece = self.get(mv.from()).unwrap();
            let key = piece.kind.to_str_name();

            if map.contains_key( &key ) {
                let target_map = map.get_mut(&key).unwrap();

                if target_map.contains_key(&mv.to()) {
                    let vec = target_map.get_mut(&mv.to()).unwrap();
                    vec.push(mv.clone());
                }
                else {
                    target_map.insert(mv.to(), vec![mv.clone()]);
                }
            }
            else {
                let mut new_hash_map: HashMap<Index144, Vec<Move>> = HashMap::new();
                new_hash_map.insert(mv.to(), vec![mv.clone()]);
                map.insert(key, new_hash_map);
            }
        }

        let board = self.fake();

        let mut names: HashMap<String, Vec<usize>> = HashMap::new();

        let mut index = 0;
        for mv in &mut self.moves {
            // lav navnet
            mv.make_name(&board, false, false);
            
            if names.contains_key(&mv.name) {
                let vec = names.get_mut(&mv.name).unwrap();
                vec.push(index);
            }
            else {
                names.insert(mv.name.clone(), vec![index]);
            }

            index += 1;
        }

        let moves_clone = self.moves.clone();


        for (_name, list) in names {
            if list.len() > 1 {
                // println!("\nmultiple moves for move name: {}", name);

                for index in &list {
                    let mv = &mut self.moves[*index];

                    let mut file_diff = false;
                    let mut rank_diff= false;

                    // kig på trækket
                    // sammenlign med alle andre træk og hvis de deler file, så slå det til, hvis den deler rank, slå det til

                    for other_index in &list {
                        if index == other_index { continue }
                        
                        let other_mv = &moves_clone[*other_index];

                        file_diff = file_diff || mv.file() != other_mv.file();
                        rank_diff = rank_diff || mv.rank() != other_mv.rank();
                    }

                    mv.make_name(&board, file_diff, rank_diff);
                }
            }
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






