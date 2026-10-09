# perft-debugger as a library

`perft-debugger` can also be used directly as a library!

Implement `PerftProvider` to supply perft results from your engine. \
Then call `debug_perft_provider` to check them against reference results.

## Provide perft results

A provider receives:

1. A chess position in FEN.
2. A sequence of UCI moves to replay.
3. A perft depth to explore.

And must return the node count for each legal move from the
resulting position, along with the total node count.

A `PerftProvider` is as easy to implement as:

```rust
/// [`PerftProvider`] that simulates an engine that
/// plays just e2e4 for every position fed to it.
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
```

(`PerftProviders` are not limited to closures. See [perft_provider.rs](src/perft/perft_provider.rs) for more.)

## Run the diagnostic

```rust
let result = debug_perft_provider(
    // PerftProvider to debug.
    &mut only_plays_e2e4,
    // Position (in FEN) to start perft tests from.
    START_POS_FEN,
    // Max depth to test to.
    2,
    // Move lines to ignore while searching.
    &*START_POS_MOVES,
)?;

println!("{}", result);
```

```
Perft failed at depth 2!

Failing move found:
e2e4
Fail reason:
Illegal move was generated

Fail start condition:
position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves e2e4
```

