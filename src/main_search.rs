use anyhow::Result;

use crate::{
    cli::CLIArgs, fail_search::find_failing_leaf, initial_test::InitialTestFailReport,
    perft::perft_provider::PerftProvider,
};

/// Searches for and prints failing leaf moves from an initial perft failure.
///
/// Repeats the search up to `args.max_searches` times,
/// avoiding paths that were already reported,
/// and prints a warning if the maximum search count is reached.
///
/// # Arguments
///
/// * `engine`: [`UCIEngine`] process handle to diagnose.
/// * `args`: Commandline arguments for this run.
/// * `initial_fail`: The depth and failing moves of the initial perft report.
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
