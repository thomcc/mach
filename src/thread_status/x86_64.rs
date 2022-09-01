//! This module corresponds to x86_64's `mach/thread_status.h`.

use super::*;

pub const x86_THREAD_STATE32: thread_state_flavor_t = 1;
pub const x86_FLOAT_STATE32: thread_state_flavor_t = 2;
pub const x86_EXCEPTION_STATE32: thread_state_flavor_t = 3;
pub const x86_THREAD_STATE64: thread_state_flavor_t = 4;
pub const x86_FLOAT_STATE64: thread_state_flavor_t = 5;
pub const x86_EXCEPTION_STATE64: thread_state_flavor_t = 6;
pub const x86_THREAD_STATE: thread_state_flavor_t = 7;
pub const x86_FLOAT_STATE: thread_state_flavor_t = 8;
pub const x86_EXCEPTION_STATE: thread_state_flavor_t = 9;
pub const x86_DEBUG_STATE32: thread_state_flavor_t = 10;
pub const x86_DEBUG_STATE64: thread_state_flavor_t = 11;
pub const x86_DEBUG_STATE: thread_state_flavor_t = 12;
pub const THREAD_STATE_NONE: thread_state_flavor_t = 13;
pub const x86_AVX_STATE32: thread_state_flavor_t = 16;
pub const x86_AVX_STATE64: thread_state_flavor_t = 17;
pub const x86_AVX_STATE: thread_state_flavor_t = 18;
