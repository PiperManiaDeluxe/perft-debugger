use std::sync::LazyLock;

use perft_debugger::{
    PerMovePerftResult, PerftError, SingleMovePerftResult, UCIMove, debug_perft_provider,
};

const START_POS_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
const START_POS_MOVES: LazyLock<[Vec<UCIMove>; 20]> = LazyLock::new(|| {
    [
        vec![UCIMove::from_str_unwrap("a2a3")],
        vec![UCIMove::from_str_unwrap("b2b3")],
        vec![UCIMove::from_str_unwrap("c2c3")],
        vec![UCIMove::from_str_unwrap("d2d3")],
        vec![UCIMove::from_str_unwrap("e2e3")],
        vec![UCIMove::from_str_unwrap("f2f3")],
        vec![UCIMove::from_str_unwrap("g2g3")],
        vec![UCIMove::from_str_unwrap("h2h3")],
        vec![UCIMove::from_str_unwrap("a2a4")],
        vec![UCIMove::from_str_unwrap("b2b4")],
        vec![UCIMove::from_str_unwrap("c2c4")],
        vec![UCIMove::from_str_unwrap("d2d4")],
        vec![UCIMove::from_str_unwrap("e2e4")],
        vec![UCIMove::from_str_unwrap("f2f4")],
        vec![UCIMove::from_str_unwrap("g2g4")],
        vec![UCIMove::from_str_unwrap("h2h4")],
        vec![UCIMove::from_str_unwrap("b1a3")],
        vec![UCIMove::from_str_unwrap("b1c3")],
        vec![UCIMove::from_str_unwrap("g1f3")],
        vec![UCIMove::from_str_unwrap("g1h3")],
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
            mv: UCIMove::from_str_unwrap("e2e4"),
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
