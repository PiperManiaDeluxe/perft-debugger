#![allow(dead_code)]

// TODO: Use the same publicity and API as main.rs

mod bin_helpers;
mod engine_communication;
mod fail_search;
mod initial_test;
mod lib_ext;
mod perft;
mod types;

pub use lib_ext::debug_result::DebugResult;
pub use lib_ext::debugging::debug_perft_provider;
pub use perft::perft_provider::*;
pub use types::failed_move::*;
pub use types::per_move_perft_result::*;
pub use types::uci_move::{PromotionPiece, UCIMove};

pub mod helpers {
    pub use crate::types::uci_move::{SQUARE_NAMES, square_from_name, square_to_name};
}
