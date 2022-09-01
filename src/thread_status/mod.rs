//! This module corresponds to `mach/thread_status.h`.
use super::vm_types::natural_t;
pub type thread_state_t = *mut natural_t;
pub type thread_state_flavor_t = core::ffi::c_int;

#[cfg(target_arch = "x86_64")]
mod x86_64;
#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

#[cfg(target_arch = "aarch64")]
mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use self::aarch64::*;
