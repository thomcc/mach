//! This module roughly corresponds to `mach/vm_statistics.h`

use crate::vm_types::{integer_t, natural_t};

pub type vm_statistics_t = *mut vm_statistics;
pub type vm_statistics_data_t = vm_statistics;
#[cfg(feature = "deprecated")]
#[deprecated(
    since = "0.2.3",
    note = "`pmap_statistics_t` was removed after MacOSX 10.3.9"
)]
#[allow(deprecated)]
pub type pmap_statistics_t = *mut pmap_statistics;

pub const VM_PAGE_QUERY_PAGE_PRESENT: integer_t = 1;
pub const VM_PAGE_QUERY_PAGE_FICTITIOUS: integer_t = 1 << 1;
pub const VM_PAGE_QUERY_PAGE_REF: integer_t = 1 << 2;
pub const VM_PAGE_QUERY_PAGE_DIRTY: integer_t = 1 << 3;

pub const VM_MEMORY_MALLOC: core::ffi::c_uint = 1;
pub const VM_MEMORY_MALLOC_SMALL: core::ffi::c_uint = 2;
pub const VM_MEMORY_MALLOC_LARGE: core::ffi::c_uint = 3;
pub const VM_MEMORY_MALLOC_HUGE: core::ffi::c_uint = 4;
pub const VM_MEMORY_SBRK: core::ffi::c_uint = 5;
pub const VM_MEMORY_ANALYSIS_TOOL: core::ffi::c_uint = 10;
pub const VM_MEMORY_MACH_MSG: core::ffi::c_uint = 20;
pub const VM_MEMORY_IOKIT: core::ffi::c_uint = 21;
pub const VM_MEMORY_STACK: core::ffi::c_uint = 30;
pub const VM_MEMORY_GUARD: core::ffi::c_uint = 31;
pub const VM_MEMORY_SHARED_PMAP: core::ffi::c_uint = 32;
pub const VM_MEMORY_DYLIB: core::ffi::c_uint = 33;
pub const VM_MEMORY_APPKIT: core::ffi::c_uint = 40;
pub const VM_MEMORY_FOUNDATION: core::ffi::c_uint = 41;
pub const VM_MEMORY_COREGRAPHICS: core::ffi::c_uint = 42;
pub const VM_MEMORY_CARBON: core::ffi::c_uint = 43;
pub const VM_MEMORY_JAVA: core::ffi::c_uint = 44;
pub const VM_MEMORY_ATS: core::ffi::c_uint = 50;
pub const VM_MEMORY_DYLD: core::ffi::c_uint = 60;
pub const VM_MEMORY_DYLD_MALLOC: core::ffi::c_uint = 61;
pub const VM_MEMORY_APPLICATION_SPECIFIC_1: core::ffi::c_uint = 240;
pub const VM_MEMORY_APPLICATION_SPECIFIC_16: core::ffi::c_uint = 255;

pub const VM_FLAGS_FIXED: core::ffi::c_int = 0x0;
pub const VM_FLAGS_ANYWHERE: core::ffi::c_int = 0x1;
pub const VM_FLAGS_OVERWRITE: core::ffi::c_int = 0x4000;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct vm_statistics {
    pub free_count: natural_t,
    pub active_count: natural_t,
    pub inactive_count: natural_t,
    pub wire_count: natural_t,
    pub zero_fill_count: natural_t,
    pub reactivations: natural_t,
    pub pageins: natural_t,
    pub pageouts: natural_t,
    pub faults: natural_t,
    pub cow_faults: natural_t,
    pub lookups: natural_t,
    pub hits: natural_t,
    pub purgeable_count: natural_t,
    pub purges: natural_t,
    pub speculative_count: natural_t,
}

#[cfg(feature = "deprecated")]
#[deprecated(
    since = "0.2.3",
    note = "`pmap_statistics` was removed after MacOSX 10.3.9"
)]
#[cfg_attr(feature = "deprecated", allow(deprecated))]
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Hash, PartialOrd, PartialEq, Eq, Ord)]
pub struct pmap_statistics {
    pub resident_count: integer_t,
    pub wired_count: integer_t,
}
