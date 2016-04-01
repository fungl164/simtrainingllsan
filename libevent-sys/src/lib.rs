#![allow(non_camel_case_types)] #![allow(non_upper_case_globals)]

extern crate libc;
#[cfg(target_family = "windows")]
extern crate winapi;
pub mod util;
#[cfg(target_family = "windows")]
use winapi::winsock2::timeval;
#[cfg(target_family = "unix")]
use libc::timeval;

use libc::{c_uint, c_int, c_short, c_void, c_char, FILE, size_t};
pub const EVENT__NUMERIC_VERSION:u32 = 0x02010500u32;
pub const EVENT__PACKAGE_VERSION:&'static str = "2.1.5";
pub const EVENT__VERSION:&'static str = "2.1.5-alpha-beta";
pub const EVENT__PACKAGE:&'static str = "libevent";
pub const EVENT__PACKAGE_BUGREPORT:&'static str = "";
pub const EVENT__PACKAGE_NAME:&'static str = "";
pub const EVENT__PACKAGE_STRING:&'static str = "";
pub const EVENT__PACKAGE_TARNAME:&'static str = "";
pub const EVENT__DNS_USE_FTIME_FOR_ID:u32 = 1;
pub const EVENT__HAVE_FCNTL_H:u32 = 1;
pub const EVENT__HAVE_GETADDRINFO:u32 = 1;
pub const EVENT__HAVE_GETNAMEINFO:u32 = 1;
pub const EVENT__HAVE_GETPROTOBYNUMBER:u32 = 1;
pub const EVENT__HAVE_GETSERVBYNAME:u32 = 1;
pub const EVENT__HAVE_INTTYPES_H:u32 = 1;
pub const EVENT__HAVE_MEMORY_H:u32 = 1;
pub const EVENT__HAVE_OPENSSL:u32 = 1;
pub const EVENT__HAVE_PUTENV:u32 = 1;
pub const EVENT__HAVE_STDARG_H:u32 = 1;
pub const EVENT__HAVE_STDDEF_H:u32 = 1;
pub const EVENT__HAVE_STDINT_H:u32 = 1;
pub const EVENT__HAVE_STDLIB_H:u32 = 1;
pub const EVENT__HAVE_STRING_H:u32 = 1;
pub const EVENT__HAVE_STRTOLL:u32 = 1;
pub const EVENT__HAVE_STRUCT_ADDRINFO:u32 = 1;
pub const EVENT__HAVE_STRUCT_IN6_ADDR:u32 = 1;
pub const EVENT__HAVE_STRUCT_SOCKADDR_IN6:u32 = 1;
pub const EVENT__HAVE_STRUCT_SOCKADDR_STORAGE:u32 = 1;
pub const EVENT__HAVE_STRUCT_SOCKADDR_STORAGE_SS_FAMILY:u32 = 1;
pub const EVENT__HAVE_SYS_STAT_H:u32 = 1;
pub const EVENT__HAVE_SYS_TYPES_H:u32 = 1;
pub const EVENT__HAVE_UINT8_T:u32 = 1;
pub const EVENT__HAVE_UINT16_T:u32 = 1;
pub const EVENT__HAVE_UINT32_T:u32 = 1;
pub const EVENT__HAVE_UINT64_T:u32 = 1;
pub const EVENT__HAVE_UINTPTR_T:u32 = 1;
pub const EVENT__SIZEOF_INT:u32 = 4;
pub const EVENT__SIZEOF_LONG:u32 = 4;
pub const EVENT__SIZEOF_LONG_LONG:u32 = 8;
pub const EVENT__SIZEOF_OFF_T:u32 = 4;
pub const EVENT__SIZEOF_SHORT:u32 = 2;
pub const EVENT__SIZEOF_SIZE_T:u32 = 8;
pub const EVENT__SIZEOF_SOCKLEN_T:u32 = 4;
pub const EVENT__SIZEOF_VOID_P:u32 = 8;
pub type EVENT__pid_t = libc::c_int;
pub type EVENT__ssize_t = libc::ssize_t;
pub type event_base = c_void;
pub type event = c_void;
pub type event_config = c_void;
pub const EVENT_BASE_COUNT_ACTIVE:u32 = 1u32;
pub const EVENT_BASE_COUNT_VIRTUAL:u32 = 2u32;
pub const EVENT_BASE_COUNT_ADDED:u32 = 4u32;
pub enum event_method_feature {
    EV_FEATURE_ET = 0x01,
    EV_FEATURE_O1 = 0x02,
    EV_FEATURE_FDS = 0x04,
    EV_FEATURE_EARLY_CLOSE = 0x08,
}
pub enum event_base_config_flag {
	EVENT_BASE_FLAG_NOLOCK = 0x01,
	EVENT_BASE_FLAG_IGNORE_ENV = 0x02,
	EVENT_BASE_FLAG_STARTUP_IOCP = 0x04,
	EVENT_BASE_FLAG_NO_CACHE_TIME = 0x08,
	EVENT_BASE_FLAG_EPOLL_USE_CHANGELIST = 0x10,
	EVENT_BASE_FLAG_PRECISE_TIMER = 0x20,
}
pub const EVENT_LOG_DEBUG : u32 = 0u32;
pub const EVENT_LOG_MSG : u32 =   1u32;
pub const EVENT_LOG_WARN : u32 =  2u32;
pub const EVENT_LOG_ERR  : u32 =  3u32;
pub const _EVENT_LOG_DEBUGu32 : u32 = EVENT_LOG_DEBUG;
pub const _EVENT_LOG_MSG : u32 = EVENT_LOG_MSG;
pub const _EVENT_LOG_WARN : u32 = EVENT_LOG_WARN;
pub const _EVENT_LOG_ERR : u32 = EVENT_LOG_ERR;
pub type event_log_cb = extern fn(severity: c_int, msg: *const c_char);
pub type event_fatal_cb = extern fn(err : c_int);
pub const EVENT_DBG_ALL : u32 = 0xffffffffu32;
pub const EVENT_DBG_NONE : u32 = 0;
pub const EVLOOP_ONCE: u32 = 0x01;
pub const EVLOOP_NONBLOCK: u32 = 0x02;
pub const EVLOOP_NO_EXIT_ON_EMPTY: u32 = 0x04;
pub const EV_TIMEOUT : i16 = 0x01;
pub const EV_READ : i16 = 0x02;
pub const EV_WRITE : i16 = 0x04;
pub const EV_SIGNAL : i16 = 0x08;
pub const EV_PERSIST : i16 = 0x10;
pub const EV_ET : i16 = 0x20;
pub const EV_FINALIZE : i16 = 0x40;
pub const EV_CLOSED : i16 = 0x80;
pub type evutil_socket_t = libc::intptr_t;
pub type event_callback_fn = extern fn(evutil_socket_t, c_short, * mut c_void);
pub type event_finalize_callback_fn = extern fn(* mut event, * mut c_void);
pub type ev_uint32_t = libc::uint32_t;
pub const LIBEVENT_VERSION : &'static str = EVENT__VERSION;
pub const LIBEVENT_VERSION_NUMBER : u32 = EVENT__NUMERIC_VERSION;
pub const EVENT_MAX_PRIORITIES : u32 = 256;
pub type event_base_foreach_event_cb = extern fn(* mut event_base, * mut event, * mut c_void)->c_int;

