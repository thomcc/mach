//! This module corresponds to `bootstrap.h`

use super::boolean::boolean_t;
use super::kern_return::kern_return_t;
use super::port::mach_port_t;

pub const BOOTSTRAP_MAX_NAME_LEN: core::ffi::c_uint = 128;
pub const BOOTSTRAP_MAX_CMD_LEN: core::ffi::c_uint = 512;

pub const BOOTSTRAP_MAX_LOOKUP_COUNT: core::ffi::c_uint = 20;

pub const BOOTSTRAP_SUCCESS: core::ffi::c_uint = 0;
pub const BOOTSTRAP_NOT_PRIVILEGED: core::ffi::c_uint = 1100;
pub const BOOTSTRAP_NAME_IN_USE: core::ffi::c_uint = 1101;
pub const BOOTSTRAP_UNKNOWN_SERVICE: core::ffi::c_uint = 1102;
pub const BOOTSTRAP_SERVICE_ACTIVE: core::ffi::c_uint = 1103;
pub const BOOTSTRAP_BAD_COUNT: core::ffi::c_uint = 1104;
pub const BOOTSTRAP_NO_MEMORY: core::ffi::c_uint = 1105;
pub const BOOTSTRAP_NO_CHILDREN: core::ffi::c_uint = 1106;

pub const BOOTSTRAP_STATUS_INACTIVE: core::ffi::c_uint = 0;
pub const BOOTSTRAP_STATUS_ACTIVE: core::ffi::c_uint = 1;
pub const BOOTSTRAP_STATUS_ON_DEMAND: core::ffi::c_uint = 2;

pub type name_t = [core::ffi::c_char; 128];
pub type cmd_t = [core::ffi::c_char; 512];
pub type name_array_t = *mut name_t;
pub type bootstrap_status_t = core::ffi::c_int;
pub type bootstrap_status_array_t = *mut bootstrap_status_t;
pub type bootstrap_property_t = core::ffi::c_uint;
pub type bootstrap_property_array_t = *mut bootstrap_property_t;
pub type bool_array_t = *mut boolean_t;

extern "C" {
    pub static bootstrap_port: mach_port_t;
    pub fn bootstrap_create_server(
        bp: mach_port_t,
        server_cmd: *mut core::ffi::c_char,
        server_uid: libc::uid_t,
        on_demand: boolean_t,
        server_port: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn bootstrap_subset(
        bp: mach_port_t,
        requestor_port: mach_port_t,
        subset_port: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn bootstrap_unprivileged(bp: mach_port_t, unpriv_port: *mut mach_port_t) -> kern_return_t;
    pub fn bootstrap_parent(bp: mach_port_t, parent_port: *mut mach_port_t) -> kern_return_t;
    pub fn bootstrap_register(
        bp: mach_port_t,
        service_name: *mut core::ffi::c_char,
        sp: mach_port_t,
    ) -> kern_return_t;
    pub fn bootstrap_create_service(
        bp: mach_port_t,
        service_name: *mut core::ffi::c_char,
        sp: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn bootstrap_check_in(
        bp: mach_port_t,
        service_name: *const core::ffi::c_char,
        sp: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn bootstrap_look_up(
        bp: mach_port_t,
        service_name: *const core::ffi::c_char,
        sp: *mut mach_port_t,
    ) -> kern_return_t;
    pub fn bootstrap_status(
        bp: mach_port_t,
        service_name: *mut core::ffi::c_char,
        service_active: *mut bootstrap_status_t,
    ) -> kern_return_t;
    pub fn bootstrap_strerror(r: kern_return_t) -> *const core::ffi::c_char;
}
