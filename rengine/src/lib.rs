mod index;
mod r#move;
mod piece;
mod directions;

use index::Index;
use piece::{Color, PieceType, Piece};
use r#move::{FromTo, Move};



#[derive(Clone, Copy)]
struct Snapshot {
    index: Index,
    piece: Option<Piece>,
}

pub struct Board {
    board: [Option<Piece>; 64],
    moves: Vec<Move>,
    // meta
    turn: Color,
    castling: [Castling; 2], // 0: hvid, 1: sort
    phantom_pawn: Option<Index>,
    snapshots: Vec<Snapshot>,
}

pub struct Castling {
    queen_side: bool,
    king_side: bool,
}

#[derive(Default, Clone, Copy)]
pub struct MoveRules {
    only_capture: bool,
    only_empty: bool,
    limit: bool, // enten 8 eller 1. Ingen grund til andet valg
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum MoveReturn {
    Stop,
    Continue,
}

impl Board {
    /// generer en masse pseudo moves og gem dem
    pub fn generate_moves_and_save(&mut self) {
        let all_moves = self.generate_moves();
        self.moves = all_moves;
    }

    /// fjerner alle pseudo moves som ikke er gyldige fra self.moves
    pub fn remove_pseudo_moves(&mut self) {
        let mut real_moves = Vec::new();

        let binding = self.find_specific_pieces(PieceType::King, self.turn);
        let king = binding.first().unwrap();

        self.turn = self.turn.opposite();

        'main: for m in &self.moves.clone() {
            self.execute_move(m);
            let list = self.generate_moves();
            self.undo();

            for new_move in &list {
                for micro_move in &new_move.moves {
                    if micro_move.to == *king {
                        continue 'main;
                    }
                }
            }

            real_moves.push(m.clone());
        }

        println!("Real moves: {:?}", real_moves);
    }


    /// generer en masse pseudo moves og returnere dem
    fn generate_moves(&self) -> Vec<Move> {
        // gå alle brikkerne igennem
        let mut index = Index::new(0);

        let mut all_moves = Vec::new();
        
        for maybe_piece in self.board {
            // kommer kun forbi her hvis der faktisk er en brik og farven er i orden!
            if maybe_piece.is_none() { index.next(); continue }
            let piece = maybe_piece.unwrap();
            if self.turn != piece.color { index.next(); continue }


            // for fat i alle retninger som skal regnes ud fra brikken.
            let mut directions: Vec<((i32, i32), MoveRules)> = directions::get_directions_for_piece(piece.kind);
            let mut new_moves = Vec::new();

            match piece.kind {
                PieceType::King => {
                    let king = &index;

                    if self.castling[self.turn.as_index()].queen_side {
                        // tjek om de næste to felter er tomme
                        let d1 = Index::new_with_offset(king, (-1, 0));
                        let c1 = Index::new_with_offset(king, (-2, 0));
                        let b1 = Index::new_with_offset(king, (-3, 0));

                        if d1.is_empty(self) && c1.is_empty(self) && b1.is_empty(self) {
                            println!("Kongen kan rokere - O-O-O");
                            all_moves.push(Move {
                                name: "...".to_string(),
                                moves: vec![
                                    FromTo { // kongen
                                        from: king.clone(),
                                        to: Index::new_with_offset(king, (-2, 0)),
                                    },
                                    FromTo {
                                        from: Index::new_with_offset(king, (-4, 0)),
                                        to: Index::new_with_offset(king, (-1, 0)),
                                    },
                                ],
                                promotion: None,
                            });
                        }
                    }

                    if self.castling[self.turn.as_index()].king_side {
                        // tjek om de næste to felter er tomme
                        let f1 = Index::new_with_offset(king, (1, 0));
                        let g1 = Index::new_with_offset(king, (2, 0));

                        if f1.is_empty(self) && g1.is_empty(self) {
                            println!("Kongen kan rokere - O-O");

                            all_moves.push(Move {
                                name: "...".to_string(),
                                moves: vec![
                                    FromTo { // kongen
                                        from: king.clone(),
                                        to: Index::new_with_offset(king, (2, 0)),
                                    },
                                    FromTo {
                                        from: Index::new_with_offset(king, (3, 0)),
                                        to: Index::new_with_offset(king, (1, 0)),
                                    },
                                ],
                                promotion: None,
                            });
                        }
                    }
                }
                PieceType::Pawn => {

                    // vend retningen om hvis det er sort
                    if piece.color == Color::Black {
                        directions = directions.iter().map(|((x, y), r)| ((*x, -y), *r)).collect();
                    }


                    // tjek for en passant via fantom bonden
                    if let Some(phantom_pawn) = self.phantom_pawn {
                        let (x, y) = phantom_pawn.get_offset(&index);

                        if x.abs() == 1 && y.abs() == 1 {
                            // vi har en passant
                            new_moves.push(Move {
                                name: "...".to_string(),
                                moves: vec![
                                    FromTo { // først rykker vi den fjendtlige bonde hen til hvor vi gerne vil hen.
                                        from: Index::new_with_offset(&phantom_pawn, (0, self.turn.as_num() * -1)),
                                        to: phantom_pawn.clone(),
                                    },
                                    FromTo {
                                        from: index.clone(),
                                        to: phantom_pawn.clone(),
                                    },
                                ],
                                promotion: None,
                            });

                            println!("\n -> Passant! {:?}", Move {
                                name: "...".to_string(),
                                moves: vec![
                                    FromTo {
                                        from: index.clone(),
                                        to: phantom_pawn.clone(),
                                    },
                                ],
                                promotion: None,
                            });
                        }
                    }
                },
                _ => {}
            }

            // regn moves ud for hver retning
            for direction in directions {
                if index.get_square() == "a2" {
                    println!("Direction: {:?}", direction.0);
                }
                
                let specific_new_moves = self.gen_for_piece_in_direction(
                    &index,
                    direction.0,
                    direction.1,
                );

                // tilføj alle moves til listen
                new_moves.extend(specific_new_moves);
            }


            all_moves.extend(new_moves);

            index.next();
        }

        // println!("All moves: {:?}", all_moves);

        return all_moves;
    }

