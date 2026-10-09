use std::time::Instant;

use anyhow::{Context, Result};

use crate::{
    engine_communication::UCIEngine,
    get_debug_perft_results,
    helpers::parse_board,
    perft::{compare_results::compare_perft_results, true_perft::get_true_perft_results},
    result_types::FailedMove,
};

/// Iteratively runs deeper perft tests until `max_depth` is reached, or a failing perft depth is hit.
///
/// # Arguments
///
/// * `position_fen`: Position (in FEN) to start perft tests from.
/// * `max_depth`: Max perft depth to test.
/// * `engine`: [`UCIEngine`] process handle for the engine we want to debug.
///
/// # Returns
///
/// * `Ok(Some(Report))` If a depth test fails. The report contains the first
/// failing depth, and the moves whose perft results differ from ground truth.
/// * `Ok(None)` If all tested depths passed.
pub fn initial_perft_test(
    position_fen: &str,
    max_depth: usize,
    engine: &mut UCIEngine,
) -> Result<Option<InitialTestFailReport>> {
    let root_board = parse_board(position_fen)?;

    for depth in 1..=max_depth {
        let start = Instant::now();

        let expected = get_true_perft_results(&root_board, depth);
        let reported = get_debug_perft_results(engine, position_fen, &vec![], depth)
            .context("Failed to run engine perft")?;

        println!(
            "Perft {} took {:.1} seconds. Nodes: {}",
            depth,
            start.elapsed().as_secs_f32(),
            expected.total_nodes
        );

        let failing_moves = compare_perft_results(expected, reported);

        if !failing_moves.is_empty() {
            return Ok(Some(InitialTestFailReport {
                depth,
                failing_moves,
            }));
        }
    }

    Ok(None)
}

/// Details the first perft test failure found by [`initial_perft_test`].
pub struct InitialTestFailReport {
    /// The first depth at which the engine's perft results differ from the expected results.
    pub depth: usize,
    /// Moves whose perft results differ from the expected results.
    pub failing_moves: Vec<FailedMove>,
}
