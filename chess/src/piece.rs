use crate::player::Player;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum PromotionPieceKind {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl PromotionPieceKind {
    pub const ALL: &'static [Self; 4] = &[Self::Knight, Self::Bishop, Self::Rook, Self::Queen];

    #[must_use]
    pub fn piece(&self) -> PieceKind {
        match self {
            Self::Knight => PieceKind::Knight,
            Self::Bishop => PieceKind::Bishop,
            Self::Rook => PieceKind::Rook,
            Self::Queen => PieceKind::Queen,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub kind: PieceKind,
    pub player: Player,
}

impl Piece {
    pub const WHITE_PAWN: Self = Self::white(PieceKind::Pawn);
    pub const WHITE_KNIGHT: Self = Self::white(PieceKind::Knight);
    pub const WHITE_BISHOP: Self = Self::white(PieceKind::Bishop);
    pub const WHITE_ROOK: Self = Self::white(PieceKind::Rook);
    pub const WHITE_QUEEN: Self = Self::white(PieceKind::Queen);
    pub const WHITE_KING: Self = Self::white(PieceKind::King);

    pub const BLACK_PAWN: Self = Self::black(PieceKind::Pawn);
    pub const BLACK_KNIGHT: Self = Self::black(PieceKind::Knight);
    pub const BLACK_BISHOP: Self = Self::black(PieceKind::Bishop);
    pub const BLACK_ROOK: Self = Self::black(PieceKind::Rook);
    pub const BLACK_QUEEN: Self = Self::black(PieceKind::Queen);
    pub const BLACK_KING: Self = Self::black(PieceKind::King);

    #[must_use]
    pub const fn new(player: Player, kind: PieceKind) -> Self {
        Self { kind, player }
    }

    #[must_use]
    pub const fn white(kind: PieceKind) -> Self {
        Self::new(Player::White, kind)
    }

    #[must_use]
    pub const fn black(kind: PieceKind) -> Self {
        Self::new(Player::Black, kind)
    }
}
