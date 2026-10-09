//! UCI format chess moves, and helpers for converting
//! between move strings and board square indices.

use std::{
    fmt::{self, Write},
    str::FromStr,
};

use cozy_chess::{Board, Move, Piece};

use crate::{bin_helpers::remap_castling_move, perft::perft_provider::PerftError};

/// A chess move represented by UCI coordinate notation.
///
/// A move consists of its source and destination squares, optionally followed
/// by a lowercase promotion piece character. For example, "e2e4" or "a7a8q".
///
/// This type stores squares as indices with `a1` at index `0` and
/// `h8` at index `63`. Castling destinations follow UCI notation:
/// the king's destination square, rather then the rook's source square.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct UCIMove {
    /// The moved piece's starting square as an index.
    pub from: usize,
    /// The moved piece's end square as an index.
    pub to: usize,
    /// The type of piece this move promoted to, if it exists.
    pub promotion: Option<PromotionPiece>,
}

impl UCIMove {
    /// Converts a `cozy_chess` move to UCI coordinates.
    ///
    /// Remaps castling moves so their destination is the king's destination
    /// square, as required by UCI notation.
    pub(crate) fn from_cozy(board: &Board, cozy_mv: &Move) -> Self {
        let cozy_mv = remap_castling_move(board, cozy_mv);

        UCIMove {
            from: cozy_mv.from as usize,
            to: cozy_mv.to as usize,
            promotion: match cozy_mv.promotion {
                Some(Piece::Knight) => Some(PromotionPiece::Knight),
                Some(Piece::Bishop) => Some(PromotionPiece::Bishop),
                Some(Piece::Rook) => Some(PromotionPiece::Rook),
                Some(Piece::Queen) => Some(PromotionPiece::Queen),
                Some(_) => None,
                None => None,
            },
        }
    }
}

impl fmt::Display for UCIMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(square_to_name(self.from))?;
        f.write_str(square_to_name(self.to))?;

        if let Some(promotion) = self.promotion {
            f.write_char(promotion.char())?;
        }

        Ok(())
    }
}

impl fmt::Debug for UCIMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UCIMove")
            .field("from", &square_to_name(self.from))
            .field("to", &square_to_name(self.to))
            .field("promotion", &self.promotion)
            .finish()
    }
}

impl FromStr for UCIMove {
    type Err = PerftError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some(promo_char) = s.chars().last() else {
            return Err("String is empty".into());
        };

        let squares = [&s[0..=1], &s[2..=3]];
        let [from, to] = squares.map(square_from_name);

        let promotion = match promo_char {
            'n' => Some(PromotionPiece::Knight),
            'b' => Some(PromotionPiece::Bishop),
            'r' => Some(PromotionPiece::Rook),
            'q' => Some(PromotionPiece::Queen),
            c if s.len() >= 5 => return Err(format!("Invalid promotion char reached: {c}").into()),
            _ => None,
        };

        Ok(Self {
            from,
            to,
            promotion,
        })
    }
}

/// A chess piece type a pawn can promote to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PromotionPiece {
    Knight,
    Bishop,
    Rook,
    Queen,
}

impl PromotionPiece {
    /// Returns the character representing this promotion, as defined by
    /// [Universal Chess Interface](https://en.wikipedia.org/wiki/Universal_Chess_Interface)
    /// specification.
    pub const fn char(&self) -> char {
        match *self {
            PromotionPiece::Knight => 'n',
            PromotionPiece::Bishop => 'b',
            PromotionPiece::Rook => 'r',
            PromotionPiece::Queen => 'q',
        }
    }
}

/// Names of the 64 board squares, indexed from `a1` (0) to `h8` (63).
pub const SQUARE_NAMES: [&str; 64] = [
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1", //
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2", //
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3", //
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4", //
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5", //
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6", //
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7", //
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8", //
];

/// Returns the algebraic name of a square by its index.
///
/// Indexed from `a1` (0) to `h8` (63).
///
/// # Panics
///
/// Panics if `square` is greater than `63`.
pub const fn square_to_name(square: usize) -> &'static str {
    SQUARE_NAMES[square]
}

/// Converts a two-character algebraic square name, such as `"e4"`, to its
/// board index.
///
/// Indexed from `a1` (0) to `h8` (63).
///
/// # Panics
///
/// Panics if `name` is not exactly two bytes, or if it contains a file outside
/// `a`–`h` or a rank outside `1`–`8`.
pub const fn square_from_name(name: &str) -> usize {
    let bytes = name.as_bytes();
    assert!(bytes.len() == 2, "Square name must be 2 characters!");

    let file = match bytes[0] {
        b'a'..=b'h' => bytes[0] - b'a',
        _ => panic!("File is not a-h"),
    };

    let rank = match bytes[1] {
        b'1'..=b'8' => bytes[1] - b'1',
        _ => panic!("Rank is not 1-8"),
    };

    rank as usize * 8 + file as usize
}