//#[link(name = "event_core", kind = "static")]
extern {
    pub fn event_enable_debug_mode();
    pub fn event_debug_unassign(_:* mut event);
    pub fn event_base_new() -> * mut event_base;
    pub fn event_reinit(_:* mut event_base) -> c_int;
    pub fn event_base_dispatch(_:* mut event_base) -> c_int;
    pub fn event_base_get_method(_:* const event_base) ->* const c_char;
    pub fn event_get_supported_methods() -> * mut * const c_char;
    pub fn event_gettime_monotonic(_:* mut event_base, _:* mut timeval) -> c_int;
    pub fn event_base_get_num_events(_:* mut event_base, _:c_uint) -> c_int;
    pub fn event_base_get_max_events(_:* mut event_base, _:c_uint, _:c_int) -> c_int;
    pub fn event_config_new() -> * mut event_config;
    pub fn event_config_free(_:* mut event_config);
    pub fn event_config_avoid_method(_:*mut event_config, _:*const c_char) -> c_int;
    pub fn event_base_get_features(_:* const event_base) -> c_int;
    pub fn event_config_require_features(_:* mut event_config, _:c_int) -> c_int;
    pub fn event_config_set_flag(_:* mut event_config, _:c_int) -> c_int;
    pub fn event_config_set_num_cpus_hint(_:* mut event_config, _:c_int) -> c_int;
    pub fn event_config_set_max_dispatch_interval(
        _:*mut event_config, _:*const timeval, _:c_int, _:c_int) -> c_int;
    pub fn event_base_new_with_config(_:*const event_config) -> *mut event_base;
    pub fn event_base_free(_:* mut event_base);
    pub fn event_base_free_nofinalize(_:* mut event_base);
    pub fn event_set_log_callback(_:event_log_cb);
    pub fn event_set_fatal_callback(_:event_fatal_cb);
    pub fn event_enable_debug_logging(_:ev_uint32_t);
    pub fn event_base_set(_:* mut event_base, _:* mut event)->c_int;
    pub fn event_base_loop(_:* mut event_base)->c_int;
    pub fn event_base_loopexit(_:* mut event_base, _:* const timeval)->c_int;
    pub fn event_base_loopbreak(_:* mut event_base)->c_int;
    pub fn event_base_loopcontinue(_:* mut event_base)->c_int;
    pub fn event_base_got_exit(_:* mut event_base)->c_int;
    pub fn event_base_got_break(_:* mut event_base)->c_int;
    pub fn event_self_cbarg()->* mut c_void;
    pub fn event_new(_:* mut event_base, _:evutil_socket_t,
        _:c_short, _:event_callback_fn, _:* mut c_void) -> * mut event;
    pub fn event_assign(_:* mut event, _:* mut event_base, _:evutil_socket_t,
        _:c_short, _:event_callback_fn, _:* mut c_void) -> c_int;
    pub fn event_free(_:* mut event);
    pub fn event_finalize(_:c_uint, _:* mut event, _:event_finalize_callback_fn) -> c_int;
    pub fn event_free_finalize(_:c_uint, _:* mut event, _:event_finalize_callback_fn) -> c_int;
    pub fn event_base_once(_:* mut event_base, _:evutil_socket_t, _:c_short,
        _:event_callback_fn, _:* mut c_void, _:* const timeval) -> c_int;
    pub fn event_add(ev : * mut event, timeout : * const timeval) -> c_int;
    pub fn event_remove_timer(_:* mut event) -> c_int;
    pub fn event_del(_:* mut event) -> c_int;
    pub fn event_del_noblock(_:* mut event) -> c_int;
    pub fn event_del_block(_:* mut event) -> c_int;
    pub fn event_active(_:* mut event, _:c_int, _:c_short);
    pub fn event_pending(_:* const event, _:c_short, _:* mut timeval)-> c_int;
    pub fn event_base_get_running_event(_:* mut event_base)-> * mut event;
    pub fn event_initialized(_:* const event)-> c_int;
    pub fn event_get_fd(_:* const event)-> evutil_socket_t;
    pub fn event_get_base(_:* const event)-> * mut event_base;
    pub fn event_get_events(_:* const event)-> c_short;
    pub fn event_get_callback(_:* const event)-> event_callback_fn;
    pub fn event_get_callback_arg(_:* const event)-> * mut c_void;
    pub fn event_get_priority(_:* const event)-> c_int;
    pub fn event_get_assignment(_:* const event, _:*mut * mut event_base,
        _:* mut evutil_socket_t, _:* mut c_short,
        _:event_callback_fn, _:*mut * mut c_void);
    pub fn event_get_struct_event_size()-> size_t;
    pub fn event_get_version()-> * const c_char;
    pub fn event_get_version_number()-> ev_uint32_t;
    pub fn event_base_priority_init(_:* mut event_base, _:c_int)-> c_int;
    pub fn event_base_get_npriorities(_:* mut event_base)-> c_int;
    pub fn event_priority_set(_:* mut event, _:c_int)-> c_int;
    pub fn event_base_init_common_timeout(_:* mut event_base,
        _:* const timeval)-> *const timeval;
    pub fn event_set_mem_functions(_:* mut extern fn(size_t),
        _:* mut extern fn(* mut c_void, size_t),
        _:extern fn(* mut c_void));
    pub fn event_base_dump_events(_:* mut event_base, _:* mut FILE);
    pub fn event_base_active_by_fd(_:* mut event_base, _:evutil_socket_t, _:c_short);
    pub fn event_base_active_by_signal(_:* mut event_base, _:c_int);
    pub fn event_base_foreach_event(_:* mut event_base, _:event_base_foreach_event_cb, _:* mut c_void)->c_int;
    pub fn event_base_gettimeofday_cached(_:* mut event_base, _:* mut timeval)-> c_int;
    pub fn event_base_update_cache_time(_:* mut event_base)-> c_int;
    pub fn libevent_global_shutdown();
    pub fn event_init()->* mut event_base;
    pub fn event_dispatch()->libc::c_int;
    pub fn event_loop(_ : libc::c_int)->libc::c_int;
    pub fn event_loopexit(_:* const timeval)->libc::c_int;
    pub fn event_loopbreak()->libc::c_int;
    pub fn event_once(_:evutil_socket_t, _:c_short, _:event_callback_fn, _:* mut c_void, _:* const timeval)->libc::c_int;
    pub fn event_get_method()->* const c_char;
    pub fn event_priority_init()->c_int;
    pub fn event_set(_ : * mut event, _ : evutil_socket_t, _ : c_short, _ : event_callback_fn, _ : * mut c_void);
}
pub fn evtimer_set(ev : * mut event, cb : event_callback_fn, arg : * mut c_void){
    unsafe{
        event_set(ev, -1, 0, cb, arg);
    }
}
pub fn evsignal_set(ev : * mut event, x : evutil_socket_t, cb : event_callback_fn, arg : * mut c_void){
    unsafe{
        event_set(ev, x, EV_SIGNAL|EV_PERSIST, cb, arg);
    }
}

