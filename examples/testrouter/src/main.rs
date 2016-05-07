#![cfg_attr(nightly, feature(custom_derive, plugin))]
#![cfg_attr(nightly, plugin(serde_macros))]

extern crate serde;
extern crate serde_json;
extern crate simtraining;
extern crate iron;
extern crate router;

#[cfg(feature = "serde_macros")]
include!("main.rs.in");

#[cfg(not(feature = "serde_macros"))]
include!(concat!(env!("OUT_DIR"), "/main.rs"));
