#![feature(drop_types_in_const)]
extern crate simtraining;
extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate router;
extern crate mount;
// use std::cell::RefCell;
// use std::rc::Rc;
use std::thread;
use simtraining::route;
use iron::prelude::*;
use router::Router;
use simtraining::xitong::{XiTongThread, ZhiLingHandler};
use simtraining::zhiling::ZhiLing;
// use simtraining::simctrl;
// use simtraining::simctrl::{Timer, TimerHandler};
fn main() {
    let xt = XiTongThread::new(0);
    let zl = ZhiLingHandler { xt : xt.xt.clone() };
    // let x = Rc::new(RefCell::new(xt));
    let mut router_xt = Router::new();
    router_xt.get("/", xt);
    let mut router_zl = Router::new();
    router_zl.post("/", zl);
    router_zl.get("/example", ZhiLing::zhi_ling_example_handler);
    let mut mount_root = route::build_route();
    mount_root.mount("/api/v1/xitong/", router_xt);
    mount_root.mount("/api/v1/zhiling/", router_zl);
    // Timer::new().start(&mut TimerHandler::new(x), simctrl::FANG_ZHEN_BU_CHANG as u64, simctrl::FANG_ZHEN_BU_CHANG as u64);
    let th = thread::spawn(move || {
        Iron::new(mount_root).http("localhost:3000").unwrap();
    });
    th.join().unwrap();

}
