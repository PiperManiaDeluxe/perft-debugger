//! Runs perft tests at successively increasing depths to find the first
//! discrepancy between a target provider and a reference provider.

use std::time::Instant;

use anyhow::{Result, anyhow};

use crate::{
    perft::{compare_results::compare_perft_results, perft_provider::PerftProvider},
    types::{failed_move::FailedMove, uci_move::UCIMove},
};

/// Details the first perft test failure found by [`initial_perft_test`].
pub struct InitialTestFailReport {
    /// The first depth at which the providers' perft results differed.
    pub depth: usize,
    /// Moves with missing, illegal, or mismatched perft results at that depth.
    pub failing_moves: Vec<FailedMove>,
}

/// Tests perft results at successively increasing depths,
/// stopping at the first discrepancy or after testing `max_depth`.
///
/// # Parameters
///
/// * `target_perft`: Provider whose results are being checked.
/// * `true_perft`: Provider supplying the expected reference results.
/// * `position_fen`: FEN string for the starting position.
/// * `max_depth`: Greatest depth to test. If this is `0`, no tests are run.
/// * `print`: Whether to print timing and expected node-count information for
///   each tested depth.
/// * `ignored_initial_moves`: Moves to exclude from the comparison at depth 1.
///   This list is not applied at greater depths.
///
/// # Returns
///
/// * `Ok(Some(report))` if a discrepancy is found. The report contains the first
///   failing depth and the moves that differed at that depth.
/// * `Ok(None)` if all tested depths pass, or if `max_depth` is `0`.
///
/// # Errors
///
/// Returns an error if either provider fails to produce perft results. The
/// error includes context identifying which provider failed.
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

        let expected = true_perft //
            .perft(position_fen, &[], depth)
            .map_err(|err| {
                anyhow!("Error while getting perft results from true_perft provider: {err}")
            })?;
        let reported = target_perft
            .perft(position_fen, &[], depth)
            .map_err(|err| {
                anyhow!("Error while getting perft results from target_perft provider: {err}")
            })?;

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
