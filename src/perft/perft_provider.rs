//! Items related to making a move generator accessible to `perft-debugger`.

use crate::types::{per_move_perft_result::PerMovePerftResult, uci_move::UCIMove};

/// TODO: Docs
pub type PerftError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// TODO: Docs
pub trait PerftProvider {
    /// Runs and returns per-move perft results for a [`PerftTarget`].
    ///
    /// Note: This function splits `position_fen` and `position_uci_moves`
    /// to reveal bugs in an engine's ability to make moves.
    ///
    /// # Arguments
    ///
    /// * `position_fen`: Position (in FEN) to initialize the board state in.
    /// * `position_uci_moves`: Moves to make before starting the perft test.
    /// * `depth`: Total depth to explore.
    fn perft(
        &mut self,
        position_fen: &str,
        position_uci_moves: &[UCIMove],
        depth: usize,
    ) -> Result<PerMovePerftResult, PerftError>;
}

/// TODO: Docs?
impl<F> PerftProvider for F
where
    F: FnMut(&str, &[UCIMove], usize) -> Result<PerMovePerftResult, PerftError>,
{
    fn perft(
        &mut self,
        position_fen: &str,
        position_uci_moves: &[UCIMove],
        depth: usize,
    ) -> Result<PerMovePerftResult, PerftError> {
        self(position_fen, position_uci_moves, depth)
    }
}
