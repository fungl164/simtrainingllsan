extern crate libc;
extern crate libevent_sys;
extern crate time;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate ws2_32;
#[cfg(windows)]
use winapi::winsock2::timeval;
#[cfg(unix)]
use libc::timeval;
use libevent_sys::evutil_socket_t;
use libevent_sys::util::ws_init;

extern fn timeout_cb(_fd : evutil_socket_t, _what : libc::c_short, arg : * mut libc::c_void) {
	unsafe{
		let mut lasttime = arg as * mut time::Timespec;
		let newtime = time::get_time();
		let difference = newtime - *lasttime;
		println!("last time is at {}.{}", (*lasttime).sec, (*lasttime).nsec);
		println!("timeout_cb called at {}.", newtime.sec);
		println!("{} nanoseconds elapsed.\n", difference);
		*lasttime = newtime;
	}
}

fn main() {
	ws_init();
	let mut tv : timeval = timeval{
		tv_sec : 0,
		tv_usec : 0,
	};
	let flags = libevent_sys::EV_PERSIST;
	unsafe{
		let base = libevent_sys::event_base_new();
		libevent_sys ::	evutil_timerclear(&mut tv);
		tv.tv_usec = 500000;
		let mut lasttime = time::get_time();
		let timeout = libevent_sys::event_new(base, -1, flags, timeout_cb, &mut lasttime as *mut time::Timespec as *mut libc::c_void);
		libevent_sys::event_add(timeout, &tv);
		libevent_sys::event_base_dispatch(base);
		libevent_sys::event_base_free(base);
	}
}
