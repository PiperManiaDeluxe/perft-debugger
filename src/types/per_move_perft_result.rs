use cozy_chess::Move;

/// Per-move perft results for a root position.
#[derive(Debug, Default)]
pub struct PerMovePerftResult {
    /// [`SingleMovePerftResult`]s for every move on the root position.
    pub moves: Vec<SingleMovePerftResult>,
    /// Total number of nodes reached.
    pub total_nodes: usize,
}

/// Perft results for a single root move.
#[derive(Debug)]
pub struct SingleMovePerftResult {
    /// Root move for this result.
    pub mv: Move,
    /// Total nodes reached for this move.
    pub nodes: usize,
}
