//! Types describing moves that differ between expected
//! and reported perft results.
//!
//! Includes the move's expected and reported node counts, its failure reason,
//! and reports identifying a failing move at the end of a move path.

use std::fmt;

use crate::types::uci_move::UCIMove;

/// A move that failed a perft test.
#[derive(Debug, Clone, Copy)]
pub struct FailedMove {
    /// The move that failed.
    pub uci_move: UCIMove,
    /// Node count we expected to find.
    pub expected_nodes: usize,
    /// Node count reported by the provider.
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

/// Describes how a move's reported perft result differs from the expected result.
#[derive(Debug, PartialEq, Eq)]
pub enum MoveFailReason {
    /// Both providers reported a nonzero node count, but the counts differ.
    MisMatched,
    /// The reference provider reported this legal move, but the target provider
    /// did not generate it.
    Missing,
    /// The target provider generated a move that the reference provider did not
    /// report as legal.
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

/// Details about a failing move found as a perft leaf.
#[derive(Debug, Clone)]
pub struct FailingLeafReport {
    /// Moves played from the starting position to reach the offending move.
    pub path: Vec<UCIMove>,
    /// The move that failed.
    pub offending_move: FailedMove,
}
