//! Types representing perft node counts for each move from a position
//! and the total node count for the search.

use std::ffi::c_char;

use libc::size_t;

/// Per-move perft results for a given root position.
#[repr(C)]
pub struct PerMovePerftResult {
    /// [`SingleMovePerftResult`]s for every move playable on the root position.
    pub moves: *const SingleMovePerftResult,
    /// `moves` length.
    pub move_count: size_t,
    /// Total number of nodes reached.
    pub total_nodes: size_t,
}

/// Perft results for a single root move in a given position.
#[repr(C)]
pub struct SingleMovePerftResult {
    /// Move for this result in UCI format.
    pub uci_move: *const c_char,
    /// Total nodes reached after this move.
    pub nodes: size_t,
}
