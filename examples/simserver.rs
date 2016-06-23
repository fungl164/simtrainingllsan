#![feature(drop_types_in_const)]
extern crate simtraining;
extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate router;
extern crate mount;
// use std::cell::RefCell;
// use std::rc::Rc;
use simtraining::route;
use iron::prelude::*;
use router::Router;
use simtraining::xitong::XiTong;
// use simtraining::simctrl;
// use simtraining::simctrl::{Timer, TimerHandler};
fn main() {
    let xt = XiTong::new(0);
    // let x = Rc::new(RefCell::new(xt));
    let mut router = Router::new();
    router.get("/", xt);
    let mut mount_root = route::build_route();
    mount_root.mount("/api/v1/xitong/", router);
    // Timer::new().start(&mut TimerHandler::new(x), simctrl::FANG_ZHEN_BU_CHANG as u64, simctrl::FANG_ZHEN_BU_CHANG as u64);
    Iron::new(mount_root).http("localhost:3000").unwrap();
}
