//! Command line tool for finding move generation bugs in UCI chess engines.

use anyhow::{Context, Result, anyhow};
use clap::Parser;

use crate::{
    cli::CLIArgs, engine_communication::UCIEngine, initial_test::initial_perft_test,
    main_search::do_main_search, perft::cozy_perft::CozyChessPerftTarget,
};

mod bin_helpers;
mod cli;
mod engine_communication;
mod fail_search;
mod initial_test;
mod main_search;
mod perft;
mod types;

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
    let mut true_perft = CozyChessPerftTarget;

    // Find the first depth we fail a perft test at, if at all
    let initial_result = initial_perft_test(
        engine,
        &mut true_perft,
        &args.position_fen,
        args.max_depth,
        false,
        &[],
    )
    .context("Failed to run initial perft test")?;

    match initial_result {
        Some(initial_fail) => {
            do_main_search(engine, &mut true_perft, &args, initial_fail)?;
        }
        None => {
            println!("The engine passed all perft depths!");
        }
    }

    Ok(())
}
