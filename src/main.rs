use anyhow::{Context, Result, anyhow};
use clap::Parser;
use cozy_chess::Move;

use crate::{
    engine_communication::UCIEngine, initial_test::initial_perft_test, main_search::do_main_search,
    perft::engine_perft::get_uci_engine_perft_results, results::PerMovePerftResult,
};

/// Computes the per-move perft results to debug.
///
/// This is an intentional adaptation point.
///
/// If your engine is not compatible with this debugger,
/// replace this body with a perft implementation
/// that returns results for every root move.
///
/// Note: This function splits `start_position` and `moves`
/// to reveal bugs in an engine's ability to make moves.
///
/// See [`PerMovePerftResult`] for details.
///
/// # Arguments
///
/// * `start_position`: Position (in FEN) to initialize the board state in.
/// * `moves`: Moves to make before starting the perft test.
/// * `depth`: Total depth to explore.
/// * `engine`: [`UCIEngine`] process handle for the engine we want to debug.
///     - Note: If you are swapping out this function this argument is not entirely necessary!
pub fn get_debug_perft_results(
    engine: &mut UCIEngine,
    start_position: &str,
    moves: &[Move],
    depth: usize,
) -> Result<PerMovePerftResult> {
    get_uci_engine_perft_results(engine, start_position, moves, depth)
}

pub mod engine_communication;
pub mod fail_search;
pub mod helpers;
pub mod initial_test;
pub mod main_search;
pub mod perft;
pub mod results;

// TODO: Include "infinite mode" setting
// infinite mode loops over every illegal move until
// we run out or the application is quit
// all results are written to a file

#[derive(Debug, Parser)]
#[command(version, long_about = None)]
#[command(about = "Finds illegal moves an engine generates by using perft commands")]
pub struct CLIArgs {
    /// Executable path for the engine to be diagnosed
    engine_path: String,

    /// Position (in FEN) to diagnose
    #[arg(default_value = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")]
    position_fen: String,

    /// Max perft depth to test
    #[arg(short = 'd', long = "depth", default_value = "4")]
    max_depth: usize,

    /// Maximum illegal moves to attempt a search for
    #[arg(short = 's', long = "searches", default_value = "3")]
    max_searches: usize,
}

fn main() -> Result<()> {
    let args = CLIArgs::parse();

    if args.max_depth == 0 {
        return Err(anyhow!("The minimum test depth is 1"));
    }

    let mut engine =
        UCIEngine::open_engine(&args.engine_path).context("Couldn't initialize engine process")?;

    let run_result = run(&mut engine, args);

    match run_result {
        Ok(_) => {
            engine
                .close()
                .context("Error while closing engine process")?;
        }
        Err(err) => {
            eprintln!(
                "Caught an error before reaching engine shutdown! Attempting to shutdown engine..."
            );

            if let Err(engine_err) = engine.close() {
                eprintln!("Error while shutting down engine!");
                eprintln!("{engine_err:#}");
            }

            return Err(err);
        }
    }

    Ok(())
}

/// Error catching wrapper for the main functionality of this program.
fn run(engine: &mut UCIEngine, args: CLIArgs) -> Result<()> {
    // Find the first depth we fail a perft test at, if at all
    let initial_result = initial_perft_test(&args.position_fen, args.max_depth, engine)
        .context("Failed to run initial perft test")?;

    match initial_result {
        Some(initial_fail) => {
            do_main_search(engine, &args, initial_fail)?;
        }
        None => {
            println!("The engine passed all perft depths!");
        }
    }

    Ok(())
}
