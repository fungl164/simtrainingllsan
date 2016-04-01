extern crate libc;
extern crate libevent_sys;
extern crate rand;
#[cfg(target_family = "windows")]
extern crate winapi;
#[cfg(target_family = "windows")]
extern crate ws2_32;
#[cfg(target_family = "windows")]
use winapi::winsock2::timeval;
#[cfg(unix)]
use libc::timeval;
use rand::Rng;
use libevent_sys::{event, evutil_socket_t};
use libevent_sys::util::ws_init;

static mut called : i32 = 0;
const NEVENT : usize = 20;

extern fn time_cb(fd : evutil_socket_t, what : libc::c_short, arg : * mut libc::c_void)
{
	unsafe{
		called += 1;
		let ev = arg as *mut event;
		//assert_eq!(-1, fd);
		assert_eq!(1, what);
		println!("{}", fd);
		libevent_sys::event_del(ev);
		libevent_sys::event_free(ev);
	}
}

#[test]
pub fn test_time()
{
	let mut tv : timeval = timeval{
		tv_sec : 0,
		tv_usec : 0,
	};
	let mut rng = rand::thread_rng();
	ws_init();
	unsafe{
		let base = libevent_sys::event_base_new();
		for i in 0..NEVENT {
			let ev = libevent_sys::event_new(base, -1, 0, time_cb, libevent_sys::event_self_cbarg());
			tv.tv_sec = (i - i) as i64;
			tv.tv_usec = rng.gen_range(0, 50000-1);
			libevent_sys::event_add(ev, &tv);
		}

		libevent_sys::event_base_dispatch(base);
		println!("{}, {}", called, NEVENT);
		libevent_sys::event_base_free(base);
	}
}
