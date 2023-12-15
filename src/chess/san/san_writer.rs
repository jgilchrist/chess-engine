use crate::chess::game::Game;
use crate::chess::moves::Move;
use crate::chess::piece::{PieceKind, PromotionPieceKind};
use crate::chess::player::Player;
use crate::chess::san;
use crate::chess::square::squares;

#[derive(Debug, Eq, PartialEq)]
enum AmbiguityResolution {
    None,
    File,  // Two pieces can move to the same square - specify the file
    Rank,  // Two pieces on the same file can move to the same square - specify the rank
    Exact, // Two pieces on the same file or rank can move to the same square - specify the exact source square
}

fn format_move_t<const PLAYER: bool>(game: &Game, mv: Move) -> String {
    let from = mv.src;
    let to = mv.dst;

    let piece = game.board.piece_at(from).unwrap();

    let king_start = squares::king_start::<PLAYER>();
    if piece.kind == PieceKind::King && from == king_start {
        let kingside_castle_dest = squares::kingside_castle_dest::<PLAYER>();
        if to == kingside_castle_dest {
            return san::KINGSIDE_CASTLE.to_string();
        }

        let queenside_castle_dest = squares::queenside_castle_dest::<PLAYER>();
        if to == queenside_castle_dest {
            return san::QUEENSIDE_CASTLE.to_string();
        }
    }

    let mut game_after_move = game.clone();
    game_after_move.make_move(mv);
    let places_opponent_in_check = game_after_move.is_king_in_check();

    let captured_piece = game.board.piece_at(to);
    let is_en_passant = piece.kind == PieceKind::Pawn && Some(to) == game.en_passant_target;

    let capture_happened = captured_piece.is_some() || is_en_passant;

    let ambiguity_resolution_required = required_ambiguity_resolution(game, mv);

    let piece_identifier: &'static str = match piece.kind {
        PieceKind::Pawn => {
            if capture_happened {
                from.file().notation()
            } else {
                ""
            }
        }
        PieceKind::Knight => "N",
        PieceKind::Bishop => "B",
        PieceKind::Rook => "R",
        PieceKind::Queen => "Q",
        PieceKind::King => "K",
    };

    let ambiguity_resolution = match ambiguity_resolution_required {
        AmbiguityResolution::None => String::new(),
        AmbiguityResolution::File => from.file().notation().to_string(),
        AmbiguityResolution::Rank => from.rank().notation().to_string(),
        AmbiguityResolution::Exact => from.notation(),
    };

    let capture_x = if capture_happened {
        san::CAPTURE.to_string()
    } else {
        String::new()
    };

    let destination_notation = to.notation();

    let promotion_specifier = match mv.promotion {
        None => "",
        Some(p) => match p {
            PromotionPieceKind::Knight => "=N",
            PromotionPieceKind::Bishop => "=B",
            PromotionPieceKind::Rook => "=R",
            PromotionPieceKind::Queen => "=Q",
        },
    };

    let opponent_in_check_specifier = if places_opponent_in_check {
        san::CHECK.to_string()
    } else {
        String::new()
    };

    format!("{piece_identifier}{ambiguity_resolution}{capture_x}{destination_notation}{promotion_specifier}{opponent_in_check_specifier}")
}

#[allow(unused)]
pub fn format_move(game: &Game, mv: Move) -> String {
    match game.player {
        Player::White => format_move_t::<true>(game, mv),
        Player::Black => format_move_t::<false>(game, mv),
    }
}

