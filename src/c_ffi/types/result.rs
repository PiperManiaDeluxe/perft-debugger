//! TODO_DOCS

use std::{
    ffi::{CString, NulError, c_char},
    ptr,
};

// Owned by C user
/// TODO_DOCS
#[repr(C)]
pub struct PerftResult {
    /// TODO_DOCS
    pub successful: bool,
    /// TODO_DOCS
    pub error: *const c_char,
}

impl PerftResult {
    /// TODO_DOCS
    pub const SUCCESSFUL: Self = Self {
        successful: true,
        error: ptr::null(),
    };

    /// TODO_DOCS
    pub fn fail(fail_reason: &str) -> Result<Self, NulError> {
        let error = CString::new(fail_reason)?;

        Ok(Self {
            successful: false,
            error: error.into_raw(),
        })
    }
}

// Owned by Rust FFI
/// TODO_DOCS
#[repr(C)]
pub struct RunResult {
    /// TODO_DOCS
    pub successful: bool,
    /// TODO_DOCS
    pub error: *const c_char,
}

impl RunResult {
    /// TODO_DOCS
    pub const SUCCESSFUL: Self = Self {
        successful: true,
        error: ptr::null(),
    };

    /// TODO_DOCS
    pub fn fail(fail_reason: &str) -> Result<Self, NulError> {
        let error = CString::new(fail_reason)?;

        Ok(Self {
            successful: false,
            error: error.into_raw(),
        })
    }
}

/// TODO_DOCS
#[unsafe(no_mangle)]
pub unsafe extern "C" fn free_run_result(result: RunResult) {
    if !result.error.is_null() {
        unsafe {
            drop(CString::from_raw(result.error as *mut c_char));
        }
    }
}
