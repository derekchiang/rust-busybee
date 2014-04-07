/* automatically generated by rust-bindgen */

use std::libc::*;
pub type int8_t = c_schar;
pub type int16_t = c_short;
pub type int32_t = c_int;
pub type int64_t = c_long;
pub type uint8_t = c_uchar;
pub type uint16_t = c_ushort;
pub type uint32_t = c_uint;
pub type uint64_t = c_ulong;
pub type int_least8_t = c_schar;
pub type int_least16_t = c_short;
pub type int_least32_t = c_int;
pub type int_least64_t = c_long;
pub type uint_least8_t = c_uchar;
pub type uint_least16_t = c_ushort;
pub type uint_least32_t = c_uint;
pub type uint_least64_t = c_ulong;
pub type int_fast8_t = c_schar;
pub type int_fast16_t = c_long;
pub type int_fast32_t = c_long;
pub type int_fast64_t = c_long;
pub type uint_fast8_t = c_uchar;
pub type uint_fast16_t = c_ulong;
pub type uint_fast32_t = c_ulong;
pub type uint_fast64_t = c_ulong;
pub type intptr_t = c_long;
pub type uintptr_t = c_ulong;
pub type intmax_t = c_long;
pub type uintmax_t = c_ulong;
pub type ptrdiff_t = c_long;
pub type size_t = c_ulong;
pub type wchar_t = c_int;
pub type Struct_busybee_mapper = c_void;
pub type busybee_mapper = Struct_busybee_mapper;
pub type lookup_func_t =
    ::std::option::Option<extern "C" fn
                              (arg1: *mut c_void, arg2: uint64_t,
                               arg3: *mut *c_schar, arg4: *mut uint16_t)
                              -> c_int>;
pub type Enum_busybee_returncode = c_uint;
pub static BUSYBEE_SUCCESS: c_uint = 4608;
pub static BUSYBEE_SHUTDOWN: c_uint = 4609;
pub static BUSYBEE_POLLFAILED: c_uint = 4610;
pub static BUSYBEE_DISRUPTED: c_uint = 4611;
pub static BUSYBEE_ADDFDFAIL: c_uint = 4612;
pub static BUSYBEE_TIMEOUT: c_uint = 4613;
pub static BUSYBEE_EXTERNAL: c_uint = 4614;
pub static BUSYBEE_INTERRUPTED: c_uint = 4615;
pub type Struct_busybee_mta = c_void;
pub type busybee_mta = Struct_busybee_mta;
pub type busybee_returncode = Enum_busybee_returncode;

#[link_args = "-I/usr/local/include  -L/usr/local/lib"]
#[link(name = "busybee")]
#[link(name = "e")]
extern "C" {
    pub fn busybee_mapper_create(user_data: *mut c_void,
                                 lookup: lookup_func_t) ->
     *mut busybee_mapper;
    pub fn busybee_mta_create(mapper: *mut busybee_mapper, address: *c_schar,
                              port: uint16_t, server_id: uint64_t,
                              num_threads: size_t) -> *mut busybee_mta;
    pub fn busybee_mta_delete(bb: *mut busybee_mta);
    pub fn busybee_mta_shutdown(bb: *mut busybee_mta);
    pub fn busybee_mta_pause(bb: *mut busybee_mta);
    pub fn busybee_mta_unpause(bb: *mut busybee_mta);
    pub fn busybee_mta_wake_one(bb: *mut busybee_mta);
    pub fn busybee_mta_set_id(bb: *mut busybee_mta, server_id: uint64_t);
    pub fn busybee_mta_set_timeout(bb: *mut busybee_mta, timeout: c_int);
    pub fn busybee_mta_set_ignore_signals(bb: *mut busybee_mta);
    pub fn busybee_mta_unset_ignore_signals(bb: *mut busybee_mta);
    pub fn busybee_mta_add_signals(bb: *mut busybee_mta);
    pub fn busybee_mta_get_addr(bb: *mut busybee_mta, server_id: uint64_t,
                                address: *mut *c_schar, port: *mut uint16_t)
     -> busybee_returncode;
    pub fn busybee_mta_deliver(bb: *mut busybee_mta, server_id: uint64_t,
                               msg: *c_schar, msg_sz: size_t) -> c_int;
    pub fn busybee_mta_poll_fd(bb: *mut busybee_mta) -> c_int;
    pub fn busybee_mta_drop(bb: *mut busybee_mta, server_id: uint64_t) ->
     busybee_returncode;
    pub fn busybee_mta_send(bb: *mut busybee_mta, server_id: uint64_t,
                            msg: *c_schar, msg_sz: size_t) ->
     busybee_returncode;
    pub fn busybee_mta_recv(bb: *mut busybee_mta, server_id: *mut uint64_t,
                            msg: *mut *c_schar, msg_sz: *mut size_t) ->
     busybee_returncode;
}