//! Implements [`PerftProvider`] for a running UCI engine.
//!
//! Sends the engine position and perft commands, then parses its output into
//! per move and total node counts. The engine must support the commands and
//! output format described in the implementation documentation.

use std::str::FromStr;

use crate::{
    bin_helpers::string_is_uci_move,
    engine_communication::UCIEngine,
    perft::perft_provider::{PerftError, PerftProvider},
    types::{
        per_move_perft_result::{PerMovePerftResult, SingleMovePerftResult},
        uci_move::UCIMove,
    },
};

/// Implements [`PerftProvider`] by requesting perft results from a UCI engine.
///
/// Sends the engine a `position` command containing the FEN and any moves to replay,
/// then runs `go perft` at the requested depth. Then parses the engine's
/// output into per move node counts and a total node count.
///
/// ### Note: Engine compatibility
///
/// The engine must support the UCI commands `position fen [FEN] moves [MOVES]`
/// and `go perft [DEPTH]`. Its perft output must include one line per move
/// in the form `[move]: [node count]`, followed by a total node count line
/// in the form `Nodes searched: [total node count]`.
impl PerftProvider for UCIEngine {
    fn perft(
        &mut self,
        position_fen: &str,
        position_uci_moves: &[UCIMove],
        depth: usize,
    ) -> Result<PerMovePerftResult, PerftError> {
        let mut position_command = format!("position fen {position_fen}");

        if !position_uci_moves.is_empty() {
            position_command.push_str(" moves");
            for uci_move in position_uci_moves {
                let uci_move = *uci_move;
                position_command.push(' ');
                position_command.push_str(&uci_move.to_string());
            }
        }

        // Set position
        self.send_command(&position_command)?;

        // Run perft
        self.send_command(&format!("go perft {depth}"))?;

        // Parse the outputs
        let mut result = PerMovePerftResult::default();

        for line in self.iter_rx() {
            let is_complete = parse_engine_result_line(&mut result, &line)?;

            if is_complete {
                break;
            }
        }

        Ok(result)
    }
}

/// Parses a line from the engine and pushes any perft results found.
///
/// Returns true when the total nodes searched result has been parsed.
fn parse_engine_result_line(
    result: &mut PerMovePerftResult,
    line: &str,
) -> Result<bool, PerftError> {
    let line = line.trim();
    if line.is_empty() {
        return Ok(false);
    }

    let Some((prefix, nodes_str)) = line.split_once(": ") else {
        return Ok(false);
    };

    if string_is_uci_move(prefix) {
        // This is a move result
        let nodes = nodes_str
            .parse::<usize>()
            .map_err(|_| "Nodes not parsable")?;

        result.moves.push(SingleMovePerftResult {
            uci_move: UCIMove::from_str(prefix)?,
            nodes,
        });
    } else if prefix == "Nodes searched" {
        // This is the total result
        let nodes = nodes_str
            .parse::<usize>()
            .map_err(|_| "Nodes not parsable")?;
        result.total_nodes = nodes;

        return Ok(true);
    }

    Ok(false)
}
