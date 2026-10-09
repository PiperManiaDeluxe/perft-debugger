use std::{str::FromStr, sync::LazyLock};

use perft_debugger::{
    PerMovePerftResult, PerftError, SingleMovePerftResult, UCIMove, debug_perft_provider,
};

const START_POS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const START_POS_MOVES: LazyLock<[Vec<UCIMove>; 20]> = LazyLock::new(|| {
    [
        vec![UCIMove::from_str("a2a3").unwrap()],
        vec![UCIMove::from_str("b2b3").unwrap()],
        vec![UCIMove::from_str("c2c3").unwrap()],
        vec![UCIMove::from_str("d2d3").unwrap()],
        vec![UCIMove::from_str("e2e3").unwrap()],
        vec![UCIMove::from_str("f2f3").unwrap()],
        vec![UCIMove::from_str("g2g3").unwrap()],
        vec![UCIMove::from_str("h2h3").unwrap()],
        vec![UCIMove::from_str("a2a4").unwrap()],
        vec![UCIMove::from_str("b2b4").unwrap()],
        vec![UCIMove::from_str("c2c4").unwrap()],
        vec![UCIMove::from_str("d2d4").unwrap()],
        vec![UCIMove::from_str("e2e4").unwrap()],
        vec![UCIMove::from_str("f2f4").unwrap()],
        vec![UCIMove::from_str("g2g4").unwrap()],
        vec![UCIMove::from_str("h2h4").unwrap()],
        vec![UCIMove::from_str("b1a3").unwrap()],
        vec![UCIMove::from_str("b1c3").unwrap()],
        vec![UCIMove::from_str("g1f3").unwrap()],
        vec![UCIMove::from_str("g1h3").unwrap()],
    ]
});

/// [`perft_debugger::PerftProvider`] that simulates an engine
/// that plays just e2e4 for every position fed to it.
fn only_plays_e2e4(
    _position_fen: &str,
    _position_uci_moves: &[UCIMove],
    _depth: usize,
) -> Result<PerMovePerftResult, PerftError> {
    Ok(PerMovePerftResult {
        moves: vec![SingleMovePerftResult {
            uci_move: UCIMove::from_str("e2e4").unwrap(),
            nodes: 1,
        }],
        total_nodes: 1,
    })
}

fn main() -> Result<(), PerftError> {
    let result = debug_perft_provider(
        // PerftProvider to debug.
        &mut only_plays_e2e4,
        // Position (in FEN) to start perft tests from.
        START_POS_FEN,
        // Max depth to test to.
        2,
        // Move lines to ignore while searching.
        &(*START_POS_MOVES),
    )?;

    println!("{}", result);

    Ok(())
}
