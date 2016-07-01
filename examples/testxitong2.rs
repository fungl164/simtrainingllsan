extern crate simtraining;
extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate router;
extern crate mount;
extern crate time;
use simtraining::xitong::XiTong;
use simtraining::zhiling::{ZhiLing, ZhiLingType};
use simtraining::simctrl;
use simtraining::route;

use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use router::Router;
use std::thread;
use std::sync::Arc;
use std::sync::RwLock;
use std::time::Duration;
use simtraining::jizu::{JiZuCtrl};

fn main() {
    let mut flow = vec![ZhiLing::from_params(ZhiLingType::BeiChe, simctrl::DevType::JiZu, 0, 0, 0, simctrl::ZhanWeiType::JiaoLian); 7];
    let mut flow2 = vec![ZhiLing::from_params(ZhiLingType::BeiChe, simctrl::DevType::JiZu, 1, 0, 0, simctrl::ZhanWeiType::JiaoLian); 7];
    flow.append(&mut flow2);
    flow[13].zhi_ling_type = ZhiLingType::BeiChe;
    flow[12].zhi_ling_type = ZhiLingType::QiDong;
    flow[11].zhi_ling_type = ZhiLingType::HeZhaBingChe;
    flow[10].zhi_ling_type = ZhiLingType::BianZai(550.0, 550.0);
    flow[10].dev_type = simctrl::DevType::FuZai;
    flow[10].dev_id = 3;
    flow[9].zhi_ling_type = ZhiLingType::BianZai(-150.0, -150.0);
    flow[9].dev_type = simctrl::DevType::FuZai;
    flow[9].dev_id = 3;
    flow[8].zhi_ling_type = ZhiLingType::FenZhaJieLie;
    flow[7].zhi_ling_type = ZhiLingType::TingJi;
    flow[6].zhi_ling_type = ZhiLingType::BeiChe;
    flow[5].zhi_ling_type = ZhiLingType::QiDong;
    flow[4].zhi_ling_type = ZhiLingType::HeZhaBingChe;
    flow[3].zhi_ling_type = ZhiLingType::BianZai(550.0, 550.0);
    flow[3].dev_type = simctrl::DevType::FuZai;
    flow[2].zhi_ling_type = ZhiLingType::BianZai(-150.0, -150.0);
    flow[2].dev_type = simctrl::DevType::FuZai;
    flow[1].zhi_ling_type = ZhiLingType::FenZhaJieLie;
    flow[0].zhi_ling_type = ZhiLingType::TingJi;
    let xt = XiTongThread::new(0, flow);
    let zl = ZhiLingHandler { xt : xt.xt.clone() };
    let mut router_xt = Router::new();
    router_xt.get("/", xt);
    let mut router_zl = Router::new();
    router_zl.post("/", zl);
    router_zl.get("/example", ZhiLing::zhi_ling_example_handler);
    router_zl.get("/examplepretty", ZhiLing::zhi_ling_example_handler);
    let mut mount_root = route::build_route();
    mount_root.mount("/api/v1/xitong/", router_xt);
    mount_root.mount("/api/v1/zhiling/", router_zl);
    let th = thread::spawn(move || {
        Iron::new(mount_root).http("0.0.0.0:3000").unwrap();
    });
    th.join().unwrap();
}

pub struct ZhiLingHandler {
    pub xt : Arc<RwLock<XiTong>>,
}
impl iron::middleware::Handler for ZhiLingHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        use std::ops::DerefMut;
        use std::io::prelude::*;
        let xt_shared = self.xt.clone();
        let mut xt_raw = xt_shared.write().unwrap();

        let mut data_json = String::new();
        let content_type_ok = "application/json".parse::<Mime>().unwrap();
        let content_type_err = "text/plain".parse::<Mime>().unwrap();
        match req.body.read_to_string(&mut data_json) {
            Ok(_) => {
                match serde_json::from_str::<ZhiLing>(&data_json) {
                    Ok(data) => {
                        xt_raw.deref_mut().handle_zhi_ling(&data);
                        return Ok(Response::with((content_type_ok,
                                                  status::Ok,
                                                  data_json)));
                    }
                    Err(e) => {
                        return Ok(Response::with((content_type_err,
                                                  status::BadRequest,
                                                  format!("{:?}", e))))
                    }
                }
            }
            Err(e) => {
                return Ok(Response::with((content_type_err,
                                          status::BadRequest,
                                          format!("{:?}", e))))
            }
        }
    }
}

