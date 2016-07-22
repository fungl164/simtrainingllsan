extern crate simtraining;
extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate router;
extern crate mount;
// use std::cell::RefCell;
// use std::rc::Rc;
use simtraining::zhiling::{ZhiLing, ZhiLingType};
use simtraining::simctrl;
use std::thread;
use simtraining::route;
use iron::prelude::*;
use router::Router;
use simtraining::xitong::{XiTongThread, ZhiLingHandler};
// use simtraining::simctrl;
// use simtraining::simctrl::{Timer, TimerHandler};
fn main() {
    let mut flow = vec![ZhiLing::from_params(ZhiLingType::BeiChe, simctrl::DevType::JiZu, 0, 0, 0, simctrl::ZhanWeiType::Internal); 17];

    flow[16].zhi_ling_type = ZhiLingType::OperatingStation(simctrl::OperatingStation::JiKong);
    flow[16].dev_type = simctrl::DevType::DianZhan;

    flow[15].zhi_ling_type = ZhiLingType::CtrlMode(simctrl::CtrlMode::SemiAuto);
    flow[15].dev_type = simctrl::DevType::DianZhan;

    flow[14].zhi_ling_type = ZhiLingType::OperatingStation(simctrl::OperatingStation::JiKong);
    flow[14].dev_type = simctrl::DevType::DianZhan;
    flow[14].dev_id = 1;

    flow[13].zhi_ling_type = ZhiLingType::CtrlMode(simctrl::CtrlMode::SemiAuto);
    flow[13].dev_type = simctrl::DevType::DianZhan;
    flow[13].dev_id = 1;

    flow[12].zhi_ling_type = ZhiLingType::OperatingStation(simctrl::OperatingStation::JiKong);
    flow[12].dev_type = simctrl::DevType::DianZhan;
    flow[12].dev_id = 2;

    flow[11].zhi_ling_type = ZhiLingType::CtrlMode(simctrl::CtrlMode::SemiAuto);
    flow[11].dev_type = simctrl::DevType::DianZhan;
    flow[11].dev_id = 2;

    flow[10].zhi_ling_type = ZhiLingType::OperatingStation(simctrl::OperatingStation::JiKong);
    flow[10].dev_type = simctrl::DevType::DianZhan;
    flow[10].dev_id = 3;

    flow[9].zhi_ling_type = ZhiLingType::CtrlMode(simctrl::CtrlMode::SemiAuto);
    flow[9].dev_type = simctrl::DevType::DianZhan;
    flow[9].dev_id = 3;

    //机组并联：电站0/1（控制方式半自动---操作部位集控）---机组0（备车--启动--合闸）--负载0（加载）--断路器2/4/32/33（合闸）---机组3（备车--启动--合闸）---断路器9并车----机组3(解列---停机）---断路器2解列---机组0（分闸--停机）
    for i in 0..(simctrl::ZONG_SHU_JI_ZU_CHAI_YOU + simctrl::ZONG_SHU_JI_ZU_QI_LUN) {
        flow[i].dev_id = i;
    }

    let xt = XiTongThread::new(0, flow);
    // let xt = XiTongThread::new(0, Vec::new());
    let zl = ZhiLingHandler { xt : xt.xt.clone() };
    let mut router_xt = Router::new();
    router_xt.get("/", xt);
    let mut router_zl = Router::new();
    router_zl.post("/", zl);
    router_zl.get("/example/generateyibanguzhang", ZhiLing::zhi_ling_example_generate_yi_ban_gu_zhang_handler);
    router_zl.get("/example/eliminateyibanguzhang", ZhiLing::zhi_ling_example_eliminate_yi_ban_gu_zhang_handler);
    router_zl.get("/example/operatingstation", ZhiLing::zhi_ling_example_operating_station_handler);
    router_zl.get("/example/ctrlmode", ZhiLing::zhi_ling_example_ctrl_mode_handler);
    router_zl.get("/example/prio", ZhiLing::zhi_ling_example_prio_handler);
    let mut mount_root = route::build_route();
    mount_root.mount("/api/v1/xitong/", router_xt);
    mount_root.mount("/api/v1/zhiling/", router_zl);
    println!("服务器启动成功, 正在运行中......");
    let th = thread::spawn(move || {
        Iron::new(mount_root).http("0.0.0.0:3000").unwrap();
    });
    th.join().unwrap();

}
// fn main() {
//     use simtraining::xitong::XiTong;
//     let xt = XiTong::new(0);
//     let mut router_xt = Router::new();
//     router_xt.get("/", xt);
//     let mut mount_root = route::build_route();
//     mount_root.mount("/api/v1/xitong/", router_xt);
//     // Timer::new().start(&mut TimerHandler::new(x), simctrl::FANG_ZHEN_BU_CHANG as u64, simctrl::FANG_ZHEN_BU_CHANG as u64);
//     let th = thread::spawn(move || {
//         Iron::new(mount_root).http("localhost:3000").unwrap();
//     });
//     th.join().unwrap();
//
// }
