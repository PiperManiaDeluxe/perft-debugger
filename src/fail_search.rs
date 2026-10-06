use anyhow::Result;
use cozy_chess::{Board, Move};

use crate::{
    engine_communication::UCIEngine,
    get_debug_perft_results,
    perft::{compare_results::compare_perft_results, true_perft::get_true_perft_results},
    results::{FailedMove, MoveFailReason},
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
/// * `engine`: [`UCIEngine`] process handle to diagnose.
/// * `start_position`: Starting position (in FEN) for the engine perft query.
/// * `board`: Current position in the search.
/// * `depth`: Remaining perft depth to explore.
/// * `path`: Moves played from `start_position` to reach `board`.
/// * `failing_moves`: Per-move perft discrepancies found at `board`.
/// * `reported_lines`: Failure leaf move paths that have already been reported.
///
/// # Returns
///
/// * `Ok(Some(report))` if an unreported failing move path is found.
/// * `Ok(None)` if no unreported failure is found within the search depth.
pub fn find_failing_leaf(
    engine: &mut UCIEngine,
    start_position: &str,
    board: &Board,
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

        let mut leaf_board = board.clone();
        leaf_board.play_unchecked(failed_move.mv);
        let mut leaf_path = path.clone();
        leaf_path.push(failed_move.mv);

        let expected = get_true_perft_results(&leaf_board, depth - 1);

        let mut moves_string = String::new();
        for mv in &leaf_path {
            moves_string.push_str(&mv.to_string());
            moves_string.push(' ');
        }

        let reported = get_debug_perft_results(engine, start_position, &leaf_path, depth - 1)?;

        let leaf_failing_moves = compare_perft_results(expected, reported);

        let result = find_failing_leaf(
            engine,
            start_position,
            &leaf_board,
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
