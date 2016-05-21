#![feature(custom_derive, custom_attribute, plugin)]
#![plugin(serde_macros, diesel_codegen, dotenv_macros)]

#[macro_use]
extern crate diesel;
extern crate dotenv;

extern crate serde;
extern crate serde_json;

extern crate time;
extern crate rand;
extern crate libc;
extern crate mio;

extern crate iron;
extern crate router;
extern crate mount;
extern crate staticfile;

#[cfg(windows)]
extern crate winapi;
#[cfg(windows)]
extern crate ws2_32;

pub mod jizu;
pub mod simctrl;
pub mod util;
pub mod zhiling;
pub mod dianzhan;
pub mod duanluqi;
pub mod fuzai;
pub mod node;
pub mod zhilu;
pub mod xitong;

pub mod schema;
pub mod user;
pub mod trainingsession;
// pub mod trainingaction;
// pub mod dev;
pub mod zhanwei;

pub mod route;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
