use std::collections::HashMap;

use crate::{index::Index, piece::PieceType};

#[derive(Clone)]
pub struct Move {
    pub name: String, /// Kan først udarbejdes når man kender alle andre moves!
    pub moves: Vec<FromTo>,
    pub promotion: Option<PieceType>,
}

#[derive(Clone, Copy)]
pub struct FromTo {
    pub from: Index,
    pub to: Index,
}


impl std::fmt::Debug for FromTo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.from.get_square(), self.to.get_square())
    }
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.moves)
    }
}


// Jeg vil gerne lave en funktion som navngiver alle trækkene!

// impl Move {
//     pub fn compile_names(list: &mut Vec<Move>) {
//         let mut sorted_list: HashMap<Index, Vec<&mut Move>> = HashMap::new();

//         for m in list {
//             let key = m.moves[0].to;

//             // hvis destinationen endnu ikke er i listen, så tilføj den
//             if sorted_list.contains_key(&key) {
//                 sorted_list.insert(m.moves[0].to, vec![m]);
//             }
//             else {
//                 let values = sorted_list.get_mut(&key).unwrap();
//                 values.push(m);
//             }
//         }

//         for (target, moves) in sorted_list {
//             let moves_len = moves.len();

//             for m in moves {
//                 if moves_len == 1 {
//                     m.name = m.moves[0].to.get_square().to_string();
//                 }
//                 else {
//                     let mut name = String::new();
//                     for m in moves {
//                         name.push_str(&m.moves[0].from.get_square());
//                     }
//                     moves[0].name = name;
//                 }
//             }
//         }
//     }
// }