//! TODO_DOCS

use std::{
    ffi::{CString, c_char, c_int, c_void},
    ptr,
};

use libc::size_t;

/// TODO_DOCS
#[repr(C)]
pub struct FFISlice {
    pub data: *const u8,
    pub len: size_t,
}

impl FFISlice {
    pub const EMPTY: Self = Self {
        data: ptr::null(),
        len: 0,
    };
}

/// TODO_DOCS
#[repr(C)]
pub struct FFIBuffer {
    pub data: *mut u8,
    pub len: size_t,
}

impl FFIBuffer {
    pub const EMPTY: Self = Self {
        data: ptr::null_mut(),
        len: 0,
    };
}

/// TODO_DOCS
#[repr(C)]
pub struct CPerftProvider {
    pub user_data: *mut c_void,
    pub perft: Option<
        unsafe extern "C" fn(
            user_data: *mut c_void,
            fen: *const c_char,
            moves: *const FFISlice,
            move_count: size_t,
            depth: size_t,
            output: *mut FFIBuffer,
            error: *mut FFIBuffer,
        ) -> c_int,
    >,
    pub free_buffer: Option<unsafe extern "C" fn(user_data: *mut c_void, buffer: FFIBuffer)>,
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_perft_test(provider: *mut CPerftProvider) -> c_int {
    let result = std::panic::catch_unwind(move || {
        if provider.is_null() {
            return -1;
        }

        let provider = unsafe { &*provider };

        let Some(perft) = provider.perft else {
            // No callback was provided.
            return -2;
        };

        let fen = match CString::new("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1") {
            Ok(fen) => fen,
            Err(_) => return -3,
        };

        let mut output = FFIBuffer::EMPTY;
        let mut error = FFIBuffer::EMPTY;

        let status = unsafe {
            perft(
                provider.user_data,
                fen.as_ptr(),
                ptr::null(),
                0,
                1,
                &mut output,
                &mut error,
            )
        };

        let Some(free_buffer) = provider.free_buffer else {
            // Cannot safely release callback allocated buffers.
            return -4;
        };

        unsafe {
            if !output.data.is_null() {
                free_buffer(provider.user_data, output);
            }
            if !error.data.is_null() {
                free_buffer(provider.user_data, error);
            }
        }

        status
    });

    match result {
        Err(_err) => 1,
        Ok(status) => status,
    }
}
