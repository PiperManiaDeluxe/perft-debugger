use anyhow::Result;

use crate::{
    cli::CLIArgs, fail_search::find_failing_leaf, helpers::parse_board,
    initial_test::InitialTestFailReport, perft::perft_target::PerftTarget,
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
    target_perft: &mut impl PerftTarget,
    true_perft: &mut impl PerftTarget,
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
        let board = parse_board(&args.position_fen)?;

        let fail_leaf_result = find_failing_leaf(
            target_perft,
            true_perft,
            &args.position_fen,
            &board,
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
            for mv in &report.path {
                print!("{} ", mv.to_string());
                full_path.push(*mv);
            }
            println!("");

            full_path.push(report.offending_move.mv);
        }

        reported_lines.push(full_path);

        if i == args.max_searches {
            println!("\nMax search limit hit. There might be more failing leaf nodes!");
        }
    }

    Ok(())
}
