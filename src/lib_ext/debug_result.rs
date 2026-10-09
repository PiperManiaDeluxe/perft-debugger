//! Provides the [`DebugResult`] enum.

use std::fmt;

use crate::FailingLeafReport;

/// Outcome of checking a perft provider against reference results.
#[derive(Clone)]
pub enum DebugResult {
    /// A perft test differed from the reference results.
    ///
    /// This variant includes the failing depth, and if found,
    /// details identifying a missing or illegal move.
    Failed {
        /// The first & lowest tested depth at which the perft results differed.
        ///
        /// This is also the depth a report in `failing_leaf` will be at.
        fail_depth: usize,
        /// A report identifying a missing or illegal move, if the search found one.
        ///
        /// This may be [`None`] when the failure could not
        /// be traced to a missing or illegal move.
        failing_leaf: Option<FailingLeafReport>,
        /// The initial position of the failed perft test.
        fail_start_position_fen: String,
    },
    /// All tested perft depths passed, subject to any configured ignored lines.
    Successful,
}

impl fmt::Display for DebugResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let DebugResult::Failed {
            fail_depth,
            failing_leaf,
            fail_start_position_fen,
        } = self
        else {
            return writeln!(f, "Perft did not fail!");
        };

        writeln!(f, "Perft failed at depth {fail_depth}!\n")?;

        match failing_leaf {
            Some(failing_leaf_report) => {
                writeln!(
                    f,
                    "Failing move found:\n  {}",
                    failing_leaf_report.offending_move.uci_move
                )?;
                writeln!(
                    f,
                    "Fail reason:\n  {}\n",
                    failing_leaf_report.offending_move.get_fail_reason()
                )?;

                write!(
                    f,
                    "Fail start condition:\n  position fen {fail_start_position_fen}"
                )?;

                if !failing_leaf_report.path.is_empty() {
                    write!(f, " moves ")?;
                }

                for uci_move in &failing_leaf_report.path {
                    write!(f, "{} ", uci_move)?;
                }
                writeln!(f)?;
            }
            None => {
                writeln!(f, "No failing leaf move was found.")?;
            }
        }

        Ok(())
    }
}
