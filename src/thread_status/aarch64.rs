//! This module corresponds to aarch64's `mach/thread_status.h`.
use super::*;

/* Size of maximum exported thread state in words */
pub const ARM_THREAD_STATE_MAX: usize = 1296;

pub const ARM_THREAD_STATE: thread_state_flavor_t = 1;
pub const ARM_UNIFIED_THREAD_STATE: thread_state_flavor_t = ARM_THREAD_STATE;
pub const ARM_VFP_STATE: thread_state_flavor_t = 2;
pub const ARM_EXCEPTION_STATE: thread_state_flavor_t = 3;
pub const ARM_DEBUG_STATE: thread_state_flavor_t = 4; /* pre-armv8 */
pub const THREAD_STATE_NONE: thread_state_flavor_t = 5;
pub const ARM_THREAD_STATE64: thread_state_flavor_t = 6;
pub const ARM_EXCEPTION_STATE64: thread_state_flavor_t = 7;
//      ARM_THREAD_STATE_LAST    8 /* legacy */
pub const ARM_THREAD_STATE32: thread_state_flavor_t = 9;
pub const ARM_DEBUG_STATE32: thread_state_flavor_t = 14;
pub const ARM_DEBUG_STATE64: thread_state_flavor_t = 15;
pub const ARM_NEON_STATE: thread_state_flavor_t = 16;
pub const ARM_NEON_STATE64: thread_state_flavor_t = 17;
pub const ARM_CPMU_STATE64: thread_state_flavor_t = 18;
pub const ARM_PAGEIN_STATE: thread_state_flavor_t = 27;