    /// generer moves for en brik i en bestemt retnning udfra de givne regler.
    /// bruges til at beregne gyldige træk for langt de fleste brikker.
    /// alt undtagen kongerokade og en passant faktisk :)
    fn gen_for_piece_in_direction(&self, index: &Index, (original_x, original_y): (i32, i32), rules: MoveRules) -> Vec<Move> {
        let mut moves = Vec::new();
        let (mut x, mut y) = (original_x, original_y);

        loop {
            let (res, state) = self.try_move(index, (x, y), &rules);

            if res {
                moves.push(Move {
                    name: "...".to_string(),
                    moves: vec![FromTo {
                        from: index.clone(),
                        to: Index::new_with_offset(index, (x, y)),
                    }],
                    promotion: None,
                });
            }

            if state == MoveReturn::Stop {
                break;
            }

            x += original_x;
            y += original_y;

            if rules.limit {
                break;
            }
        }

        moves
    }

    /// forsøger blot at rykke på en brik i en given retning indtil vi enten ikke længere befinder os på brættet
    /// eller brikken støder ind i en anden, hvor vi så må tage stilling til hvorvidt vi skal dræbbe eller lade vær
    pub fn try_move(&self, index: &Index, (x, y): (i32, i32), rules: &MoveRules) -> (bool, MoveReturn) {
        // tjek først hvis den nye position er indenfor brættet
        if !index.is_new_index_valid((x, y)) {
            return (false, MoveReturn::Stop);
        }

        
        let new_index = Index::new_with_offset(index, (x, y));
        let piece = self.try_get(&new_index);


        // vi har muligvis fanget en brik
        if let Some(piece) = piece {
            if piece.color != self.turn {
                // hvis det er modsat farve, så skal vi snuppe den!
                return (true, MoveReturn::Stop);
            }
            return (false, MoveReturn::Stop); // hvis ikke stopper vi omgående!
        } else {
            match rules.only_capture {
                true => (false, MoveReturn::Stop),
                false => (true, MoveReturn::Continue),
            }
        }
    }
    

