use anyhow::Result;

use crate::{
    perft::{compare_results::compare_perft_results, perft_provider::PerftProvider},
    types::{
        failed_move::{FailedMove, MoveFailReason},
        uci_move::UCIMove,
    },
};

/// Details about a failing move found as a perft leaf.
///
/// TODO: Move to types
#[derive(Debug, Clone)]
pub struct FailingLeafReport {
    /// Moves played from the starting position to reach the offending move.
    pub path: Vec<UCIMove>,
    /// The move that failed.
    pub offending_move: FailedMove,
}

/// Traces a perft failure from the root position to a failing leaf node.
///
/// The search follows failed moves deeper into the tree
/// until it finds a missing or illegal move at a leaf.
///
/// # Arguments
///
/// * `target_perft`: The [`PerftTarget`] we want to diagnose.
/// * `true_perft`: The [`PerftTarget`] to use for ground truth results.
/// * `start_position`: Starting position (in FEN) for the engine perft query.
/// * `depth`: Remaining perft depth to explore.
/// * `path`: Moves played from `start_position` to reach the current testing position.
/// * `failing_moves`: Per-move perft discrepancies found at the current testing position.
/// * `reported_lines`: Failure leaf move paths that have already been reported.
///
/// # Returns
///
/// * `Ok(Some(report))` if an unreported failing move path is found.
/// * `Ok(None)` if no unreported failure is found within the search depth.
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
            .unwrap();

        let reported = target_perft
            .perft(start_position_fen, &leaf_path, depth - 1)
            .unwrap();

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
