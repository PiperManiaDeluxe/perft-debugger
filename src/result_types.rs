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

/// A move that failed a perft test.
#[derive(Debug, Clone, Copy)]
pub struct FailedMove {
    /// The move that failed.
    pub mv: Move,
    /// Node count we expected to find.
    pub expected_nodes: usize,
    /// Node count we actual found.
    pub reported_nodes: usize,
}

impl FailedMove {
    /// The reason this move failed perft.
    pub fn get_fail_reason(&self) -> MoveFailReason {
        if self.expected_nodes == 0 {
            MoveFailReason::Illegal
        } else if self.reported_nodes == 0 {
            MoveFailReason::Missing
        } else {
            MoveFailReason::MisMatched
        }
    }

    /// Prints out the reason this move failed perft.
    pub fn report_fail_reason(&self) {
        match self.get_fail_reason() {
            MoveFailReason::Illegal => {
                println!("Engine generated an illegal move: {}", self.mv);
            }
            MoveFailReason::Missing => {
                println!("Engine missed a legal move: {}", self.mv);
            }
            MoveFailReason::MisMatched => {
                println!("Move has a perft count mismatch: {}", self.mv);
            }
        }
    }
}

/// Reasons a move can fail perft
#[derive(Debug, PartialEq, Eq)]
pub enum MoveFailReason {
    MisMatched,
    Missing,
    Illegal,
}
