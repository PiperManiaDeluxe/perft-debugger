use cozy_chess::Board;

use crate::{
    helpers::remap_castling_move,
    results::{PerMovePerftResult, SingleMovePerftResult},
};

/// Computes ground truth per-move perft results for every legal move on the given board.
///
/// # Arguments
///
/// * `board`: [`cozy_chess::Board`] position to start the perft search from.
/// * `depth`: Total depth to explore.
pub fn get_true_perft_results(board: &Board, depth: usize) -> PerMovePerftResult {
    let mut results = PerMovePerftResult::default();

    board.generate_moves(|legal_moves| {
        for leaf_move in legal_moves {
            let mut leaf_board = board.clone();

            // Cozy chess castling moves are not UCI compliant
            let reported_move = remap_castling_move(board, &leaf_move);

            leaf_board.play_unchecked(leaf_move);

            let nodes = if depth > 1 {
                do_single_perft(&leaf_board, depth - 1)
            } else {
                1
            };
            results.total_nodes += nodes;

            results.moves.push(SingleMovePerftResult {
                mv: reported_move,
                nodes,
            });
        }

        false
    });

    results
}

/// Computes ground truth perft results for a given board.
///
/// # Arguments
///
/// * `board`: [`cozy_chess::Board`] position to search from.
/// * `depth`: Remaining depth to explore.
fn do_single_perft(board: &Board, depth: usize) -> usize {
    if depth == 0 {
        return 1;
    }

    let mut nodes = 0;
    board.generate_moves(|legal_moves| {
        for leaf_move in legal_moves {
            let mut leaf_board = board.clone();
            leaf_board.play_unchecked(leaf_move);

            nodes += do_single_perft(&leaf_board, depth - 1);
        }
        // Continue until all moves are processed
        false
    });

    nodes
}