    /// tager et move og udfører det på brættet.
    /// generer også et lille "snapshot", som undo funktionen kan bruge til at finde tilbage til spillets stadie før.
    pub fn execute_move(&mut self, m: &Move) {
        let mut snapshots = Vec::new();

        // gemmer ændringer
        for FromTo { from, to } in &m.moves {
            snapshots.push(Snapshot {
                index: from.clone(),
                piece: self.try_get(from),
            });

            snapshots.push(Snapshot {
                index: to.clone(),
                piece: self.try_get(to),
            });
        }

        // udfører hver enkelt move
        for FromTo { from, to } in &m.moves {
            // placerer brikken på den nye plads! Tager højde for promotion
            self.board[to.get()] = if let Some(promotion) = m.promotion {
                let color = self.get(from).color;
                
                Some(Piece {
                    kind: promotion,
                    color, // TODO: Problem
                })
            }
            else {
                self.board[from.get()]
            };


            // fjerner den gamle brik
            self.set(from, None);
        }

        // overskriver snapshots
        self.snapshots = snapshots;
    }

    /// tager det sidste og går tilbage!
    pub fn undo(&mut self) {
        let snapshots = self.snapshots.clone();

        // fjerner alle ændringer
        for Snapshot { index, piece } in snapshots {
            self.set(&index, piece)
        }

        self.snapshots.clear();
    }

    /// kun brugbart når vi tester, men funktionen logger hele brættet
    pub fn print(&self) {
        let mut lateral_count = 0;
        let mut y = 8;

        for index in 0..64 {
            if lateral_count >= 8 {
                lateral_count = 0;
                y -= 1;
            }

            let piece = self.board[index];

            let p_str = match piece {
                Some(p) => p.to_string(),
                None => "*".to_string(),
            };

            if lateral_count == 0 {
                print!("\n {}| {}", y, p_str);
            } else {
                print!(" {}", p_str);
            }

            lateral_count += 1;
        }

        print!("\n    ---------------");
        print!("\n    a b c d e f g h");

        print!("\n\n turn: {}\n", self.turn.to_string(),);

        print!("\n\n");
    }

