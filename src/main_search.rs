//! Searches for and prints failing leaf moves found after an initial perft
//! discrepancy.
//!
//! The search repeats up to the configured limit, tracking reported move
//! paths so that it can look for additional failures without reporting the
//! same path again.

use anyhow::Result;

use crate::{
    cli::CLIArgs, fail_search::find_failing_leaf, initial_test::InitialTestFailReport,
    perft::perft_provider::PerftProvider,
};

/// Searches an initial perft failure for failing leaf moves and prints any
/// reports it finds.
///
/// Repeats the leaf search up to `args.max_searches` times, avoiding paths
/// already reported so it can look for additional failures. Prints a warning
/// when the search limit is reached; additional failing leaves may remain.
///
/// # Arguments
///
/// * `target_perft`: Provider being diagnosed.
/// * `true_perft`: Provider supplying the reference perft results.
/// * `args`: CLI settings, including the starting position and search limit.
/// * `initial_fail`: First failing depth and its per-move discrepancies.
///
/// # Returns
///
/// Returns `Ok(())` after the search completes and its reports are printed.
pub fn do_main_search(
    target_perft: &mut impl PerftProvider,
    true_perft: &mut impl PerftProvider,
    args: &CLIArgs,
    initial_fail: InitialTestFailReport,
) -> Result<()> {
    println!(
        "\nPerft failed at depth {}, finding failing leaf node(s)...",
        initial_fail.depth
    );
    println!("position fen {}", args.position_fen);

    let mut reported_lines = vec![];
    for i in 1..=args.max_searches {
        let fail_leaf_result = find_failing_leaf(
            target_perft,
            true_perft,
            &args.position_fen,
            initial_fail.depth,
            vec![],
            &initial_fail.failing_moves,
            &reported_lines,
        )?;

        let mut full_path = vec![];

        if let Some(report) = fail_leaf_result {
            println!("\nFailing leaf node found!");
            report.offending_move.report_fail_reason();

            print!("moves ");
            for uci_move in &report.path {
                print!("{} ", uci_move.to_string());
                full_path.push(*uci_move);
            }
            println!("");

            full_path.push(report.offending_move.uci_move);
        }

        reported_lines.push(full_path);

        if i == args.max_searches {
            println!("\nMax search limit hit. There might be more failing leaf nodes!");
        }
    }

    Ok(())
}
