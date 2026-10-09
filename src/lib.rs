#![allow(dead_code)]

mod bin_helpers;
mod engine_communication;
mod fail_search;
mod initial_test;
mod lib_ext;
mod perft;
mod types;

pub use fail_search::FailingLeafReport;
pub use lib_ext::debug_result::DebugResult;
pub use perft::perft_provider::*;
pub use types::per_move_perft_result::*;
pub use types::uci_move::UCIMove;

use crate::fail_search::find_failing_leaf;
use crate::initial_test::initial_perft_test;
use crate::perft::cozy_perft::CozyChessPerftTarget;

pub mod helpers {
    pub use crate::types::uci_move::{square_from_name, square_to_name};
}

/// TODO: Docs
pub fn debug_perft_target(
    perft_target: &mut impl PerftProvider,
    test_position_fen: &str,
    max_test_depth: usize,
    ignored_lines: &[Vec<UCIMove>],
) -> Result<DebugResult, PerftError> {
    let mut true_perft = CozyChessPerftTarget;

    let mut ignored_initial_moves = vec![];
    for line in ignored_lines {
        for mv in line {
            ignored_initial_moves.push(*mv);
            break;
        }
    }

    let initial_result = initial_perft_test(
        perft_target,
        &mut true_perft,
        test_position_fen,
        max_test_depth,
        false,
        &ignored_initial_moves,
    )?;

    match initial_result {
        Some(initial_fail) => {
            let failing_leaf = find_failing_leaf(
                perft_target,
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