    /// parser spillets stadie til fen kode
    pub fn to_fen(&self) -> &'static str {
        ""
    }

    /// parser en fen kode, så vi hurtigt kan komme i gang med at spille
    pub fn parse_fen(&mut self, fen: &str) {
        // forklarer hvor langt vi er i parsing processen
        // f.eks.
        // 1 = parsing af brikker
        // 2 = parsing af hvis tur det er
        let mut parsing_counter = 0;
        for part in fen.split_whitespace() {
            parsing_counter += 1;

            // fen parsing
            if parsing_counter == 1 {
                let iter = part.split("");

                let mut index = 0;

                for command in iter {
                    match command {
                        "1" => index += 1,
                        "2" => index += 2,
                        "3" => index += 3,
                        "4" => index += 4,
                        "5" => index += 5,
                        "6" => index += 6,
                        "7" => index += 7,
                        "8" => index += 8,
                        "9" => index += 9,
                        "/" => {}
                        "" => {}
                        command_letter => {
                            if let Ok(piece) = Piece::try_from_str(command_letter) {
                                self.board[index] = Some(piece);
                            }
                            index += 1;
                        }
                    }
                }
            }
            // turn
            if parsing_counter == 2 {
                self.turn = match part {
                    "b" => Color::Black,
                    _ => Color::White,
                };
            }
        }
    }


    // finder alle brikker af en bestemt type og farve
    pub fn find_specific_pieces(&self, kind: PieceType, color: Color) -> Vec<Index> {
        let mut collected_pieces = Vec::new();
        let mut index = 0;

        for maybe_piece in &self.board {
            if let Some(piece) = maybe_piece {
                if piece.kind == kind && piece.color == color {
                    collected_pieces.push(Index::new(index))
                }
            }

            index += 1;
        }

        collected_pieces
    }

    pub fn try_get(&self, index: &Index) -> Option<Piece> {
        self.board[index.get()]
    }

    pub fn get(&self, index: &Index) -> Piece {
        self.board[index.get()].unwrap()
    }

    pub fn set(&mut self, index: &Index, piece: Option<Piece>) {
        self.board[index.get()] = piece;
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Self {
            board: [None; 8 * 8],
            moves: Vec::new(),
            turn: Color::White,
            castling: [
                Castling {
                    king_side: true,
                    queen_side: true,
                },
                Castling {
                    king_side: true,
                    queen_side: true,
                },
            ],
            phantom_pawn: None,
            snapshots: Vec::new(),
        };

        board.parse_fen("pppppppp/rnbqkbnr/8/8/8/2b5/PPPPPPPP/RNBQKBNR");

        return board;
    }
}

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum PieceType {
//     Pawn,
//     Bishop,
//     Knight,
//     Rook,
//     Queen,
//     King,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub enum Color {
//     White,
//     Black,
// }

// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
// pub struct Piece {
//     piecetype: PieceType,
//     color: Color,
//     index: Option<usize>,
// }

// impl Piece {
//     fn try_from_str(str: &str) -> Option<Self> {
//         let color = if str == str.to_uppercase() {
//             Color::White
//         } else {
//             Color::Black
//         };

//         let piece = match str.to_lowercase().as_str() {
//             "p" => PieceType::Pawn,
//             "r" => PieceType::Rook,
//             "n" => PieceType::Knight,
//             "b" => PieceType::Bishop,
//             "q" => PieceType::Queen,
//             "k" => PieceType::King,
//             _ => return None,
//         };

//         Some(Piece {
//             piecetype: piece,
//             color,
//             index: None,
//         })
//     }

//     fn try_from_index(i: usize, b: &Board) -> Option<Self> {
//         match b.pieces[i] {
//             Some(mut piece) => {
//                 piece.index = Some(i);
//                 Some(piece)
//             }
//             None => None,
//         }
//     }

//     fn new(pt: PieceType, c: Color, i: usize) -> Self {
//         Self {
//             piecetype: pt,
//             color: c,
//             index: Some(i),
//         }
//     }

//     fn square_string(&self) -> String {
//         if self.index.is_none() {
//             "--".to_string();
//         }

//         let mut x = self.index.unwrap();
//         let mut y = 0;

//         while x >= 8 {
//             x -= 8;
//             y += 1;
//         }

//         let letters = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];

//         format!("{}{}", letters[x], y)
//     }

//     fn to_string(&self) -> String {
//         let p = match self.piecetype {
//             PieceType::Pawn => "p",
//             PieceType::Bishop => "b",
//             PieceType::Knight => "n",
//             PieceType::Rook => "r",
//             PieceType::Queen => "q",
//             PieceType::King => "k",
//         };

//         match self.color {
//             Color::White => p.to_uppercase(),
//             Color::Black => p.to_string(),
//         }
//     }
// }

// pub struct Board {
//     pub pieces: [Option<Piece>; 64],
//     pub turn: Color,
//     pub castling: (bool, bool, bool, bool),
// }

// impl Board {
//     pub fn default() -> Board {
//         Board::load_fen("pppppppp/rnbqkbnr/8/8/8/8/PPPPPPPP/RNBQKBNR")
//     }

//     pub fn load_fen(fen: impl ToString) -> Board {
//         Board {
//             pieces: [None; 64],
//             turn: Color::White,
//             castling: (false, false, false, false),
//         }
//         .parse_fen(fen.to_string().as_str())
//     }

//     pub fn find_pieces(&self, pt: PieceType) -> Vec<Piece> {
//         let mut vec = Vec::new();

//         for i in 0..64 {
//             match Piece::try_from_index(i, self) {
//                 Some(piece) => {
//                     if piece.piecetype == pt {
//                         vec.push(piece);
//                     }
//                 }
//                 None => {}
//             }
//         }

//         vec
//     }

//     pub fn parse_fen(mut self, fen: &str) -> Board {
//         let mut i = 0;
//         for part in fen.split_whitespace() {
//             i += 1;

//             // fen parsing
//             if i == 1 {
//                 let iter = part.split("");

//                 let mut index = 0;

//                 for p in iter {
//                     match p {
//                         "1" => index += 1,
//                         "2" => index += 2,
//                         "3" => index += 3,
//                         "4" => index += 4,
//                         "5" => index += 5,
//                         "6" => index += 6,
//                         "7" => index += 7,
//                         "8" => index += 8,
//                         "9" => index += 9,
//                         "/" => {}
//                         "" => {}
//                         str => {
//                             self.pieces[index] = Piece::try_from_str(str);
//                             index += 1;
//                         }
//                     }
//                 }

//                 // tjekker om der faktisk må castles!
//                 let kings = self.find_pieces(PieceType::King);
//                 let rooks = self.find_pieces(PieceType::Rook);

//                 if kings.len() != 2 {
//                     panic!("No king");
//                 }

//                 let mut w = false;
//                 let mut b = false;

//                 for king in kings {
//                     if king.color == Color::White {
//                         if king.square_string() == "e1" {
//                             w = true;
//                         }
//                     } else if king.color == Color::Black {
//                         if king.square_string() == "e8" {
//                             b = true;
//                         }
//                     }
//                 }

//                 println!("rooks: {:?}", rooks);

//                 for rook in rooks {
//                     println!(
//                         "rook: {:?} at square: {}",
//                         rook.piecetype,
//                         rook.square_string()
//                     );
//                     match rook.square_string().as_str() {
//                         "a1" => self.castling.0 = w,
//                         "h1" => self.castling.1 = w,
//                         "a8" => self.castling.2 = b,
//                         "h8" => self.castling.3 = b,
//                         _ => panic!("no square for piece: {:?}", rook),
//                     }
//                 }
//             }
//             // turn
//             if i == 2 {
//                 self.turn = match part {
//                     "b" => Color::Black,
//                     _ => Color::White,
//                 };
//             }
//             // castling rights
//             if i == 3 {
//                 for str in part.split("") {
//                     if str == "" {
//                         continue;
//                     }

//                     match str {
//                         "Q" => self.castling.0 = true,
//                         "K" => self.castling.1 = true,
//                         "q" => self.castling.2 = true,
//                         "k" => self.castling.3 = true,
//                         _ => self.castling = (false, false, false, false),
//                     }
//                 }
//             }
//         }

//         self
//     }

//     pub fn to_fen(&self) -> String {
//         let mut str = String::new();
//         let mut space = 0;
//         let mut x = 0;

//         for p in self.pieces {
//             if x >= 8 {
//                 x = 0;
//                 if space > 0 {
//                     str += &space.to_string();
//                     space = 0;
//                 }
//                 str += "/";
//             }
//             match p {
//                 Some(piece) => {
//                     if space > 0 {
//                         str += &space.to_string();
//                         space = 0;
//                     }
//                     str += &piece.to_string()
//                 }
//                 None => space += 1,
//             }

//             x += 1;
//         }

//         str
//     }

//     pub fn print(&self) {
//         let mut c = 0;
//         let mut y = 8;

//         for i in 0..64 {
//             if c >= 8 {
//                 c = 0;
//                 y -= 1;
//             }

//             let p = self.pieces[i];
//             let p_str = match p {
//                 Some(p) => p.to_string(),
//                 None => "*".to_string(),
//             };

//             if c == 0 {
//                 print!("\n {}| {}", y, p_str);
//             } else {
//                 print!(" {}", p_str);
//             }

//             c += 1;
//         }

//         print!("\n    ---------------");
//         print!("\n    a b c d e f g h");

//         print!(
//             "\n\n turn: {:?}\n castling rights: {:?}",
//             self.turn, self.castling
//         );

//         print!("\n\n");
//     }
// }
