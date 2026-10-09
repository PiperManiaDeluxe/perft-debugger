use anyhow::{Context, Result};

use crate::{
    engine_communication::UCIEngine,
    helpers::string_is_uci_move,
    perft::perft_target::{Error, PerftTarget},
    types::{
        per_move_perft_result::{PerMovePerftResult, SingleMovePerftResult},
        uci_move::UCIMove,
    },
};

/// TODO: Docs?
impl PerftTarget for UCIEngine {
    fn perft(
        &mut self,
        position_fen: &str,
        position_uci_moves: &[UCIMove],
        depth: usize,
    ) -> Result<PerMovePerftResult, Error> {
        let mut position_command = format!("position fen {position_fen}");

        if !position_uci_moves.is_empty() {
            position_command.push_str(" moves");
            for mv in position_uci_moves {
                let mv = *mv;
                position_command.push(' ');
                position_command.push_str(&mv.to_string());
            }
        }

        // Set position
        self.send_command(&position_command)?;

        // Run perft
        self.send_command(&format!("go perft {depth}"))?;

        // Parse the outputs
        let mut result = PerMovePerftResult::default();

        for line in self.iter_rx() {
            let is_complete = parse_engine_result_line(&mut result, &line).with_context(|| {
                format!("Error while parsing a line from the engine: \"{line}\"")
            })?;

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
fn parse_engine_result_line(result: &mut PerMovePerftResult, line: &str) -> Result<bool> {
    let line = line.trim();
    if line.is_empty() {
        return Ok(false);
    }

    let Some((prefix, nodes_str)) = line.split_once(": ") else {
        return Ok(false);
    };

    if string_is_uci_move(prefix) {
        // This is a move result
        let nodes = nodes_str.parse::<usize>().context("Nodes not parsable")?;

        result.moves.push(SingleMovePerftResult {
            mv: UCIMove::from_str_unwrap(prefix),
            nodes,
        });
    } else if prefix == "Nodes searched" {
        // This is the total result
        let nodes = nodes_str.parse::<usize>().context("Nodes not parsable")?;
        result.total_nodes = nodes;

        return Ok(true);
    }

    Ok(false)
}
