use std::{str::FromStr, time::{Duration, Instant}};

use chess::{Board, ChessMove, Game};
use clap::Parser;
use tungsten_eval::search::Search;

use crate::cli::{CLI_BLUE_HEADER, CLI_RED_HEADER, CLI_YELLOW_HEADER, Cli};

mod cli;
mod pgn;
mod perf;

const STARTING_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        cli::Commands::PlaySelf {
            position,
            depth,
            output_pgn,
        } => {
            cli::log_header(
                "cmd",
                format!("play-self [position = {position}, depth = {depth}]").as_str(),
                0,
                Some(CLI_BLUE_HEADER),
            );
            let board = Board::from_str(&position);

            if board.is_err() {
                cli::log_header("error", "Invalid FEN position", 0, Some(CLI_RED_HEADER));
                return;
            }

            let board = board.unwrap();
            let mut game = Game::new_with_board(board);

            let mut moves: Vec<ChessMove> = Vec::new();

            let mut total_time = Duration::new(0, 0);

            loop {
                let search = Search::new(&game.current_position());
                let start = Instant::now();

                let (score, best_move) = search.alpha_beta(depth);

                let end = Instant::now();

                let time = end - start;
                total_time += time; 
                let time = time.as_millis();

                if game.can_declare_draw() {
                    game.declare_draw();
                }

                if let Some(result) = game.result() {
                    cli::log_header(
                        "eval",
                        format!("game finished result={:?} total_time={}ms", result, total_time.as_millis()).as_str(),
                        0,
                        Some(CLI_YELLOW_HEADER),
                    );
                    if output_pgn {
                        println!("{}", pgn::to_pgn(&board, &moves));
                    }
                    break;
                }

                cli::log_header(
                    "eval",
                    format!(
                        "score={score} best_move={:?} time={}ms",
                        best_move.map(|m| (m.get_source().to_string(), m.get_dest().to_string())),
                        time
                    )
                    .as_str(),
                    2,
                    Some(CLI_YELLOW_HEADER),
                );

                if let Some(mv) = best_move {
                    game.make_move(mv);
                    moves.push(mv);
                } else {
                    cli::log_header(
                        "eval",
                        format!("game finished [result={:?},total_time={}ms]", game.result(), total_time.as_millis()).as_str(),
                        0,
                        Some(CLI_YELLOW_HEADER),
                    );
                    if output_pgn {
                        println!("{}", pgn::to_pgn(&board, &moves));
                    }
                    break;
                }
            }
        }
    }
}
