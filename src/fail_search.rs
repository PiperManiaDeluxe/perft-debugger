//! Searches a perft discrepancy down the move tree for missing or illegal
//! moves at the leaves.
//!
//! The search follows non terminal node count mismatches deeper into the tree,
//! until it finds an unreported missing or illegal move at a leaf, or it
//! exhausts the search depth.

use anyhow::{Result, anyhow};

use crate::{
    perft::{compare_results::compare_perft_results, perft_provider::PerftProvider},
    types::{
        failed_move::{FailedMove, FailingLeafReport, MoveFailReason},
        uci_move::UCIMove,
    },
};

/// Traces a perft failure from the root position to a failing leaf node.
///
/// The search follows failed moves deeper into the tree
/// until it finds a missing or illegal move at a leaf.
///
/// # Arguments
///
/// * `target_perft`: The [`PerftProvider`] we want to diagnose.
/// * `true_perft`: The [`PerftProvider`] to use for ground truth results.
/// * `start_position_fen`: FEN string for the position from which the search
///                         path is replayed.
/// * `depth`: Remaining perft depth to explore.
/// * `path`: Moves played from `start_position` to reach the current testing position.
/// * `failing_moves`: Per-move perft discrepancies found at the current testing position.
/// * `reported_lines`: Failure leaf move paths that have already been reported.
///
/// # Returns
///
/// * `Ok(Some(report))` if an unreported failing move path is found.
/// * `Ok(None)` if no unreported failure is found within the search depth.
///
/// # Errors
///
/// Returns an error if either provider fails to produce perft results while
/// the search explores a deeper position. The error includes context
/// identifying whether `true_perft` or `target_perft` failed.
pub fn find_failing_leaf(
    target_perft: &mut impl PerftProvider,
    true_perft: &mut impl PerftProvider,
    start_position_fen: &str,
    depth: usize,
    path: Vec<UCIMove>,
    failing_moves: &[FailedMove],
    reported_lines: &[Vec<UCIMove>],
) -> Result<Option<FailingLeafReport>> {
    if failing_moves.is_empty() || depth == 0 {
        return Ok(None);
    }

    for failed_move in failing_moves {
        if failed_move.get_fail_reason() == MoveFailReason::MisMatched {
            continue;
        }

        let mut full_line = path.clone();
        full_line.push(failed_move.uci_move);

        let already_reported = reported_lines.iter().any(|line| line == &full_line);

        if !already_reported {
            return Ok(Some(FailingLeafReport {
                path,
                offending_move: *failed_move,
            }));
        }
    }

    if depth == 1 {
        return Ok(None);
    }

    for failed_move in failing_moves {
        if failed_move.get_fail_reason() == MoveFailReason::Illegal
            || failed_move.get_fail_reason() == MoveFailReason::Missing
        {
            continue;
        }

        let mut leaf_path = path.clone();
        leaf_path.push(failed_move.uci_move);

        let expected = true_perft
            .perft(start_position_fen, &leaf_path, depth - 1)
            .map_err(|err| {
                anyhow!("Error while getting perft results from true_perft provider: {err}")
            })?;
        let reported = target_perft
            .perft(start_position_fen, &leaf_path, depth - 1)
            .map_err(|err| {
                anyhow!("Error while getting perft results from target_perft provider: {err}")
            })?;

        let leaf_failing_moves = compare_perft_results(expected, reported, &[]);

        let result = find_failing_leaf(
            target_perft,
            true_perft,
            start_position_fen,
            depth - 1,
            leaf_path,
            &leaf_failing_moves,
            reported_lines,
        )?;

        if result.is_some() {
            return Ok(result);
        }
    }

    Ok(None)
}
