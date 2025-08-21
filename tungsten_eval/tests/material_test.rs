#[cfg(test)]
mod material_test {
    use std::{str::FromStr, sync::Arc};

    use chess::Board;

    #[test]
    fn material_eq() {
        let board = Board::from_str("3r2k1/2q5/8/8/1K5Q/8/8/1R6 w - - 0 1").expect("We should have a board");
        let evaluator = tungsten_eval::Evaluator::new(Arc::from(board));
        assert_eq!(evaluator.evaluate(), 0.0, "These should be equal")
    }

    #[test]
    fn material_diff() {
        let board = Board::from_str("3r2k1/2q5/8/8/1K6/8/8/1R6 w - - 0 1").expect("We should have a board");
        let evaluator = tungsten_eval::Evaluator::new(Arc::from(board));
        assert_ne!(evaluator.evaluate(), 0.0, "These should be different");
        assert_eq!(evaluator.evaluate(), -9.0, "These should be different");
    }
}