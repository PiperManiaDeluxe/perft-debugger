//! Perft provider implementations and utilities for producing
//! and comparing per move perft results.
//!
//! This module includes the [`PerftProvider`] interface, a `cozy_chess`
//! reference implementation, a [`UCIEngine`] implementation, and logic for
//! comparing results and identifying move discrepancies.
//!
//! [`PerftProvider`]: crate::perft::perft_provider::PerftProvider
//! [`UCIEngine`]: crate::engine_communication::UCIEngine

pub mod compare_results;
pub mod cozy_perft;
pub mod engine_perft;
pub mod perft_provider;
