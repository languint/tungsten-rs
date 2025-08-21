#[cfg(test)]
mod search_test {
    use std::{str::FromStr};

    use chess::{Board, ChessMove, Square};
    use tungsten_eval::search::Search;

    #[test]
    fn draw_kings() {
        let board = Board::from_str("3k4/8/8/8/8/8/8/3K4 w - - 0 1").expect("We should have a board");
        let searcher = Search::new(&board);
        let (score, _) = searcher.alpha_beta(1);
        assert_eq!(score, 0.0, "This should be a draw")
    }

    #[test]
    fn white_mate_one() {
        let board = Board::from_str("3k4/Q7/3K4/8/8/8/8/8 w - - 0 1").expect("We should have a board");
        let searcher = Search::new(&board);
        let (score, best_move) = searcher.alpha_beta(1);
        assert!(score > 0.0, "White should be winning");
        assert_eq!(best_move, Some(ChessMove::new(Square::A7, Square::D7, None)));
    }

    #[test]
    fn black_prevent_extend_mate() {
        let board = Board::from_str("3k4/R6R/8/5n2/8/8/8/7K b - - 0 1").expect("We should have a board");
        let searcher = Search::new(&board);
        let (score, best_move) = searcher.alpha_beta(2);
        assert!(score > 0.0, "White should be winning");
        assert_eq!(best_move, Some(ChessMove::new(Square::F5, Square::G3, None)), "Best move should be to extend checkmate");
    }
}