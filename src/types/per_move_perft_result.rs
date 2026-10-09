use crate::types::uci_move::UCIMove;

/// Per-move perft results for a given root position.
#[derive(Debug, Default)]
pub struct PerMovePerftResult {
    /// [`SingleMovePerftResult`]s for every move playable on the root position.
    pub moves: Vec<SingleMovePerftResult>,
    /// Total number of nodes reached.
    pub total_nodes: usize,
}

/// Perft results for a single root move in a given position.
#[derive(Debug)]
pub struct SingleMovePerftResult {
    /// Move for this result.
    pub uci_move: UCIMove,
    /// Total nodes reached after this move.
    pub nodes: usize,
}
