use anyhow::Context;
use cozy_chess::Board;

use crate::{
    helpers::{parse_board, parse_move, remap_castling_move},
    perft::perft_target::{self, PerftTarget},
    types::per_move_perft_result::{PerMovePerftResult, SingleMovePerftResult},
};

/// [`PerftTarget`] for the ground truth move generator [`cozy_chess`].
pub struct CozyChessPerftTarget;

impl PerftTarget for CozyChessPerftTarget {
    fn perft(
        &mut self,
        position_fen: &str,
        position_moves: &[&str],
        depth: usize,
    ) -> Result<PerMovePerftResult, perft_target::Error> {
        let mut board = parse_board(position_fen)
            .context("Failed to parse `position_fen` as cozy_chess::Board")?;

        // Play the moves to this point
        for mv in position_moves {
            let mv = parse_move(*mv)
                .context("Failed to parse a move in `position_moves` as cozy_chess::Move")?;

            // TODO: We might want the checked variant
            board.play_unchecked(mv);
        }

        let mut results = PerMovePerftResult::default();

        board.generate_moves(|legal_moves| {
            for child_move in legal_moves {
                let mut child_board = board.clone();

                // Cozy chess moves are not UCI compliant.
                let reported_move = remap_castling_move(&board, &child_move);

                child_board.play_unchecked(child_move);

                let nodes = if depth > 1 {
                    do_single_perft(&child_board, depth - 1)
                } else {
                    1
                };

                results.total_nodes += nodes;
                results.moves.push(SingleMovePerftResult {
                    mv: reported_move,
                    nodes,
                });
            }

            // Generate every legal move
            false
        });

        Ok(results)
    }
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
