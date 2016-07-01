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
    //机组并联：电站0/1（控制方式半自动---操作部位集控）---机组0（备车--启动--合闸）--负载0（加载）--断路器2/4/32/33（合闸）--机组1（备车--启动--并车)---机组3（备车--启动--合闸）---断路器9并车----机组3(解列---停机）---断路器2解列------机组1(分闸---停机）---机组0（分闸--停机）
    let mut flow = vec![ZhiLing::from_params(ZhiLingType::BeiChe, simctrl::DevType::JiZu, 0, 0, 0, simctrl::ZhanWeiType::JiaoLian); 26];

    flow[25].zhi_ling_type = ZhiLingType::OperatingStation(simctrl::OperatingStation::JiKong);
    flow[25].dev_type = simctrl::DevType::DianZhan;

    flow[24].zhi_ling_type = ZhiLingType::CtrlMode(simctrl::CtrlMode::SemiAuto);
    flow[24].dev_type = simctrl::DevType::DianZhan;

    flow[23].zhi_ling_type = ZhiLingType::OperatingStation(simctrl::OperatingStation::JiKong);
    flow[23].dev_type = simctrl::DevType::DianZhan;
    flow[23].dev_id = 1;

    flow[22].zhi_ling_type = ZhiLingType::CtrlMode(simctrl::CtrlMode::SemiAuto);
    flow[22].dev_type = simctrl::DevType::DianZhan;
    flow[22].dev_id = 1;

    //机组并联：电站0/1（控制方式半自动---操作部位集控）---机组0（备车--启动--合闸）--负载0（加载）--断路器2/4/32/33（合闸）--机组1（备车--启动--并车)---机组3（备车--启动--合闸）---断路器9并车----机组3(解列---停机）---断路器2解列------机组1(分闸---停机）---机组0（分闸--停机）
    flow[21].zhi_ling_type = ZhiLingType::BeiChe;

    flow[20].zhi_ling_type = ZhiLingType::QiDong;

    flow[19].zhi_ling_type = ZhiLingType::HeZhaBingChe;

    flow[18].zhi_ling_type = ZhiLingType::BianZai(1600.0, 1200.0);
    flow[18].dev_type = simctrl::DevType::FuZai;

    flow[17].zhi_ling_type = ZhiLingType::HeZhaBingChe;
    flow[17].dev_type = simctrl::DevType::DuanLuQi;
    flow[17].dev_id = 2;

    flow[16].zhi_ling_type = ZhiLingType::HeZhaBingChe;
    flow[16].dev_type = simctrl::DevType::DuanLuQi;
    flow[16].dev_id = 4;

    flow[15].zhi_ling_type = ZhiLingType::HeZhaBingChe;
    flow[15].dev_type = simctrl::DevType::DuanLuQi;
    flow[15].dev_id = 32;

    flow[14].zhi_ling_type = ZhiLingType::HeZhaBingChe;
    flow[14].dev_type = simctrl::DevType::DuanLuQi;
    flow[14].dev_id = 33;

    //机组并联：电站0/1（控制方式半自动---操作部位集控）---机组0（备车--启动--合闸）--负载0（加载）--断路器2/4/32/33（合闸）--机组1（备车--启动--并车)---机组3（备车--启动--合闸）---断路器9并车----机组3(解列---停机）---断路器2解列------机组1(分闸---停机）---机组0（分闸--停机）

    flow[13].zhi_ling_type = ZhiLingType::BeiChe;
    flow[13].dev_id = 1;

    flow[12].zhi_ling_type = ZhiLingType::QiDong;
    flow[12].dev_id = 1;

    flow[11].zhi_ling_type = ZhiLingType::HeZhaBingChe;
    flow[11].dev_id = 1;

    flow[10].zhi_ling_type = ZhiLingType::BeiChe;
    flow[10].dev_id = 3;

    flow[9].zhi_ling_type = ZhiLingType::QiDong;
    flow[9].dev_id = 3;

    flow[8].zhi_ling_type = ZhiLingType::HeZhaBingChe;
    flow[8].dev_id = 3;

    flow[7].zhi_ling_type = ZhiLingType::HeZhaBingChe;
    flow[7].dev_type = simctrl::DevType::DuanLuQi;
    flow[7].dev_id = 9;

    //机组并联：电站0/1（控制方式半自动---操作部位集控）---机组0（备车--启动--合闸）--负载0（加载）--断路器2/4/32/33（合闸）--机组1（备车--启动--并车)---机组3（备车--启动--合闸）---断路器9并车----机组3(解列---停机）---断路器2解列------机组1(分闸---停机）---机组0（分闸--停机）

    flow[6].zhi_ling_type = ZhiLingType::FenZhaJieLie;
    flow[6].dev_id = 3;

    flow[5].zhi_ling_type = ZhiLingType::TingJi;
    flow[5].dev_id = 3;

    flow[4].zhi_ling_type = ZhiLingType::FenZhaJieLie;
    flow[4].dev_type = simctrl::DevType::DuanLuQi;
    flow[4].dev_id = 2;

    flow[3].zhi_ling_type = ZhiLingType::FenZhaJieLie;
    flow[3].dev_id = 1;

    flow[2].zhi_ling_type = ZhiLingType::TingJi;
    flow[2].dev_id = 1;

    flow[1].zhi_ling_type = ZhiLingType::FenZhaJieLie;
    flow[1].dev_id = 0;

    flow[0].zhi_ling_type = ZhiLingType::TingJi;
    flow[0].dev_id = 0;


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
                        println!("电站0-1（uid, 控制方式, 操作部位, 控制方式设置, 操作部位设置, u, p）:\n({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?}, {:?}, {:?}, {:?})", xt_raw.dian_zhan_vec[0].uid, xt_raw.dian_zhan_vec[0].ctrl_mode, xt_raw.dian_zhan_vec[0].operating_station, xt_raw.dian_zhan_vec[0].ctrl_mode_she_zhi, xt_raw.dian_zhan_vec[0].operating_station_she_zhi, xt_raw.dian_zhan_vec[0].u, xt_raw.dian_zhan_vec[0].p, xt_raw.dian_zhan_vec[1].uid, xt_raw.dian_zhan_vec[1].ctrl_mode, xt_raw.dian_zhan_vec[1].operating_station, xt_raw.dian_zhan_vec[1].ctrl_mode_she_zhi, xt_raw.dian_zhan_vec[1].operating_station_she_zhi, xt_raw.dian_zhan_vec[1].u, xt_raw.dian_zhan_vec[1].p);
                        println!("机组0-2（uid, current_range, u, f, p）:\n({:?}, {:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?}, {:?})", xt_raw.ji_zu_vec[0].uid, xt_raw.ji_zu_vec[0].common_ji.current_range, xt_raw.ji_zu_vec[0].common_ji.uab_ext, xt_raw.ji_zu_vec[0].common_ji.f_ext, xt_raw.ji_zu_vec[0].common_ji.p, xt_raw.ji_zu_vec[1].uid, xt_raw.ji_zu_vec[1].common_ji.current_range, xt_raw.ji_zu_vec[1].common_ji.uab_ext, xt_raw.ji_zu_vec[1].common_ji.f_ext, xt_raw.ji_zu_vec[1].common_ji.p, xt_raw.ji_zu_vec[2].uid, xt_raw.ji_zu_vec[2].common_ji.current_range, xt_raw.ji_zu_vec[2].common_ji.uab_ext, xt_raw.ji_zu_vec[2].common_ji.f_ext, xt_raw.ji_zu_vec[2].common_ji.p);
                        println!("节点1-3,7-9（uid, u, f, 状态）:\n({:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?})", xt_raw.node_vec[1].uid, xt_raw.node_vec[1].u, xt_raw.node_vec[1].f, xt_raw.node_vec[1].status, xt_raw.node_vec[2].uid, xt_raw.node_vec[2].u, xt_raw.node_vec[2].f, xt_raw.node_vec[2].status, xt_raw.node_vec[3].uid, xt_raw.node_vec[3].u, xt_raw.node_vec[3].f, xt_raw.node_vec[3].status, xt_raw.node_vec[7].uid, xt_raw.node_vec[7].u, xt_raw.node_vec[7].f, xt_raw.node_vec[7].status, xt_raw.node_vec[8].uid, xt_raw.node_vec[8].u, xt_raw.node_vec[8].f, xt_raw.node_vec[8].status, xt_raw.node_vec[9].uid, xt_raw.node_vec[9].u, xt_raw.node_vec[9].f, xt_raw.node_vec[9].status);
                        println!("支路0,2,5,19,20（uid, p, i, 状态）:\n({:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?})\n({:?}, {:?}, {:?}, {:?})", xt_raw.zhi_lu_vec[0].uid, xt_raw.zhi_lu_vec[0].p, xt_raw.zhi_lu_vec[0].i, xt_raw.zhi_lu_vec[0].status, xt_raw.zhi_lu_vec[2].uid, xt_raw.zhi_lu_vec[2].p, xt_raw.zhi_lu_vec[2].i, xt_raw.zhi_lu_vec[2].status, xt_raw.zhi_lu_vec[5].uid, xt_raw.zhi_lu_vec[5].p, xt_raw.zhi_lu_vec[5].i, xt_raw.zhi_lu_vec[5].status, xt_raw.zhi_lu_vec[19].uid, xt_raw.zhi_lu_vec[19].p, xt_raw.zhi_lu_vec[19].i, xt_raw.zhi_lu_vec[19].status, xt_raw.zhi_lu_vec[20].uid, xt_raw.zhi_lu_vec[20].p, xt_raw.zhi_lu_vec[20].i, xt_raw.zhi_lu_vec[20].status);
                        println!("断路器0,1,2,4,6,9,32,33（uid, 状态）:\n({:?}, {:?})  ({:?}, {:?})  ({:?}, {:?})  ({:?}, {:?})\n({:?}, {:?})  ({:?}, {:?})  ({:?}, {:?})  ({:?}, {:?})\n", xt_raw.duan_lu_qi_vec[0].uid, xt_raw.duan_lu_qi_vec[0].is_on(), xt_raw.duan_lu_qi_vec[1].uid, xt_raw.duan_lu_qi_vec[1].is_on(), xt_raw.duan_lu_qi_vec[2].uid, xt_raw.duan_lu_qi_vec[2].is_on(), xt_raw.duan_lu_qi_vec[4].uid, xt_raw.duan_lu_qi_vec[4].is_on(), xt_raw.duan_lu_qi_vec[6].uid, xt_raw.duan_lu_qi_vec[6].is_on(), xt_raw.duan_lu_qi_vec[9].uid, xt_raw.duan_lu_qi_vec[9].is_on(), xt_raw.duan_lu_qi_vec[32].uid, xt_raw.duan_lu_qi_vec[32].is_on(), xt_raw.duan_lu_qi_vec[33].uid, xt_raw.duan_lu_qi_vec[33].is_on());
                    }
                }
                thread::sleep(Duration::from_millis(simctrl::FANG_ZHEN_BU_CHANG as u64));
            }
        });
    }
}
