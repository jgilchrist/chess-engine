use chess::piece::PieceKind;
use chess::{board::PlayerPieces, game::Game};

use super::Eval;

pub fn material_diff(game: &Game) -> Eval {
    white_piece_value(game) + black_piece_value(game)
}

fn white_piece_value(game: &Game) -> Eval {
    count_piece_values(&game.board.white_pieces)
}

fn black_piece_value(game: &Game) -> Eval {
    -count_piece_values(&game.board.black_pieces)
}

fn count_piece_values(pieces: &PlayerPieces) -> Eval {
    Eval(
        i32::from(pieces.pawns.count()) * PieceKind::value_of(PieceKind::Pawn)
            + i32::from(pieces.knights.count()) * PieceKind::value_of(PieceKind::Knight)
            + i32::from(pieces.bishops.count()) * PieceKind::value_of(PieceKind::Bishop)
            + i32::from(pieces.rooks.count()) * PieceKind::value_of(PieceKind::Rook)
            + i32::from(pieces.queens.count()) * PieceKind::value_of(PieceKind::Queen),
    )
}
