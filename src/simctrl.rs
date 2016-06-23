use mio;
use mio::{ Handler };
// use std::cell::RefCell;
// use std::rc::Rc;
pub trait Update : Handler {
    fn update(&mut self);
    fn run(&mut self, timeout : Self::Timeout, delay : u64) {
        let mut ev_cfg = mio::EventLoopConfig::new();
        ev_cfg.timer_tick_ms(10);
        let mut ev_loop = mio::EventLoop::configured(ev_cfg).unwrap();
        let _ = ev_loop.timeout_ms(timeout, delay).unwrap();
        ev_loop.run(self).unwrap();
    }
}
// pub struct TimerHandler<E : Update> {
//     pub entity : E,
// }
// impl<E : Update> TimerHandler<E> {
//     pub fn new(e : E) -> TimerHandler<E> {
//         let mut ev_cfg = EventLoopConfig::new();
//         ev_cfg.timer_tick_ms(1);
//         TimerHandler {
//             entity : e,
//         }
//     }
// }
//
// impl<E : Update> Handler for TimerHandler<E> {
//     type Timeout = u64;
//     type Message = ();
//     fn timeout(&mut self, _event_loop: &mut EventLoop<Self>, _timeout: Self::Timeout) {
//         self.entity.update();
//         let _ = _event_loop.timeout_ms(_timeout, _timeout).unwrap();
//     }
// }
// pub struct Timer<H : Handler> {
//     pub ev_loop : Box<EventLoop<H>>,
// }
//
// impl<H : Handler> Timer<H> {
//     pub fn new() -> Timer<H> {
//         let mut ev_cfg = ::mio::EventLoopConfig::new();
//         ev_cfg.timer_tick_ms(1);
//         Timer {
//             ev_loop : Box::new(::mio::EventLoop::configured(ev_cfg).unwrap()),
//         }
//     }
//     pub fn start(&mut self, h : &mut H, timeout : H::Timeout, delay : u64) {
//         let _ = self.ev_loop.timeout_ms(timeout, delay).unwrap();
//         self.ev_loop.run(h).unwrap();
//     }
//     pub fn stop(&mut self){
//         if !self.ev_loop.is_running() {
//             self.ev_loop.shutdown();
//         }
//     }
// }

pub const FANG_ZHEN_BU_CHANG : f64 = 100.0; //ms
pub const CHAO_LIU_JI_SUAN_BU_CHANG : f64 = 1000.0; //ms
pub const SIM_MO_REN_BU_CHANG : f64 = FANG_ZHEN_BU_CHANG;

//自动或其他情况下控制序列更新周期
pub const KONG_ZHI_XU_LIE_GENG_XIN_ZHOU_QI : f64 = 1000.0;

//控制序列步骤最大数
pub const KONG_ZHI_XU_LIE_ZUI_DA_BU_ZHOU_SHU : u32 = 25u32;
//pub const DEBUG_ENABLE

//数据更新周期
pub const SHU_JU_GENG_XIN_T : f64 = FANG_ZHEN_BU_CHANG; //ms

pub const TONG_YONG_ZAO_SHENG_XIANG_DUI_FU_ZHI : f64 = 0.0001f64;
pub const DIAN_WANG_E_DING_XIAN_DIAN_YA : f64 = 380.0f64;

pub const GEN_START_SIM_INTERVAL : f64 = FANG_ZHEN_BU_CHANG;
pub const GEN_WEN_SIM_INTERVAL : f64 = FANG_ZHEN_BU_CHANG;

pub const ZONG_SHU_DIAN_ZHAN : usize = 4;
pub const ZONG_SHU_JI_ZU : usize = 13;
pub const ZONG_SHU_JI_ZU_CHAI_YOU : usize = 5;
pub const ZONG_SHU_JI_ZU_QI_LUN : usize = 4;
pub const ZONG_SHU_AN_DIAN : usize = 4;
pub const ZONG_SHU_NODE : usize = 8;
pub const ZONG_SHU_ZHI_LU : usize = 25;
pub const ZONG_SHU_FU_ZAI : usize = 9;
pub const ZONG_SHU_DUAN_LU_QI : usize = 38;
pub const ZONG_SHU_SHOU_TI_DUI_JI_ZU : usize = 0;
pub const ZONG_SHU_WEI_TI_DUI_JI_ZU : usize = 0;
pub const ZONG_SHU_JI_ZU_IN_ONE_DIAN_ZHAN : usize = 3;

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DevType{
    JiZu,
    DuanLuQi,
    FuZai,
    DianZhan,
    AnDian,
    Node,
    ZhiLu,
    Wu,
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ZhanWeiType{
    JiPang,
    ZhuKongZhiPing,
    JiZuKongZhiQi,
    DianZhanKongZhiQi,
    DianZhanJianKongTai,
    JiKongTai,
    BeiYongJiKongTai,
    JiaoLian,
    Admin,
    Internal,
    Wu,
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum CtrlMode {
    Manual,
    SemiAuto,
    Auto,
}
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum OperatingStation {
    JiPang,
    Remote,
    Local,
    JiKong,
}

pub trait SimCtrl {
    fn start_sim(&mut self) -> Self;
    fn stop_sim(&mut self) -> Self;
}
