//! Helper functions for the binary half of this crate.

use std::{ops::RangeInclusive, str::FromStr};

use anyhow::{Context, Result};
use cozy_chess::{Board, Color, File, Move, Piece, Square};

/// Convenience wrapper function to parse a new [`cozy_chess::Board`] from a FEN position.
///
/// Wraps the [`cozy_chess::FenParseError`] in an [`anyhow::Result`]
/// so we can use [`anyhow`]'s error handling.
pub fn parse_board(position_fen: &str) -> Result<Board> {
    Board::from_fen(position_fen, false)
        .map_err(|err| anyhow::anyhow!("{err}"))
        .context("Error while parsing board from FEN")
}

/// Convenience wrapper function to parse a [`cozy_chess::Move`] from a UCI compatible string.
///
/// Wraps the [`cozy_chess::MoveParseError`] in an [`anyhow::Result`]
/// so we can use [`anyhow`]'s error handling.
///
/// Note: This will fail if we have to use the resulting move on a [`cozy_chess::Board`]. \
/// [`cozy_chess`] defines castling as the king moving to the rook's square. \
/// UCI spec defines castling as the king moving to its end square.
pub fn parse_move(move_uci: &str) -> Result<Move> {
    Move::from_str(move_uci)
        .map_err(|err| anyhow::anyhow!("{err}"))
        .context("Error while parsing move from UCI string")
}

/// Returns a [`cozy_chess::Move`] with castling remapped to proper UCI spec.
///
/// [`cozy_chess`] defines castling as the king moving to the rook's square. \
/// UCI spec defines castling as the king moving to its end square.
///
/// # Arguments
///
/// * `board`: Board that the move __will__ be played on.
/// * `cozy_move`: Move to check.
pub fn remap_castling_move(board: &Board, cozy_move: &Move) -> Move {
    if !move_is_cozy_castling(board, cozy_move) {
        return cozy_move.clone();
    };

    // This move has to be a castling move, remap the move.
    let mut remapped_move = cozy_move.clone();

    let destination = if cozy_move.to.file() == File::A {
        Square::C1
    } else {
        Square::G1
    };

    let white_king = board.color_on(cozy_move.from) == Some(Color::White);
    if !white_king {
        remapped_move.to = destination.flip_rank();
    } else {
        remapped_move.to = destination;
    }

    remapped_move
}

/// Returns true if a move represents castling in [`cozy_chess`]'s format.
///
/// [`cozy_chess`] defines castling as the king moving to the rook's square. \
/// UCI spec defines castling as the king moving to its end square.
///
/// # Arguments
///
/// * `board`: Board that the move __will__ be played on.
/// * `cozy_move`: Move to check.
pub fn move_is_cozy_castling(board: &Board, cozy_move: &Move) -> bool {
    let king_moved = board.piece_on(cozy_move.from) == Some(Piece::King);
    let rook_attacked = board.piece_on(cozy_move.to) == Some(Piece::Rook);

    if !(king_moved && rook_attacked) {
        // Cozy chess maps castling as the king attacking the castling rook.
        return false;
    }

    let white_king = board.color_on(cozy_move.from) == Some(Color::White);
    let king_start_square = if white_king { Square::E1 } else { Square::E8 };

    let king_on_start = cozy_move.from == king_start_square;
    if !king_on_start {
        // The king must be on its starting square to castle.
        return false;
    }

    // We can ignore the specific castling side.
    // If the rook and king are already two files away, the move *has* to be castling.
    let rook_on_starting_file = cozy_move.to.file() == File::A || cozy_move.to.file() == File::H;
    if !rook_on_starting_file {
        // The rook must be on its starting file to castle.
        return false;
    }

    true
}

/// Returns true if `string` is likely to be a UCI compatible move description.
pub fn string_is_uci_move(string: &str) -> bool {
    if string.len() < 4 || string.len() > 5 {
        // UCI moves are always either 4 or 5 (promotion) characters long.
        return false;
    }

    let mut chars = string.chars();

    let char_opt_in_range = |char: Option<char>, range: RangeInclusive<char>| {
        let Some(char) = char else {
            return false;
        };

        range.contains(&char)
    };

    for _square in 0..2 {
        let file_valid = char_opt_in_range(chars.next(), 'a'..='h');
        let rank_valid = char_opt_in_range(chars.next(), '1'..='8');

        if !file_valid || !rank_valid {
            return false;
        }
    }

    true
}
