use crate::eval::IncrementalEvalFields;
use chess::direction::Direction;
use chess::game::Game;
use chess::movegen::MoveTypes;
use chess::moves::Move;
use chess::piece::{Piece, PieceKind};
use chess::player::Player;
use chess::square::{squares, Square};
use chess::zobrist::ZobristHash;
use color_eyre::Result;

#[derive(Debug, Clone)]
struct History {
    pub incremental_eval: IncrementalEvalFields,
}

#[derive(Debug, Clone)]
pub struct EngineGame {
    pub game: Game,
    pub incremental_eval: IncrementalEvalFields,
    history: Vec<History>,
}

impl Default for EngineGame {
    fn default() -> Self {
        Self::new()
    }
}

impl EngineGame {
    pub fn new() -> Self {
        Self::from_game(Game::new())
    }

    pub fn from_game(game: Game) -> Self {
        let incremental_eval_fields = IncrementalEvalFields::init(&game);

        Self {
            game,
            incremental_eval: incremental_eval_fields,
            history: Vec::new(),
        }
    }

    pub fn from_fen(fen: &str) -> Result<Self> {
        let game = Game::from_fen(fen)?;
        Ok(Self::from_game(game))
    }

    pub fn moves(&self) -> Vec<Move> {
        self.game.moves()
    }

    pub fn moves_with_type(&self, move_types: &MoveTypes) -> Vec<Move> {
        self.game.moves_with_type(move_types)
    }

    pub fn is_stalemate_by_fifty_move_rule(&self) -> bool {
        self.game.is_stalemate_by_fifty_move_rule()
    }

    pub fn is_repeated_position(&self) -> bool {
        self.game.is_repeated_position()
    }

    pub fn is_stalemate_by_repetition(&self) -> bool {
        self.game.is_stalemate_by_repetition()
    }

    #[inline(always)]
    pub fn is_king_in_check(&self) -> bool {
        self.game.board.king_in_check(self.game.player)
    }

    #[inline(always)]
    pub fn player(&self) -> Player {
        self.game.player
    }

    #[inline(always)]
    pub fn zobrist(&self) -> ZobristHash {
        self.game.zobrist.clone()
    }

    fn set_at(&mut self, sq: Square, piece: Piece) {
        self.incremental_eval.set_at(sq, piece);
    }

    fn remove_at(&mut self, sq: Square) -> Piece {
        let removed_piece = self.game.board.piece_at(sq).unwrap();
        self.incremental_eval.remove_at(sq, removed_piece);
        removed_piece
    }

    pub fn make_move(&mut self, mv: &Move) {
        let from = mv.src;
        let to = mv.dst;

        let player = self.game.player;

        let moved_piece = self.game.board.piece_at(from).unwrap();
        let maybe_captured_piece = self.game.board.piece_at(to);

        // Capture the irreversible aspects of the position so that they can be restored
        // if we undo this move.
        let history = History {
            incremental_eval: self.incremental_eval.clone(),
        };

        self.history.push(history);

        self.remove_at(from);

        if maybe_captured_piece.is_some() {
            self.remove_at(to);
        }

        if let Some(promoted_to) = mv.promotion {
            let promoted_piece = Piece::new(player, promoted_to.piece());
            self.set_at(to, promoted_piece);
        } else {
            self.set_at(to, moved_piece);
        }

        let pawn_move_direction = Direction::pawn_move_direction(player);

        // If we moved a pawn to the en passant target, this was an en passant capture, so we
        // remove the captured pawn from the board.
        if let Some(en_passant_target) = self.game.en_passant_target {
            if moved_piece.kind == PieceKind::Pawn && to == en_passant_target {
                // Remove the piece behind the square the pawn just moved to
                let capture_square = to.in_direction(!pawn_move_direction);
                self.remove_at(capture_square);
            }
        }

        if moved_piece.kind == PieceKind::King && from == squares::king_start(player) {
            // We're castling!
            if to == squares::kingside_castle_dest(player) {
                let rook_remove_square = squares::kingside_rook_start(player);
                let rook_add_square = squares::kingside_rook_castle_end(player);

                let rook = self.remove_at(rook_remove_square);
                self.set_at(rook_add_square, rook);
            } else if to == squares::queenside_castle_dest(player) {
                let rook_remove_square = squares::queenside_rook_start(player);
                let rook_add_square = squares::queenside_rook_castle_end(player);

                let rook = self.remove_at(rook_remove_square);
                self.set_at(rook_add_square, rook);
            }
        }

        self.game.make_move(mv);
    }

    pub fn undo_move(&mut self) {
        let history = self.history.pop().unwrap();
        self.incremental_eval = history.incremental_eval;

        self.game.undo_move();
    }
}