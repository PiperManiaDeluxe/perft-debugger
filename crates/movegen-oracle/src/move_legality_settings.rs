#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MoveLegalitySettings {
    pub pieces: PerPieceAvailability,
    pub rules: PerPieceRules,
    pub may_capture_opponent_king: bool,
    pub check: CheckHandling,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerPieceAvailability {
    pub kings: SinglePieceAvailability,
    pub pawns: SinglePieceAvailability,
    pub knights: SinglePieceAvailability,
    pub bishops: SinglePieceAvailability,
    pub rooks: SinglePieceAvailability,
    pub queens: SinglePieceAvailability,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SinglePieceAvailability {
    NotPresent,
    PresentCantMove,
    Present,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PerPieceRules {
    pub kings: KingMoveRules,
    pub pawns: PawnMoveRules,
    pub knights: KnightMoveRules,
    pub bishops: SlidingMoveRules,
    pub rooks: SlidingMoveRules,
    pub queens: SlidingMoveRules,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KingMoveRules {
    pub castling: bool,
    pub captures_only_opponent_pieces: bool,
    pub require_castling_squares_unattacked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PawnMoveRules {
    pub forward_moves_require_empty_squares: bool,
    pub diagonal_captures_require_occupied_squares: bool,
    pub may_double_step_from_starting_rank: bool,
    pub promotion: PromotionMode,
    pub en_passant: bool,
    pub captures_only_opponent_pieces: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KnightMoveRules {
    pub captures_only_opponent_pieces: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SlidingMoveRules {
    pub rays_stop_at_occupied_squares: bool,
    pub captures_only_opponent_pieces: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PromotionMode {
    Disabled,
    QueenOnly,
    AnyPromotionPiece,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CheckHandling {
    Ignore,
    PreventMovesLeavingKingInCheck,
}
