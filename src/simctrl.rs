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

pub const ZONG_SHU_DIAN_ZHAN : usize = 2;
pub const ZONG_SHU_JI_ZU : usize = 7;
pub const ZONG_SHU_JI_ZU_CHAI_YOU : usize = 6;
pub const ZONG_SHU_JI_ZU_QI_LUN : usize = 0;
pub const ZONG_SHU_AN_DIAN : usize = 1;
pub const ZONG_SHU_NODE : usize = 8;
pub const ZONG_SHU_ZHI_LU : usize = 8;
pub const ZONG_SHU_FU_ZAI : usize = 6;
pub const ZONG_SHU_DUAN_LU_QI : usize = 15;
pub const ZONG_SHU_SHOU_TI_DUI_JI_ZU : usize = 0;
pub const ZONG_SHU_WEI_TI_DUI_JI_ZU : usize = 0;
pub const ZONG_SHU_JI_ZU_IN_ONE_DIAN_ZHAN : usize = 3;

#[derive(PartialEq, Copy, Clone, Debug)]
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

#[derive(PartialEq, Copy, Clone, Debug)]
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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum CtrlMode {
    Manual,
    SemiAuto,
    Auto,
}
#[derive(PartialEq, Copy, Clone, Debug)]
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
