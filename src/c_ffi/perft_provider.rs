//! TODO_DOCS

use std::{
    ffi::{CStr, CString, c_char, c_void},
    ptr, slice,
};

use libc::size_t;

use crate::c_ffi::types::{
    per_move_perft_result::{PerMovePerftResult, SingleMovePerftResult},
    result::{PerftResult, RunResult},
};

/// TODO_DOCS
#[repr(C)]
pub struct CPerftProvider {
    pub user_data: *mut c_void,
    pub perft: Option<
        unsafe extern "C" fn(
            user_data: *mut c_void,
            // &str
            fen: *const c_char,
            // &[&str]
            moves: *const *const c_char,
            // moves.len()
            move_count: size_t,
            // usize
            depth: size_t,
            output: *mut PerMovePerftResult,
        ) -> PerftResult,
    >,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_perft_test(provider: *mut CPerftProvider) -> RunResult {
    let panic_result = std::panic::catch_unwind(move || {
        if provider.is_null() {
            return RunResult::fail("Called on null CPerftProvider").unwrap();
        }

        let provider = unsafe { &*provider };

        let Some(perft) = provider.perft else {
            // No callback was provided.
            return RunResult::fail("No perft callback set").unwrap();
        };

        let fen = match CString::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1") {
            Ok(fen) => fen,
            Err(_) => return RunResult::fail("FEN string is null").unwrap(),
        };

        let mut output = PerMovePerftResult {
            moves: ptr::null(),
            move_count: 0,
            total_nodes: 0,
        };

        let result = unsafe {
            perft(
                provider.user_data,
                fen.as_ptr(),
                ptr::null(),
                0,
                1,
                &mut output,
            )
        };

        // Handle errors
        if !result.successful {
            todo!()
        }

        output.moves;

        let moves: &[SingleMovePerftResult] = if output.move_count > 0 {
            if output.moves.is_null() {
                panic!("Moves is null");
            }

            unsafe { slice::from_raw_parts(output.moves, output.move_count) }
        } else {
            &[]
        };

        for mv in moves {
            if mv.uci_move.is_null() {
                panic!("Move uci is null");
            }

            let uci_move = unsafe { CStr::from_ptr(mv.uci_move) }
                .to_str()
                .expect("Could not unwrap cstr");

            println!("{uci_move}: {}", mv.nodes);
        }

        println!("{}", output.total_nodes);

        RunResult::SUCCESSFUL
    });

    match panic_result {
        Err(_err) => RunResult::fail("C callback panicked").unwrap(),
        Ok(status) => status,
    }
}
