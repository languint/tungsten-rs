use chess::{Board, BoardStatus, ChessMove, EMPTY, File, Game, MoveGen, Piece, Rank, Square};

#[inline]
fn file_to_string(file: &File) -> String {
    match file {
        File::A => "a",
        File::B => "b",
        File::C => "c",
        File::D => "d",
        File::E => "e",
        File::F => "f",
        File::G => "g",
        File::H => "h",
    }
    .to_string()
}

#[inline]
fn rank_to_string(rank: &Rank) -> String {
    match rank {
        Rank::First => "1",
        Rank::Second => "2",
        Rank::Third => "3",
        Rank::Fourth => "4",
        Rank::Fifth => "5",
        Rank::Sixth => "6",
        Rank::Seventh => "7",
        Rank::Eighth => "8",
    }
    .to_string()
}
fn to_algebraic(board: &Board, mv: &ChessMove) -> Result<String, String> {
    let mut str = String::new();

    let source_piece = board
        .piece_on(mv.get_source())
        .ok_or_else(|| "Source piece is None!".to_string())?;
    let is_capture = board.piece_on(mv.get_dest()).is_some()
        || (source_piece == Piece::Pawn && mv.get_dest().get_file() != mv.get_source().get_file());
    
    if source_piece == Piece::King
        && (mv.get_source().get_file() as i8 - mv.get_dest().get_file() as i8).abs() > 1
    {
        return Ok(if mv.get_dest().get_file() == File::G {
            "O-O"
        } else {
            "O-O-O"
        }
        .into());
    }

    let piece_str = match source_piece {
        Piece::Pawn => "",
        Piece::Knight => "N",
        Piece::Rook => "R",
        Piece::Bishop => "B",
        Piece::Queen => "Q",
        Piece::King => "K",
    };
    str.push_str(piece_str);

    if source_piece != Piece::Pawn {
        let mut competitors = vec![];
        let mut legal_moves = MoveGen::new_legal(board);
        for m in &mut legal_moves {
            if m.get_dest() == mv.get_dest()
                && board.piece_on(m.get_source()) == Some(source_piece)
                && m.get_source() != mv.get_source()
            {
                competitors.push(m);
            }
        }

        if !competitors.is_empty() {
            let mut file_needed = false;
            let mut rank_needed = false;

            if competitors
                .iter()
                .any(|c| c.get_source().get_file() == mv.get_source().get_file())
            {
                rank_needed = true;
            }

            if competitors
                .iter()
                .any(|c| c.get_source().get_rank() == mv.get_source().get_rank())
            {
                file_needed = true;
            }

            if !file_needed && !rank_needed {
                file_needed = true;
            }

            if file_needed {
                str.push_str(&file_to_string(&mv.get_source().get_file()));
            }
            if rank_needed {
                str.push_str(&rank_to_string(&mv.get_source().get_rank()));
            }
        }
    } else if is_capture {
        str.push_str(&file_to_string(&mv.get_source().get_file()));
    }

    if is_capture {
        str.push('x');
    }

    str.push_str(&mv.get_dest().to_string());

    if let Some(prom_piece) = mv.get_promotion() {
        str.push('=');
        str.push_str(&match prom_piece {
            Piece::Knight => "N",
            Piece::Rook => "R",
            Piece::Bishop => "B",
            Piece::Queen => "Q",
            _ => unreachable!(),
        });
    }

    let board_after = board.make_move_new(*mv);
    match board_after.status() {
        BoardStatus::Checkmate => str.push('#'),
        BoardStatus::Ongoing => {
            if board_after.checkers() != &EMPTY {
                str.push('+');
            }
        }
        _ => {}
    }

    Ok(str)
}

pub fn to_pgn(starting_board: &Board, moves: &Vec<ChessMove>) -> String {
    let mut pgn_string = String::new();

    let mut game = Game::new_with_board(starting_board.clone());

    for (i, mv) in moves.iter().enumerate() {
        if i % 2 == 0 {
            pgn_string.push_str(&format!("{}. ", (i / 2) + 1));
        }

        let notation = to_algebraic(&game.current_position(), mv)
            .expect("Failed to convert a legal move to algebraic notation.");

        pgn_string.push_str(&notation);

        game.make_move(*mv);

        if i % 2 == 1 {
            pgn_string.push('\n');
        } else {
            pgn_string.push(' ');
        }
    }

    pgn_string.trim().to_string()
}
