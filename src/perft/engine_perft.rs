use anyhow::{Context, Result};
use cozy_chess::Move;

use crate::{
    engine_communication::UCIEngine,
    helpers::{parse_move, string_is_uci_move},
    result_types::{PerMovePerftResult, SingleMovePerftResult},
};

/// Runs and extracts per-move perft results on a [`UCIEngine`] process.
///
/// Note: This function splits `start_position` and `moves`
/// to reveal bugs in an engine's ability to make moves.
///
/// # Arguments
///
/// * `engine`: [`UCIEngine`] process to run on.
/// * `start_fen`: Position (in FEN) to initialize the board state in.
/// * `moves`: Moves to make before starting the perft test.
/// * `depth`: Total depth to explore.
pub fn get_uci_engine_perft_results(
    engine: &mut UCIEngine,
    start_fen: &str,
    moves: &[Move],
    depth: usize,
) -> Result<PerMovePerftResult> {
    // Send the position
    let mut position_command = format!("position fen {start_fen}");

    if !moves.is_empty() {
        position_command.push_str(" moves");
        for mv in moves {
            position_command.push(' ');
            position_command.push_str(&mv.to_string());
        }
    }

    engine.send_command(&position_command)?;

    // Run perft
    engine.send_command(&format!("go perft {depth}"))?;

    // Parse the outputs
    let mut result = PerMovePerftResult::default();

    for line in engine.iter_rx() {
        let is_complete = parse_engine_result_line(&mut result, &line)
            .with_context(|| format!("Error while parsing a line from the engine: \"{line}\""))?;

        if is_complete {
            break;
        }
    }

    Ok(result)
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
        let result_move = parse_move(prefix)?;

        result.moves.push(SingleMovePerftResult {
            mv: result_move,
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