fn required_ambiguity_resolution(game: &Game, mv: Move) -> AmbiguityResolution {
    let from = mv.src;
    let to = mv.dst;

    let piece = game.board.piece_at(from).unwrap();
    if piece.kind == PieceKind::Pawn || piece.kind == PieceKind::King {
        return AmbiguityResolution::None;
    }

    let moves = game.moves();

    let potentially_ambiguous_moves: Vec<Move> = moves
        .into_iter()
        .filter(|m| {
            // A move is potentially ambiguous if:

            // It moves to the same square our move
            m.dst == to &&

                // The kind of piece being moved is the same
                game.board.piece_at(m.src).unwrap().kind == piece.kind &&

                // It's not the exact same move
                *m != mv
        })
        .collect();

    let ambiguity_by_file = potentially_ambiguous_moves
        .iter()
        .any(|m| m.src.file() == mv.src.file());

    let ambiguity_by_rank = potentially_ambiguous_moves
        .iter()
        .any(|m| m.src.rank() == mv.src.rank());

    match (ambiguity_by_file, ambiguity_by_rank) {
        (false, false) => AmbiguityResolution::None,
        (true, false) => AmbiguityResolution::Rank,
        (false, true) => AmbiguityResolution::File,
        (true, true) => AmbiguityResolution::Exact,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::fen;
    use crate::chess::game::Game;
    use crate::chess::square::squares::all::*;

    fn test_san_string(fen: &'static str, mv: Move, expected_san: &'static str) {
        crate::init();

        let game = Game::from_fen(fen).unwrap();
        let san = format_move(&game, mv);

        assert_eq!(&san, expected_san);
    }

    #[test]
    fn san_simple_pawn_move() {
        test_san_string(fen::START_POS, Move::new(E2, E4), "e4");
    }

    #[test]
    fn san_simple_pawn_capture() {
        test_san_string(
            "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 2",
            Move::new(E4, D5),
            "exd5",
        );
    }

    #[test]
    fn san_simple_knight_move() {
        test_san_string(fen::START_POS, Move::new(B1, C3), "Nc3");
    }

    #[test]
    fn san_test_en_passant() {
        test_san_string(
            "rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq d6 0 3",
            Move::new(E5, D6),
            "exd6",
        );
    }

    #[test]
    fn san_promotion() {
        let promotion_fen = "k7/7P/8/8/8/8/8/7K w - - 0 1";

        test_san_string(
            promotion_fen,
            Move::promotion(H7, H8, PromotionPieceKind::Knight),
            "h8=N",
        );

        test_san_string(
            promotion_fen,
            Move::promotion(H7, H8, PromotionPieceKind::Bishop),
            "h8=B",
        );

        test_san_string(
            promotion_fen,
            Move::promotion(H7, H8, PromotionPieceKind::Rook),
            "h8=R+",
        );

        test_san_string(
            promotion_fen,
            Move::promotion(H7, H8, PromotionPieceKind::Queen),
            "h8=Q+",
        );
    }

    #[test]
    fn san_file_ambiguity() {
        crate::init();

        let fen = "R6R/8/8/8/8/8/8/1k4K1 w - - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let ambiguous_move = Move::new(A8, B8);

        assert_eq!(
            required_ambiguity_resolution(&game, ambiguous_move),
            AmbiguityResolution::File
        );

        test_san_string(fen, ambiguous_move, "Rab8+");
    }

    #[test]
    fn san_rank_ambiguity() {
        crate::init();

        let fen = "R7/8/8/8/8/1k4K1/8/R7 w - - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let ambiguous_move = Move::new(A1, A3);

        assert_eq!(
            required_ambiguity_resolution(&game, ambiguous_move),
            AmbiguityResolution::Rank
        );

        test_san_string(fen, ambiguous_move, "R1a3+");
    }

    #[test]
    fn san_exact_ambiguity() {
        crate::init();

        let fen = "1k1K4/8/8/8/4Q2Q/8/8/7Q w - - 0 1";
        let game = Game::from_fen(fen).unwrap();
        let ambiguous_move = Move::new(H4, E1);

        assert_eq!(
            required_ambiguity_resolution(&game, ambiguous_move),
            AmbiguityResolution::Exact
        );

        test_san_string(fen, ambiguous_move, "Qh4e1");
    }

    #[test]
    fn san_castling() {
        let fen = "1k6/8/8/8/8/8/8/R3K2R w KQ - 0 1";

        test_san_string(fen, Move::new(E1, G1), "O-O");
        test_san_string(fen, Move::new(E1, C1), "O-O-O");
    }

    #[test]
    fn san_plus_for_check() {
        test_san_string(
            "k7/6P1/8/8/8/8/8/K7 w - - 0 1",
            Move::promotion(G7, G8, PromotionPieceKind::Queen),
            "g8=Q+",
        );
    }
}
