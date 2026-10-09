use std::fmt;

use crate::types::uci_move::UCIMove;

/// A move that failed a perft test.
#[derive(Debug, Clone, Copy)]
pub struct FailedMove {
    /// The move that failed.
    pub uci_move: UCIMove,
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
                println!(
                    "Engine generated an illegal move: {}",
                    self.uci_move.to_string()
                );
            }
            MoveFailReason::Missing => {
                println!(
                    "Engine did not generate an expected legal move: {}",
                    self.uci_move.to_string()
                );
            }
            MoveFailReason::MisMatched => {
                println!(
                    "Move has a perft count mismatch: {}",
                    self.uci_move.to_string()
                );
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

impl fmt::Display for MoveFailReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match *self {
            Self::MisMatched => "Non-terminal move has a perft node-count mismatch",
            Self::Missing => "Expected legal move not generated",
            Self::Illegal => "Illegal move was generated",
        };
        f.write_str(string)
    }
}
