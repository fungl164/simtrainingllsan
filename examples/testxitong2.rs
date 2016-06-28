extern crate simtraining;
extern crate iron;
extern crate serde;
extern crate serde_json;
extern crate router;
extern crate mount;
extern crate time;
use simtraining::xitong::XiTong;
use simtraining::zhiling::ZhiLing;
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
use simtraining::jizu::{JiZuCtrl, JiZuRangeLeiXing};

fn main() {
    let xt = XiTongThread::new(0);
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
        Iron::new(mount_root).http("localhost:3000").unwrap();
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
    pub fn new(id:usize) -> XiTongThread {
        let mut x = XiTongThread {
            xt : Arc::new(RwLock::new(XiTong::new(id))),
        };
        x.update();
        x
    }
    pub fn update(&mut self) {
        let xt_shared = self.xt.clone();
        let _ = thread::spawn(move || {
            // let mut tick = 0u32;
            loop{
                {
                    // tick +=1;
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

                    //以下为测试内容，实际运行时请注释
                    // println!("{:?}.{:?}", xt_raw.sec, xt_raw.nsec);
                    //机组0备车--启动--合闸---负载0加载--机组1备车--启动--断路器2/4/9/32合闸--机组1并车--负载3加载--机组3备车--启动--合闸--负载8加载--断路器33合闸--断路器4解列--机组3分断--停机--机组0解列--停机--机组2分断--停机
                    match xt_raw.ji_zu_vec[0].common_ji.current_range {
                        JiZuRangeLeiXing::TingJi => xt_raw.ji_zu_vec[0].common_ji.current_range = JiZuRangeLeiXing::BeiCheZanTai,
                        JiZuRangeLeiXing::BeiCheWanBi => xt_raw.ji_zu_vec[0].common_ji.current_range = JiZuRangeLeiXing::QiDong,
                        JiZuRangeLeiXing::Wen => {
                            let duan_lu_qi_id = xt_raw.get_duanluqiid_from_jizuid(0).unwrap();
                            if xt_raw.duan_lu_qi_vec[duan_lu_qi_id].is_off() {
                                xt_temp.duan_lu_qi_vec[0].set_on();
                            }
                            else if !xt_raw.fu_zai_vec[0].is_online{
                                let z = ZhiLing::from_params(ZhiLingType::BianZai(150.0, 150.0), simctrl::DevType::FuZai, 0, 0, 0, simctrl::ZhanWeiType::JiaoLian);
                                xt_raw.handle_zhi_ling(&z);
                            }
                            else {
                                match xt_raw.ji_zu_vec[1].common_ji.current_range {
                                    JiZuRangeLeiXing::TingJi => xt_raw.ji_zu_vec[1].common_ji.current_range = JiZuRangeLeiXing::Wen,
                                    JiZuRangeLeiXing::Wen => {
                                        let duan_lu_qi_id = xt_raw.get_duanluqiid_from_jizuid(0).unwrap();
                                        if xt_raw.duan_lu_qi_vec[duan_lu_qi_id].is_off() {
                                            xt_temp.duan_lu_qi_vec[0].set_on();
                                        }
                                        else if !xt_raw.fu_zai_vec[0].is_online{
                                            let z = ZhiLing::from_params(ZhiLingType::BianZai(150.0, 150.0), simctrl::DevType::FuZai, 0, 0, 0, simctrl::ZhanWeiType::JiaoLian);
                                            xt_raw.handle_zhi_ling(&z);
                                        }
                                        else {
                                            match xt_raw.ji_zu_vec[1].common_ji.current_range {
                                                JiZuRangeLeiXing::TingJi => xt_raw.ji_zu_vec[1].common_ji.current_range = JiZuRangeLeiXing::Wen,
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        JiZuRangeLeiXing::BianYa => xt_raw.ji_zu_vec[i].common_ji.current_range = JiZuRangeLeiXing::TingJiZanTai,
                        _ => {}
                    }
                    // if tick == 10 {
                    //     tick = 0;
                    //     println!("uid:{}, 状态:{:?}, u:{}, 转速:{}, f:{}， t_current_range:{:?}, bei_che_t:{:?}", i, xt_raw.ji_zu_vec[i].common_ji.current_range, xt_raw.ji_zu_vec[i].common_ji.uab_ext, xt_raw.ji_zu_vec[i].common_ji.zhuan_su, xt_raw.ji_zu_vec[i].common_ji.f_ext, xt_raw.ji_zu_vec[i].common_ji.t_current_range,  xt_raw.ji_zu_vec[i].common_ji.bei_che_t);
                    //     // println!("uid:{}, 状态:{:?}, u:{}, 转速:{}, f:{}， t_current_range:{:?}, bei_che_t:{:?}, bian_su_t:{:?}, bian_ya_t:{:?}", i, xt_raw.ji_zu_vec[i].common_ji.current_range, xt_raw.ji_zu_vec[i].common_ji.uab_ext, xt_raw.ji_zu_vec[i].common_ji.zhuan_su, xt_raw.ji_zu_vec[i].common_ji.f_ext, xt_raw.ji_zu_vec[i].common_ji.t_current_range,  xt_raw.ji_zu_vec[i].common_ji.bei_che_t,  xt_raw.ji_zu_vec[i].common_ji.bian_su_t,  xt_raw.ji_zu_vec[i].common_ji.bian_ya_t);
                    //     // println!("uid:{}, 状态:{:?}, u:{}, 转速:{}, f:{}， t_current_range:{:?}", i, xt_raw.ji_zu_vec[i].common_ji.current_range, xt_raw.ji_zu_vec[i].common_ji.uab_ext, xt_raw.ji_zu_vec[i].common_ji.zhuan_su, xt_raw.ji_zu_vec[i].common_ji.f_ext, xt_raw.ji_zu_vec[i].common_ji.t_current_range);
                    // }
                }
                thread::sleep(Duration::from_millis(simctrl::FANG_ZHEN_BU_CHANG as u64));
            }
        });
    }
}