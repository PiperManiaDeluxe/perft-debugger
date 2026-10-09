use std::time::Instant;

use anyhow::Result;

use crate::{
    perft::{compare_results::compare_perft_results, perft_provider::PerftProvider},
    types::{failed_move::FailedMove, uci_move::UCIMove},
};

/// Details the first perft test failure found by [`initial_perft_test`].
pub struct InitialTestFailReport {
    /// The first depth at which the engine's perft results differ from the expected results.
    pub depth: usize,
    /// Moves whose perft results differ from the expected results.
    pub failing_moves: Vec<FailedMove>,
}

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
    target_perft: &mut impl PerftProvider,
    true_perft: &mut impl PerftProvider,
    position_fen: &str,
    max_depth: usize,
    print: bool,
    ignored_initial_moves: &[UCIMove],
) -> Result<Option<InitialTestFailReport>> {
    for depth in 1..=max_depth {
        let start = Instant::now();

        let expected = true_perft.perft(position_fen, &[], depth).unwrap();
        let reported = target_perft.perft(position_fen, &[], depth).unwrap();

        if print {
            println!(
                "Perft {} took {:.1} seconds. Nodes: {}",
                depth,
                start.elapsed().as_secs_f32(),
                expected.total_nodes
            );
        }

        let failing_moves = compare_perft_results(
            expected,
            reported,
            if depth == 1 {
                ignored_initial_moves
            } else {
                &[]
            },
        );

        if !failing_moves.is_empty() {
            return Ok(Some(InitialTestFailReport {
                depth,
                failing_moves,
            }));
        }
    }

    Ok(None)
}
