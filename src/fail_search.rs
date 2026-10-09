use anyhow::Result;
use cozy_chess::{Board, Move};

use crate::{
    helpers::uci_from_cozy,
    perft::{compare_results::compare_perft_results, perft_target::PerftTarget},
    types::failed_move::{FailedMove, MoveFailReason},
};

/// A failing move that was found while searching for a perft failure.
pub struct FailingLeafReport {
    /// Moves played from the starting position to reach the offending move.
    pub path: Vec<Move>,
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
    target_perft: &mut impl PerftTarget,
    true_perft: &mut impl PerftTarget,
    start_position_fen: &str,
    start_position_board: &Board,
    depth: usize,
    path: Vec<Move>,
    failing_moves: &[FailedMove],
    reported_lines: &[Vec<Move>],
) -> Result<Option<FailingLeafReport>> {
    if failing_moves.is_empty() || depth == 0 {
        return Ok(None);
    }

    for failed_move in failing_moves {
        if failed_move.get_fail_reason() == MoveFailReason::MisMatched {
            continue;
        }

        let mut full_line = path.clone();
        full_line.push(failed_move.mv);

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
        leaf_path.push(failed_move.mv);

        let mut move_path_uci = vec![];
        for mv in &leaf_path {
            move_path_uci.push(uci_from_cozy(mv, &start_position_board));
        }
        let move_path_uci_refs: Vec<&str> = move_path_uci.iter().map(String::as_str).collect();

        let expected = true_perft
            .perft(start_position_fen, &move_path_uci_refs, depth - 1)
            .unwrap();

        let reported = target_perft
            .perft(start_position_fen, &move_path_uci_refs, depth - 1)
            .unwrap();

        let leaf_failing_moves = compare_perft_results(expected, reported);

        let result = find_failing_leaf(
            target_perft,
            true_perft,
            start_position_fen,
            start_position_board,
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
