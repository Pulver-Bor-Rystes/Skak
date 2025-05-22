use super::*;


impl Piece {
    pub fn to_str_img_format(&self) -> String {
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

    pub fn to_str_fen_format(&self) -> String {
        let kind = match self.kind {
            PieceType::Bishop => "b",
            PieceType::Knight => "n",
            PieceType::King => "k",
            PieceType::Pawn => "p",
            PieceType::Queen => "q",
            PieceType::Rook => "r",
        };

        match self.color {
            ChessColor::White => kind.to_uppercase(),
            ChessColor::Black => kind.to_string(),
        }
    }

    pub fn from_str(str: &str) -> Option<Self> {
        let kind = match str.to_lowercase().as_str() {
            "p" => PieceType::Pawn,
            "r" => PieceType::Rook,
            "n" => PieceType::Knight,
            "b" => PieceType::Bishop,
            "q" => PieceType::Queen,
            "k" => PieceType::King,
            _ => return None,
        };

        let color = match str == str.to_lowercase() {
            true => ChessColor::Black,
            false => ChessColor::White,
        };

        Some(Self {
            kind,
            color,
            has_moved: false,
        })
    }

    // pub fn to_str(&self) -> String {
    //     let kind = match self.kind {
    //         PieceType::Bishop => "b",
    //         PieceType::Knight => "n",
    //         PieceType::King => "k",
    //         PieceType::Pawn => "p",
    //         PieceType::Queen => "q",
    //         PieceType::Rook => "r",
    //     };

    //     match self.color {
    //         ChessColor::White => kind.to_uppercase(),
    //         ChessColor::Black => kind.to_string(),
    //     }
    // }
}


impl PieceType {
    pub fn to_str_name(&self) -> String {
        match self {
            PieceType::Pawn => "pawn",
            PieceType::Rook => "rook",
            PieceType::Bishop => "bishop",
            PieceType::Knight => "knight",
            PieceType::Queen => "queen",
            PieceType::King => "king",
        }.to_string()
    }

    pub fn to_str_move_name_format(&self) -> String {
        match self {
            PieceType::Pawn => "",
            PieceType::Rook => "r",
            PieceType::Bishop => "b",
            PieceType::Knight => "n",
            PieceType::Queen => "q",
            PieceType::King => "k",
        }.to_string().to_uppercase()
    }
}


