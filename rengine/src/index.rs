use crate::Board;

const SQUARES: [&str; 64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];

const BOARDX12: [i32; 64] = [
    26, 27, 28, 29, 30, 31, 32, 33, 38, 39, 40, 41, 42, 43, 44, 45, 50, 51, 52, 53, 54, 55, 56, 57,
    62, 63, 64, 65, 66, 67, 68, 69, 74, 75, 76, 77, 78, 79, 80, 81, 86, 87, 88, 89, 90, 91, 92, 93,
    98, 99, 100, 101, 102, 103, 104, 105, 110, 111, 112, 113, 114, 115, 116, 117,
];

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct Index(usize);

impl Index {
    pub fn new(val: usize) -> Self {
        Index(val)
    }

    pub fn next(&mut self) {
        self.0 += 1;
    }

    pub fn new_with_offset(index: &Index, (x, y): (i32, i32)) -> Index {
        Index((index.get() as i32 + x - y * 8) as usize)
    }

    pub fn new_from_square(square: &str) -> Index {
        let index = SQUARES.iter().position(|&v| v == square).unwrap();
        Index(index)
    }

    pub fn get(&self) -> usize {
        self.0
    }

    pub fn get_ref(&self) -> &usize {
        &self.0
    }

    pub fn get_offset(&self, other: &Index) -> (i32, i32) {
        let index = self.0 as i32;
        let other_index = other.0 as i32;
        let x = index % 8 - other_index % 8;
        let y = index / 8 - other_index / 8;
        (x, y)
    }

    pub fn get_square(&self) -> &str {
        SQUARES[self.0]
    }

    pub fn is_new_index_valid(&self, (x, y): (i32, i32)) -> bool {
        // convert to 12x12 index
        let mut index_12x12 = convert(self.0) as i32;
        // make the move
        index_12x12 += x;
        index_12x12 -= y * 12; // omvendt sådan at en retning på 1 går fremad set fra hvids perspektiv
                               // check if the index is in the list of indexes
        BOARDX12.contains(&index_12x12)
    }

    pub fn is_empty(&self, board: &Board) -> bool {
        board.try_get(self).is_none()
    }
}

fn convert(v: usize) -> usize {
    let mut index = v;
    let mut result = 26;

    while index > 7 {
        result += 12;
        index -= 8;
    }

    result + index
}