pub struct XiTongThread {
    pub xt : Arc<RwLock<XiTong>>,
    pub flow : Arc<RwLock<Vec<ZhiLing>>>,
}
impl iron::middleware::Handler for XiTongThread {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        use simtraining::jsoninf::XiTongInf;
        use std::ops::Deref;
        let xt_shared = self.xt.clone();
        let xt_raw = xt_shared.read().unwrap();
        let x_ser = serde_json::to_string(&(XiTongInf::from_ob(xt_raw.deref()))).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, x_ser)))
    }
}
impl XiTongThread {
    pub fn new(id:usize, flow : Vec<ZhiLing>) -> XiTongThread {
        let mut x = XiTongThread {
            xt : Arc::new(RwLock::new(XiTong::new(id))),
            flow : Arc::new(RwLock::new(flow)),
        };
        x.update();
        x
    }
    pub fn update(&mut self) {
        use std::ops::DerefMut;
        use simtraining::zhiling::Condition;
        let xt_shared = self.xt.clone();
        let flow_share = self.flow.clone();
        let _ = thread::spawn(move || {
            let mut tick = 0u32;
            loop{
                {
                    let mut xt_raw = xt_shared.write().unwrap();
                    for i in 0..simctrl::ZONG_SHU_JI_ZU {
                        xt_raw.ji_zu_vec[i].update();
                    }
                    xt_raw.compute_xi_tong_pf();
                    xt_raw.p_shou_ti_dui = 0.0;
                    xt_raw.u_shou_ti_dui = 0.0;
                    xt_raw.f_shou_ti_dui = 0.0;
                    xt_raw.pd_shou_ti_dui = 0.0;

                    xt_raw.p_wei_ti_dui = 0.0;
                    xt_raw.u_wei_ti_dui = 0.0;
                    xt_raw.f_wei_ti_dui = 0.0;
                    xt_raw.pd_wei_ti_dui = 0.0;

                    xt_raw.p_quan_jian = 0.0;
                    for jizuid in 0..simctrl::ZONG_SHU_JI_ZU {
                        xt_raw.p_quan_jian += xt_raw.ji_zu_vec[jizuid].common_ji.p;
                    }
                    xt_raw.sec = time::get_time().sec;
                    xt_raw.nsec = time::get_time().nsec;

                    let mut flow_raw = flow_share.write().unwrap();
                    if !flow_raw.is_empty() && flow_raw.last().unwrap().can_exec(xt_raw.deref_mut()){
                        xt_raw.handle_zhi_ling(&(flow_raw.pop().unwrap()));
                    }

                    //以下为测试内容，实际运行时请注释
                    // println!("{:?}.{:?}", xt_raw.sec, xt_raw.nsec);
                    // for i in 0..simctrl::ZONG_SHU_JI_ZU {
                    //
                    //     match xt_raw.ji_zu_vec[i].common_ji.current_range {
                    //         JiZuRangeLeiXing::TingJi | JiZuRangeLeiXing::JinJiGuZhang => xt_raw.ji_zu_vec[i].common_ji.current_range = JiZuRangeLeiXing::BeiCheZanTai,
                    //         JiZuRangeLeiXing::BeiCheWanBi => xt_raw.ji_zu_vec[i].common_ji.current_range = JiZuRangeLeiXing::QiDong,
                    //         JiZuRangeLeiXing::Wen => {
                    //             xt_raw.ji_zu_vec[i].set_bian_su_params(7.0, false);
                    //             xt_raw.ji_zu_vec[i].common_ji.current_range = JiZuRangeLeiXing::BianSu;
                    //         }
                    //         JiZuRangeLeiXing::BianSu => {
                    //             xt_raw.ji_zu_vec[i].set_bian_ya_params(3.0);
                    //             xt_raw.ji_zu_vec[i].common_ji.current_range = JiZuRangeLeiXing::BianYa;
                    //         }
                    //         JiZuRangeLeiXing::BianYa => xt_raw.ji_zu_vec[i].common_ji.current_range = JiZuRangeLeiXing::TingJiZanTai,
                    //         _ => {}
                    //     }
                    // }

                    tick +=1;
                    if tick == 10 {
                        tick = 0;
                        // println!("uid:{}, 状态:{:?}, u:{}, 转速:{}, f:{}， t_current_range:{:?}, bei_che_t:{:?}", i, xt_raw.ji_zu_vec[i].common_ji.current_range, xt_raw.ji_zu_vec[i].common_ji.uab_ext, xt_raw.ji_zu_vec[i].common_ji.zhuan_su, xt_raw.ji_zu_vec[i].common_ji.f_ext, xt_raw.ji_zu_vec[i].common_ji.t_current_range,  xt_raw.ji_zu_vec[i].common_ji.bei_che_t);
                        // println!("uid:{}, 状态:{:?}, u:{}, 转速:{}, f:{}， t_current_range:{:?}, bei_che_t:{:?}, bian_su_t:{:?}, bian_ya_t:{:?}", i, xt_raw.ji_zu_vec[i].common_ji.current_range, xt_raw.ji_zu_vec[i].common_ji.uab_ext, xt_raw.ji_zu_vec[i].common_ji.zhuan_su, xt_raw.ji_zu_vec[i].common_ji.f_ext, xt_raw.ji_zu_vec[i].common_ji.t_current_range,  xt_raw.ji_zu_vec[i].common_ji.bei_che_t,  xt_raw.ji_zu_vec[i].common_ji.bian_su_t,  xt_raw.ji_zu_vec[i].common_ji.bian_ya_t);
                        println!("uid:{}, 状态:{:?}, u:{}, 转速:{}, p:{}， t_current_range:{:?}", 0, xt_raw.ji_zu_vec[0].common_ji.current_range, xt_raw.ji_zu_vec[0].common_ji.uab_ext, xt_raw.ji_zu_vec[0].common_ji.zhuan_su, xt_raw.ji_zu_vec[0].common_ji.p, xt_raw.ji_zu_vec[0].common_ji.t_current_range);
                        println!("uid:{}, 状态:{:?}, u:{}, 转速:{}, p:{}， t_current_range:{:?}", 1, xt_raw.ji_zu_vec[1].common_ji.current_range, xt_raw.ji_zu_vec[1].common_ji.uab_ext, xt_raw.ji_zu_vec[1].common_ji.zhuan_su, xt_raw.ji_zu_vec[1].common_ji.p, xt_raw.ji_zu_vec[1].common_ji.t_current_range);
                    }
                }
                thread::sleep(Duration::from_millis(simctrl::FANG_ZHEN_BU_CHANG as u64));
            }
        });
    }
}
