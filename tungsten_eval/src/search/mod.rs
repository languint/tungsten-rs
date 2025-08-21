use std::sync::Arc;

use chess::{Board, ChessMove, Color, EMPTY, MoveGen};

use crate::{Evaluator};

pub struct Search {
    pub board: Arc<Board>,
    pub evaluator: Evaluator,
}

pub type AlphaBetaResult = (f32, Option<ChessMove>);

impl Search {
    pub fn new(board: &Board) -> Self {
        Self {
            board: Arc::new(*board),
            evaluator: Evaluator {
                board: Arc::new(*board),
            },
        }
    }

    fn alpha_min(&self, alpha: f32, mut beta: f32, depth: usize) -> AlphaBetaResult {
        let move_gen = MoveGen::new_legal(&self.board);

        if move_gen.len() == 0 {
            return if self.board.checkers() == &EMPTY {
                (0.0, None)
            } else {
                (f32::INFINITY, None)
            };
        }

        if depth == 0 {
            return (self.evaluator.evaluate(), None);
        }

        let mut best_score = f32::INFINITY;
        let mut best_move: Option<ChessMove> = None;

        for mv in move_gen {
            let new_board = self.board.make_move_new(mv);
            let searcher = Search {
                board: Arc::new(new_board),
                evaluator: Evaluator {
                    board: Arc::new(new_board),
                },
            };

            let (score, _move) = searcher.alpha_max(alpha, beta, depth - 1);

            if score < best_score {
                best_score = score;
                best_move = Some(mv);
            }

            beta = beta.min(best_score);

            if beta <= alpha {
                break;
            }
        }
        (best_score, best_move)
    }

    fn alpha_max(&self, mut alpha: f32, beta: f32, depth: usize) -> AlphaBetaResult {
        let move_gen = MoveGen::new_legal(&self.board);

        if move_gen.len() == 0 {
            return if self.board.checkers() == &EMPTY {
                (0.0, None)
            } else {
                (f32::NEG_INFINITY, None)
            };
        }

        if depth == 0 {
            return (self.evaluator.evaluate(), None);
        }

        let mut best_score = f32::NEG_INFINITY;
        let mut best_move: Option<ChessMove> = None;

        for mv in move_gen {
            let new_board = self.board.make_move_new(mv);
            let searcher = Search {
                board: Arc::new(new_board),
                evaluator: Evaluator {
                    board: Arc::new(new_board),
                },
            };

            let (score, _move) = searcher.alpha_min(alpha, beta, depth - 1);

            if score > best_score {
                best_score = score;
                best_move = Some(mv);
            }

            alpha = alpha.max(best_score);

            if alpha >= beta {
                break;
            }
        }
        (best_score, best_move)
    }

    pub fn alpha_beta(&self, depth: usize) -> AlphaBetaResult {
        let alpha = f32::NEG_INFINITY;
        let beta = f32::INFINITY;

        match self.board.side_to_move() {
            Color::White => self.alpha_max(alpha, beta, depth),
            Color::Black => self.alpha_min(alpha, beta, depth),
        }
    }
}
