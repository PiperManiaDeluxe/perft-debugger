use std::collections::HashMap;

use cozy_chess::Move;

use crate::result_types::{FailedMove, PerMovePerftResult, SingleMovePerftResult};

/// Compares [`PerMovePerftResult`]s and returns all conflicting moves in `reported`.
///
/// # Arguments
///
/// * `expected`: Ground truth per-move perft results.
/// * `reported`: Per-move perft results reported from the engine.
pub fn compare_perft_results(
    expected: PerMovePerftResult,
    reported: PerMovePerftResult,
) -> Vec<FailedMove> {
    let mut failing_moves = vec![];

    // Converts a move result struct into a key-pair tuple
    let move_result_to_tuple = |result: &SingleMovePerftResult| (result.mv, result.nodes);

    let expected_key_pairs = expected.moves.iter().map(move_result_to_tuple);
    let mut expected_move_table: HashMap<Move, usize> = HashMap::from_iter(expected_key_pairs);

    for reported_result in reported.moves {
        if let Some(failed_move) = compare_single_result(&mut expected_move_table, reported_result)
        {
            failing_moves.push(failed_move);
        }
    }

    // Any moves remaining in expected_move_table is a missing move
    for (mv, nodes) in expected_move_table {
        failing_moves.push(FailedMove {
            mv,
            expected_nodes: nodes,
            reported_nodes: 0,
        });
    }

    failing_moves
}

/// Compares a [`SingleMovePerftResult`] to an expected hashmap of move results, then returns a failed move if found.
fn compare_single_result(
    expected_move_table: &mut HashMap<Move, usize>,
    reported_result: SingleMovePerftResult,
) -> Option<FailedMove> {
    match expected_move_table.remove(&reported_result.mv) {
        Some(expected_nodes) if expected_nodes == reported_result.nodes => {
            // Legal move!
            None
        }
        Some(expected_nodes) => {
            // Mismatched move
            Some(FailedMove {
                mv: reported_result.mv,
                expected_nodes,
                reported_nodes: reported_result.nodes,
            })
        }
        None => {
            // Illegal move
            Some(FailedMove {
                mv: reported_result.mv,
                expected_nodes: 0,
                reported_nodes: reported_result.nodes,
            })
        }
    }
}
