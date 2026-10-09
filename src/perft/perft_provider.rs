//! Items related to making a chess move generator accessible to `perft-debugger`.

use crate::types::{per_move_perft_result::PerMovePerftResult, uci_move::UCIMove};

// TODO: Switch to thiserror errors for the rest of this library
/// A generic error returned while requesting or producing perft results.
pub type PerftError = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Provides the ability to generate per move perft results for a given chess position.
///
/// Implementations may wrap a chess engine, another move
/// generator, or any other source of perft results.
///
/// See [`PerftProvider::perft`] & [`PerMovePerftResult`] for implementation details.
pub trait PerftProvider {
    /// Generates per move perft results after replaying the given moves from a FEN position.
    ///
    /// See [`PerMovePerftResult`] for implementation details.
    ///
    /// # Arguments
    ///
    /// * `position_fen`: FEN string defining the starting position.
    /// * `position_uci_moves`: Moves to replay from the initial position
    ///                         before starting the perft search.
    /// * `depth`: Number of plies to search from the resulting position.
    ///
    /// # Errors
    ///
    /// Returns an error if the provider cannot run perft. \
    /// This could be for a variety of reasons.
    ///
    /// ### Note:
    ///
    /// This function splits `position_fen` and `position_uci_moves`
    /// to reveal bugs in an engine's ability to make moves.
    fn perft(
        &mut self,
        position_fen: &str,
        position_uci_moves: &[UCIMove],
        depth: usize,
    ) -> Result<PerMovePerftResult, PerftError>;
}

/// Allows a mutable closure with the [`PerftProvider::perft`] signature to be used as a provider on its own.
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
