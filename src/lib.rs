//! Library for diagnosing move-generation bugs in chess engines.
//!
//! Implement [`PerftProvider`] for an engine or other source of perft results,
//! then call [`debug_perft_provider`] to compare its results with results from
//! the `cozy_chess` crate. The output reports whether the tested depths
//! passed and, when possible, identifies a missing or illegal move.

#![allow(dead_code)]

pub(crate) mod bin_helpers;
pub(crate) mod cli;
pub(crate) mod engine_communication;
pub(crate) mod fail_search;
pub(crate) mod initial_test;
pub(crate) mod lib_ext;
pub(crate) mod main_search;
pub(crate) mod perft;
pub(crate) mod types;

pub use lib_ext::debug_result::DebugResult;
pub use lib_ext::debugging::debug_perft_provider;
pub use perft::perft_provider::*;
pub use types::failed_move::*;
pub use types::per_move_perft_result::*;
pub use types::uci_move::{PromotionPiece, UCIMove};

pub mod helpers {
    pub use crate::types::uci_move::{SQUARE_NAMES, square_from_name, square_to_name};
}
