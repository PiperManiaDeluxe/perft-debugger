use crate::types::uci_move::UCIMove;

/// A move that failed a perft test.
#[derive(Debug, Clone, Copy)]
pub struct FailedMove {
    /// The move that failed.
    pub mv: UCIMove,
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
                println!("Engine generated an illegal move: {}", self.mv.to_string());
            }
            MoveFailReason::Missing => {
                println!("Engine missed a legal move: {}", self.mv.to_string());
            }
            MoveFailReason::MisMatched => {
                println!("Move has a perft count mismatch: {}", self.mv.to_string());
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
