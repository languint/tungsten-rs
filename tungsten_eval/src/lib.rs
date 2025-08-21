pub mod search;

use std::sync::Arc;

use chess::Board;

use crate::evaluations::material;

pub mod evaluations;

pub struct Evaluator {
    pub board: Arc<Board>,
}

pub type Score = f32;

pub const SCORE_DRAW: Score = 0.0;
pub const SCORE_WHITE_WIN: Score = f32::INFINITY;
pub const SCORE_BLACK_WIN: Score = f32::NEG_INFINITY;

pub const MATERIAL_WEIGHT: Score = 1.0;

impl Evaluator {
    pub fn new(board: Arc<Board>) -> Self {
        Self { board }
    }

    /// Returns the score of the position, positive score means black is winning
    pub fn evaluate(&self) -> Score {
        let mut score = 0.0;

        score += MATERIAL_WEIGHT * material::get_material_score(&self.board);

        score
    }
}