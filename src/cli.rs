use clap::Parser;

// TODO: Include "infinite mode" setting
// infinite mode loops over every illegal move until
// we run out or the application is quit
// all results are written to a file

/// CLI arguments for perft-debugger
#[derive(Debug, Parser)]
#[command(version, long_about = None)]
#[command(about = "Finds illegal moves an engine generates by using perft commands")]
pub struct CLIArgs {
    /// Executable path for the engine to be diagnosed
    pub engine_path: String,

    /// Position (in FEN) to diagnose
    #[arg(default_value = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1")]
    pub position_fen: String,

    /// Max perft depth to test
    #[arg(short = 'd', long = "depth", default_value = "4")]
    pub max_depth: usize,

    /// Maximum illegal moves to attempt a search for
    #[arg(short = 's', long = "searches", default_value = "3")]
    pub max_searches: usize,
}
