use crate::piece::Piece;
use crate::squares::all::*;
use crate::zobrist::ZobristHash;
use crate::{
    board::Board,
    direction::Direction,
    fen, move_tables,
    movegen::generate_moves,
    moves::Move,
    piece::PieceKind,
    player::Player,
    square::Square,
    squares::{self, Squares},
    zobrist,
};
use anyhow::Result;

#[derive(Debug)]
pub enum MoveError {
    InvalidMove,
}

#[derive(Debug, Copy, Clone)]
pub enum CastleRightsSide {
    Kingside,
    Queenside,
}

impl CastleRightsSide {
    pub const N: usize = 2;

    #[must_use]
    pub fn array_idx(&self) -> usize {
        match self {
            Self::Kingside => 0,
            Self::Queenside => 1,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CastleRights {
    pub king_side: bool,
    pub queen_side: bool,
}

impl CastleRights {
    #[must_use]
    pub const fn can_castle(&self) -> bool {
        self.king_side || self.queen_side
    }

    #[must_use]
    pub const fn none() -> Self {
        Self {
            king_side: false,
            queen_side: false,
        }
    }

    #[must_use]
    pub const fn without_kingside(&self) -> Self {
        Self {
            king_side: false,
            queen_side: self.queen_side,
        }
    }

    pub fn remove_kingside_rights(&mut self) {
        self.king_side = false;
    }

    #[must_use]
    pub const fn without_queenside(&self) -> Self {
        Self {
            king_side: self.king_side,
            queen_side: false,
        }
    }

    pub fn remove_queenside_rights(&mut self) {
        self.queen_side = false;
    }
}

impl Default for CastleRights {
    fn default() -> Self {
        Self {
            king_side: true,
            queen_side: true,
        }
    }
}

#[derive(Debug, Clone)]
pub struct History {
    pub mv: Move,
    pub captured: Option<Piece>,
    pub white_castle_rights: CastleRights,
    pub black_castle_rights: CastleRights,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub zobrist: ZobristHash,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub player: Player,
    pub board: Board,
    pub white_castle_rights: CastleRights,
    pub black_castle_rights: CastleRights,
    pub en_passant_target: Option<Square>,
    pub halfmove_clock: u32,
    pub plies: u32,

    pub zobrist: ZobristHash,
    pub history: Vec<History>,
}

impl Game {
    #[must_use]
    pub fn new() -> Self {
        Self::from_state(
            Board::start(),
            Player::White,
            CastleRights::default(),
            CastleRights::default(),
            None,
            0,
            0,
        )
    }

    #[must_use]
    pub fn from_state(
        board: Board,
        player: Player,
        white_castle_rights: CastleRights,
        black_castle_rights: CastleRights,
        en_passant_target: Option<Square>,
        halfmove_clock: u32,
        plies: u32,
    ) -> Self {
        let mut game = Self {
            board,
            player,
            white_castle_rights,
            black_castle_rights,
            en_passant_target,
            halfmove_clock,
            plies,

            zobrist: ZobristHash::uninit(),
            history: Vec::new(),
        };

        let zobrist = zobrist::hash(&game);
        game.zobrist = zobrist;
        game
    }

    pub fn from_fen(fen: &str) -> Result<Self> {
        fen::parse(fen)
    }

    #[must_use]
    pub fn to_fen(&self) -> String {
        fen::write(self)
    }

    #[must_use]
    pub fn turn(&self) -> u32 {
        self.plies / 2 + 1
    }

    #[must_use]
    pub fn pseudo_legal_moves(&self) -> Vec<Move> {
        generate_moves(self)
    }

    #[must_use]
    pub fn is_stalemate_by_fifty_move_rule(&self) -> bool {
        // TODO: Make sure that the order of checking checkmates vs. draws in search
        // won't cause us to detect a draw when we should have checkmate.
        self.halfmove_clock >= 100
    }

    #[must_use]
    pub fn is_stalemate_by_repetition(&self) -> bool {
        // PERF: We only need to search up to the last irreversible move
        let mut count = 0;

        for seen_state in self.history.iter().rev() {
            if self.zobrist == seen_state.zobrist {
                count += 1;
            }

            // We've seen the current state twice before, so it has occurred three times overall
            // This is a draw by threefold repetition.
            if count == 2 {
                return true;
            }
        }

        false
    }

    fn set_at(&mut self, sq: Square, piece: Piece) {
        self.board.set_at(sq, piece);
        self.zobrist.toggle_piece_on_square(sq, piece);
    }

    fn remove_at(&mut self, sq: Square) {
        let removed_piece = self.board.piece_at(sq).unwrap();
        self.board.remove_at(sq);
        self.zobrist.toggle_piece_on_square(sq, removed_piece);
    }

    pub fn make_move(&mut self, mv: &Move) {
        let from = mv.src;
        let to = mv.dst;

        let moved_piece = self.board.piece_at(from).unwrap();
        let maybe_captured_piece = self.board.piece_at(to);

        let previous_en_passant_target = self.en_passant_target;

        // Capture the irreversible aspects of the position so that they can be restored
        // if we undo this move.
        let history = History {
            mv: *mv,
            captured: maybe_captured_piece,
            white_castle_rights: self.white_castle_rights,
            black_castle_rights: self.black_castle_rights,
            en_passant_target: self.en_passant_target,
            halfmove_clock: self.halfmove_clock,
            zobrist: self.zobrist.clone(),
        };

        self.history.push(history);

        self.remove_at(from);

        if maybe_captured_piece.is_some() {
            self.remove_at(to);
        }

        if let Some(promoted_to) = mv.promotion {
            let promoted_piece = Piece::new(moved_piece.player, promoted_to.piece());
            self.set_at(to, promoted_piece);
        } else {
            self.set_at(to, moved_piece);
        }

        // If we just moved a pawn diagonally, we need to double check whether it was en-passant,
        // in which case we need to remove the captured pawn.
        //
        // PERF: It may be more performant to
        // tell this function that the move was en-passant, but it loses the cleanliness of
        // just telling the board the start and end destination for the piece.
        //
        // PERF: We only need to check mv.is_diagonal() if we moved from the rank where
        // en-passant can happen which is likely a much cheaper check (just bitwise and).
        if let Some(en_passant_target) = self.en_passant_target {
            if moved_piece.kind == PieceKind::Pawn && to == en_passant_target {
                let pawn_attacks = move_tables::pawn_attacks(from, moved_piece.player);

                if pawn_attacks.contains(to) {
                    let opponent_pieces =
                        self.board.player_pieces(moved_piece.player.other()).all();

                    // Definitely en-passant, as we made a capture but there was no piece on that square.
                    if !opponent_pieces.contains(to) {
                        // Get the square that we need to remove the pawn from.
                        let inverse_pawn_move_direction = match moved_piece.player {
                            Player::White => Direction::South,
                            Player::Black => Direction::North,
                        };

                        let capture_square = to.in_direction(&inverse_pawn_move_direction).unwrap();
                        self.remove_at(capture_square);
                    }
                }
            }
        }

        let pawn_move_direction = match self.player {
            Player::White => Direction::North,
            Player::Black => Direction::South,
        };

        let back_rank = match self.player {
            Player::White => squares::RANK_2,
            Player::Black => squares::RANK_7,
        };

        let double_push_rank = match self.player {
            Player::White => squares::RANK_4,
            Player::Black => squares::RANK_5,
        };

        self.en_passant_target = if moved_piece.kind == PieceKind::Pawn
            && back_rank.contains(from)
            && double_push_rank.contains(to)
        {
            let to_square = Squares::from_square(to);
            let en_passant_attacker_squares = to_square.west() | to_square.east();
            let enemy_pawns = self.board.player_pieces(self.player.other()).pawns;
            let en_passant_can_happen = !(en_passant_attacker_squares & enemy_pawns).is_empty();

            if en_passant_can_happen {
                Some(from.in_direction(&pawn_move_direction).unwrap())
            } else {
                None
            }
        } else {
            None
        };

        self.zobrist
            .set_en_passant(previous_en_passant_target, self.en_passant_target);

        // If we just moved a king from its start square, we may have castled.
        //
        // PERF: Here, we figure out if the move was castling. It may be more performant to
        // tell this function that the move was castling, but it loses the cleanliness of
        // just telling the board the start and end destination for the piece.
        //
        // TODO: Collapse the queenside and kingside code paths into one here
        if moved_piece.kind == PieceKind::King && from == squares::king_start(moved_piece.player) {
            let our_rook = Piece::new(moved_piece.player, PieceKind::Rook);

            // We're castling!
            if to == squares::kingside_castle_dest(moved_piece.player) {
                let rook_remove_square = squares::kingside_rook_start(moved_piece.player);
                let rook_add_square = match moved_piece.player {
                    Player::White => F1,
                    Player::Black => F8,
                };

                self.remove_at(rook_remove_square);
                self.set_at(rook_add_square, our_rook);
            } else if to == squares::queenside_castle_dest(moved_piece.player) {
                let rook_remove_square = squares::queenside_rook_start(moved_piece.player);
                let rook_add_square = match moved_piece.player {
                    Player::White => D1,
                    Player::Black => D8,
                };

                self.remove_at(rook_remove_square);
                self.set_at(rook_add_square, our_rook);
            }
        }

        let (castle_rights, other_player_castle_rights) = match self.player {
            Player::White => (&mut self.white_castle_rights, &mut self.black_castle_rights),
            Player::Black => (&mut self.black_castle_rights, &mut self.white_castle_rights),
        };

        if moved_piece.kind == PieceKind::King && from == squares::king_start(self.player) {
            if castle_rights.king_side {
                castle_rights.remove_kingside_rights();
                self.zobrist
                    .toggle_castle_rights(self.player, CastleRightsSide::Kingside);
            }

            if castle_rights.queen_side {
                castle_rights.remove_queenside_rights();
                self.zobrist
                    .toggle_castle_rights(self.player, CastleRightsSide::Queenside);
            }
        } else if moved_piece.kind == PieceKind::Rook {
            if from == squares::kingside_rook_start(self.player) && castle_rights.king_side {
                self.zobrist
                    .toggle_castle_rights(self.player, CastleRightsSide::Kingside);
                castle_rights.remove_kingside_rights();
            } else if from == squares::queenside_rook_start(self.player) && castle_rights.queen_side
            {
                castle_rights.remove_queenside_rights();
                self.zobrist
                    .toggle_castle_rights(self.player, CastleRightsSide::Queenside);
            }
        }

        if maybe_captured_piece.is_some() {
            if to == squares::kingside_rook_start(self.player.other())
                && other_player_castle_rights.king_side
            {
                other_player_castle_rights.remove_kingside_rights();
                self.zobrist
                    .toggle_castle_rights(self.player.other(), CastleRightsSide::Kingside);
            } else if to == squares::queenside_rook_start(self.player.other())
                && other_player_castle_rights.queen_side
            {
                other_player_castle_rights.remove_queenside_rights();
                self.zobrist
                    .toggle_castle_rights(self.player.other(), CastleRightsSide::Queenside);
            }
        }

        let should_reset_halfmove_clock =
            maybe_captured_piece.is_some() || moved_piece.kind == PieceKind::Pawn;

        if should_reset_halfmove_clock {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.plies += 1;

        self.player = self.player.other();
        self.zobrist.toggle_side_to_play();
    }

    pub fn undo_move(&mut self) {
        let history = self.history.pop().unwrap();
        let mv = history.mv;
        let from = mv.src;
        let to = mv.dst;

        let moved_piece = self.board.piece_at(to).unwrap();

        self.player = self.player.other();
        self.zobrist.toggle_side_to_play();

        self.plies -= 1;

        self.halfmove_clock = history.halfmove_clock;

        // If either player lost their castle rights during the move, we restore them
        if !self.white_castle_rights.king_side && history.white_castle_rights.king_side {
            self.zobrist
                .toggle_castle_rights(Player::White, CastleRightsSide::Kingside);
        }
        if !self.white_castle_rights.queen_side && history.white_castle_rights.queen_side {
            self.zobrist
                .toggle_castle_rights(Player::White, CastleRightsSide::Queenside);
        }
        if !self.black_castle_rights.king_side && history.black_castle_rights.king_side {
            self.zobrist
                .toggle_castle_rights(Player::Black, CastleRightsSide::Kingside);
        }
        if !self.black_castle_rights.queen_side && history.black_castle_rights.queen_side {
            self.zobrist
                .toggle_castle_rights(Player::Black, CastleRightsSide::Queenside);
        }

        self.white_castle_rights = history.white_castle_rights;
        self.black_castle_rights = history.black_castle_rights;

        // Undo castling, if we castled
        if moved_piece.kind == PieceKind::King && from == squares::king_start(moved_piece.player) {
            let our_rook = Piece::new(moved_piece.player, PieceKind::Rook);

            if to == squares::kingside_castle_dest(moved_piece.player) {
                let rook_removed_square = squares::kingside_rook_start(moved_piece.player);
                let rook_added_square = match moved_piece.player {
                    Player::White => F1,
                    Player::Black => F8,
                };

                self.remove_at(rook_added_square);
                self.set_at(rook_removed_square, our_rook);
            } else if to == squares::queenside_castle_dest(moved_piece.player) {
                let rook_removed_square = squares::queenside_rook_start(moved_piece.player);
                let rook_added_square = match moved_piece.player {
                    Player::White => D1,
                    Player::Black => D8,
                };

                self.remove_at(rook_added_square);
                self.set_at(rook_removed_square, our_rook);
            }
        }

        // Replace the pawn taken by en-passant capture
        if let Some(en_passant_target) = history.en_passant_target {
            if moved_piece.kind == PieceKind::Pawn && to == en_passant_target {
                let pawn_attacks = move_tables::pawn_attacks(from, moved_piece.player);

                if pawn_attacks.contains(to) {
                    // Get the square that we need to remove the pawn from.
                    let inverse_pawn_move_direction = match moved_piece.player {
                        Player::White => Direction::South,
                        Player::Black => Direction::North,
                    };

                    let capture_square = to.in_direction(&inverse_pawn_move_direction).unwrap();
                    self.set_at(
                        capture_square,
                        Piece::new(moved_piece.player.other(), PieceKind::Pawn),
                    );
                }
            }
        }

        let en_passant_target_before_undo = self.en_passant_target;
        self.en_passant_target = history.en_passant_target;

        self.zobrist
            .set_en_passant(en_passant_target_before_undo, history.en_passant_target);

        let moved_piece = self.board.piece_at(to).unwrap();
        self.remove_at(to);

        if let Some(captured_piece) = history.captured {
            self.set_at(to, captured_piece);
        }

        if mv.promotion.is_some() {
            self.set_at(from, Piece::new(self.player, PieceKind::Pawn));
        } else {
            self.set_at(from, moved_piece);
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Self::new()
    }
}
