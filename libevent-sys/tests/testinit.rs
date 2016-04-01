extern crate libevent_sys;
#[cfg(target_family = "windows")]
extern crate winapi;
#[cfg(target_family = "windows")]
extern crate ws2_32;
use libevent_sys::event_base_new;
use libevent_sys::util::ws_init;

#[test]
pub fn test_event_init(){
    ws_init();
    unsafe{
        let base = libevent_sys::event_base_new();
        libevent_sys::event_base_dispatch(base);
        libevent_sys::event_base_free(base);
    }
}
