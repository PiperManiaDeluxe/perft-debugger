//! Functions for debugging perft providers

use crate::{
    DebugResult, PerftError, PerftProvider, UCIMove, fail_search::find_failing_leaf,
    initial_test::initial_perft_test, perft::cozy_perft::CozyChessPerftTarget,
};

/// debugs a perft provider's ability to generate legal chess moves by comparing
/// its results with reference results and investigating the first discrepancy.
///
/// Performs perft tests at successively increasing depths, starting at 1 and
/// continuing through `max_test_depth`, using the position described by
/// `test_position_fen`. If a discrepancy is found, the result includes the
/// first failing perft depth and, when found, a report identifying a missing
/// or illegal move. Otherwise the result is [`DebugResult::Successful`].
///
/// At depth 1, the first move of each line in `ignored_lines` is excluded
/// from the initial comparison, The full lines are also used to avoid
/// returning previously reported failing paths during the leaf search.
///
/// # Arguments
///
/// * `perft_provider`: Provider whose results are being checked.
/// * `test_position_fen`: FEN string for the position to test.
/// * `max_test_depth`: Greatest perft depth to test. A value of `0` runs no tests.
/// * `ignored_lines`: Move paths to ignore, as described above.
///
/// # Errors
///
/// Returns an error if the diagnostic process or a perft provider returns an error.
pub fn debug_perft_provider(
    perft_provider: &mut impl PerftProvider,
    test_position_fen: &str,
    max_test_depth: usize,
    ignored_lines: &[Vec<UCIMove>],
) -> Result<DebugResult, PerftError> {
    let mut true_perft = CozyChessPerftTarget;

    let mut ignored_initial_moves = vec![];
    for line in ignored_lines {
        for uci_move in line {
            ignored_initial_moves.push(*uci_move);
            break;
        }
    }

    let initial_result = initial_perft_test(
        perft_provider,
        &mut true_perft,
        test_position_fen,
        max_test_depth,
        false,
        &ignored_initial_moves,
    )?;

    match initial_result {
        Some(initial_fail) => {
            let failing_leaf = find_failing_leaf(
                perft_provider,
                &mut true_perft,
                test_position_fen,
                initial_fail.depth,
                vec![],
                &initial_fail.failing_moves,
                ignored_lines,
            )?;

            Ok(DebugResult::Failed {
                fail_depth: initial_fail.depth,
                failing_leaf,
                fail_start_position_fen: test_position_fen.into(),
            })
        }
        None => Ok(DebugResult::Successful),
    }
}
