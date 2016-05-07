#![cfg_attr(feature = "serde_macros", feature(custom_derive, plugin))]
#![cfg_attr(feature = "serde_macros", plugin(serde_macros))]

extern crate serde;
extern crate serde_json;

extern crate time;
extern crate rand;
extern crate libc;
// extern crate libevent_sys;
extern crate mio;
#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate ws2_32;


#[cfg(feature = "serde_macros")]
include!("lib.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/lib.rs"));
