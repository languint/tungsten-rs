use chess::{Board, Color, Piece, ALL_SQUARES};

pub const PAWN_WORTH: f32 = 1.0;
pub const KNIGHT_WORTH: f32 = 3.0;
pub const BISHOP_WORTH: f32 = 3.5;
pub const ROOK_WORTH: f32 = 5.0;
pub const QUEEN_WORTH: f32 = 9.0;

pub type MateralResult = (f32, f32);

#[inline]
pub fn get_piece_worth(piece: &Piece) -> f32 {
    match piece {
        Piece::Pawn => PAWN_WORTH,
        Piece::Knight => KNIGHT_WORTH,
        Piece::Bishop => BISHOP_WORTH,
        Piece::Rook => ROOK_WORTH,
        Piece::Queen => QUEEN_WORTH,
        Piece::King => 0.0,
    }
} 

pub fn get_material(board: &Board) -> MateralResult {
    let mut w: f32 = 0.0;
    let mut b: f32 = 0.0;

    for square in ALL_SQUARES {
        if let Some(piece) = board.piece_on(square) {
            let worth = get_piece_worth(&piece);
            if let Some(c) = board.color_on(square) {
                if c == Color::White {
                    w += worth;
                } else {
                    b += worth;
                }
            } 
        }  
    }

    (w, b)
}

#[inline]
pub fn get_material_score(board: &Board) -> f32 {
    let (w, b) = get_material(board);
    w - b
}