pub fn evtimer_assign(ev:* mut event, b:* mut event_base, cb:event_callback_fn, arg:* mut c_void)->c_int{
    unsafe{
        event_assign(ev, b, -1, 0, cb, arg)
    }
}
pub fn evtimer_new(b:* mut event_base, cb:event_callback_fn, arg:* mut c_void)->*mut event{
    unsafe{
        event_new(b, -1, 0, cb, arg)
    }
}
pub fn evtimer_add(ev : * mut event, tv : * const timeval)->c_int{
    unsafe{
        event_add(ev, tv)
    }
}


pub fn evtimer_del(ev:* mut event)->c_int{
    unsafe{
        event_del(ev)
    }
}

pub fn evtimer_pending(ev:* const event, tv:* mut timeval)->c_int{
    unsafe{
        event_pending(ev, EV_TIMEOUT, tv)
    }
}

pub fn evtimer_initialized(ev:* const event)->c_int{
    unsafe{
        event_initialized(ev)
    }
}

pub fn evsignal_add(ev : * mut event, tv : * const timeval)->c_int{
    unsafe{
        event_add(ev, tv)
    }
}

pub fn evsignal_assign(ev:* mut event, b:* mut event_base, x:evutil_socket_t,  cb:event_callback_fn, arg:* mut c_void)->c_int{
    unsafe{
        event_assign(ev, b, x, EV_SIGNAL|EV_PERSIST, cb, arg)
    }
}

pub fn evsignal_new(b:* mut event_base, x:evutil_socket_t, cb:event_callback_fn, arg:* mut c_void)->*mut event{
    unsafe{
        event_new(b, x, EV_SIGNAL|EV_PERSIST, cb, arg)
    }
}

pub fn evsignal_del(ev:* mut event)->c_int{
    unsafe{
        event_del(ev)
    }
}

pub fn evsignal_pending(ev:* const event, tv:* mut timeval)->c_int{
    unsafe{
        event_pending(ev, EV_SIGNAL, tv)
    }
}

pub fn evsignal_initialized(ev:* const event)->c_int{
    unsafe{
        event_initialized(ev)
    }
}

pub fn evutil_timerclear(tv : &mut timeval){
	tv.tv_sec = 0;
    tv.tv_usec = 0;
}
