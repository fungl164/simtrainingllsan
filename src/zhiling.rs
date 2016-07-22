use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use serde_json;
use simctrl;
use xitong::XiTong;
use std::string::String;
use jizu::{ JiZuRangeLeiXing };
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ZhiLingType {
    Tick,
    ZhongZaiAsk,
    CtrlMode(simctrl::CtrlMode),
    OperatingStation(simctrl::OperatingStation),
    Prio,
    QiDong,
    TingJi,
    HeZhaBingChe,
    FenZhaJieLie,
    FenJiXieZai,
    YueKong,
    ShiDianZiQiDong,
    ZiDongTouWang,
    ZiDongGouWang,
    GongLvYuanZeZiDongZengJi,
    DianLiuYuanZeZiDongZengJi,
    GongLvYuanZeZiDongJianJi,
    GuZhangZiDongChuLi,
    WangLuoChongGou,
    BeiChe,
    BianZai(f64, f64),
    ZhongZaiJiaZai(f64, f64),
    AnDianOn,
    AnDianOff,
    AnDianHeZha,
    AnDianFenZha,
    TouRu,
    TuiChu,
    BianSu(f64),
    BianYa(f64),
    JinJiTingJi,
    XiaoSheng,
    YingDa,

    KaiShiKaoHe,
    JieShuKaoHe,

    GenerateYiBanGuZhang(FaultType),
    EliminateYiBanGuZhang(FaultType),

    GenerateYiJiGuZhang(FaultType),
    EliminateYiJiGuZhang(FaultType),

    GenerateErJiGuZhang(FaultType),
    EliminateErJiGuZhang(FaultType),

    GenerateQiTaGuZhang(FaultType),
    EliminateQiTaGuZhang(FaultType),

}
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum FaultType {
    RanYouXieLou,
    QuZhouXiangYaLiGao,
    QiDongKongQiYaLiDi,
    FaDongJiLengQueQiHaiShuiXieLou,
    PaiQiWenDuGao,
    ZengYaKongQiWenDuGao,
    QianZhouChengHuaYouLiuLiangDi,
    HouZhouChengHuaYouLiuLiangDi,
    HuaYouWenDuGao,
    HaiShuiYaLiDi,
    PengZhangShuiXiangYeWeiDi,
    QianZhouChengWenDuGao,
    HouZhouChengWenDuGao,
    QiDongShiBai,
    TingJiShiBai,
    GuoDianYa,
    QianDianYa,
    JiZuGuoDianLiu,
    SanXiangDianLiuBuPingHeng,
    DuanLuQiYuBaoJing,
    PinLvYueXian,
    DuanLuQiZongHeGuZhangBaoJing,
    FaDianJiLengQueShuiXieLou,
    JinFengWenDuGao,
    ChuFengWenDuGao,
    WuYouCaoYeWeiGao,
    JiZuZhuangZhiZhaoNeiFengJiGuZhang,
    JiZuZhuangZhiXiaoFangSheBeiGuZhang,
    ZhiNengQiNangGeZhenZhuangZhiYiBanGuZhang,

    YiJiGuZhang,
    HuaYouYaLiGuoDi,
    ChaoSuTingJi,
    QianZhouChengYuGongHuaYouLiuLiangDi,
    HouZhouChengYuGongHuaYouLiuLiangDi,
    ChaiYouJiJinJiTingJi,
    JiPangShouDongYingJiTingJi,
    PaiQiYaLiGuoGao,

    ErJiGuZhang,
    BingCheShiBai,
    RaoZuWenDuGao,
    HuaYouYaLiDi,
    LengQueShuiWenDuGao,
    RaoZuWenDuGaoTiaoZhaTingJi,
    PaiQiYaLiGao,
    JianSuQiRunHuaYouYaLiDi,
    QiLunJiHouZhouChengWenDuGao,
    FaDianJiHouZhouChengWenDuGao,
    YouXiangWenDuGao,

    WaiBuDuanLu,
    NiGongLv,
    NeiBuDuanLu,

    DianWangWuGongLvYuLiang,
    FenJiXieZai1,
    FenJiXieZai2,
    DianWangGongLvYuLiangGuoXiao,
    ZengJi,
    JianJi,
    WuJiZuNengZiQiDong,
    ZaiWangJiZuWuFaJieLie,
    JiZuZongHeBaoJing,
    GongLvYuBaoJing,
    XiangJianBuPingHeng,
    TongBuShiBai,
    FenDuanShiBai,
    GuoDianLiu,
    HeZhaShiBai,
    JieLieShiBai,
}
pub trait Condition {
    fn can_exec(&self, xt : &mut XiTong) -> bool;
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct ZhiLingResponse {
    pub code : i32,            //状态编码: 0表示指令处理失败, 1表示指令处理成功.
    pub response : String,       //指令响应字符串,只取两个值:(1)如果处理成功,则显示"指令处理成功, 请等待被操控设备状态转换完成"; (2)如果处理失败,则显示"指令处理失败"
    pub description : String,   //指令处理失败时的具体描述, 若成功则为空
    pub cause : String,         //指令处理失败的原因, 若成功则为空

    pub zhi_ling_type : String, //指令类型
    pub dev_type : String,      //设备类型
    pub dev_id : usize,         //设备id
    pub actor_id : usize,       //用户id
    pub zhan_wei_id : usize,    //站位id
    pub zhan_wei_type : String, //站位类型
}

impl ZhiLingResponse {
    pub fn new() -> ZhiLingResponse {
        ZhiLingResponse {
            code : -1,
            response : "".to_string(),
            description : "".to_string(),
            cause : "".to_string(),

            zhi_ling_type : "".to_string(),
            dev_type : "".to_string(),
            dev_id : 0,
            actor_id : 0,
            zhan_wei_id : 0,
            zhan_wei_type : "".to_string(),
        }
    }
    pub fn from_result(result : Result<YingDaType, YingDaErr>) -> ZhiLingResponse {
        match result {
            Ok(t) => {
                match t {
                    YingDaType::Success(zl) => {
                        return ZhiLingResponse {
                            code : 1,
                            response : "指令处理成功, 请等待被操控设备状态转换完成".to_string(),
                            description : "".to_string(),
                            cause : "".to_string(),

                            zhi_ling_type : match zl.zhi_ling_type {
                                ZhiLingType::Tick => "心跳".to_string(),
                                ZhiLingType::ZhongZaiAsk => "重载询问".to_string(),
                                ZhiLingType::CtrlMode(ctrl_mode) => match ctrl_mode {
                                    simctrl::CtrlMode::Manual => "控制方式: 手动".to_string(),
                                    simctrl::CtrlMode::SemiAuto => "控制方式: 半自动".to_string(),
                                    simctrl::CtrlMode::Auto => "控制方式: 自动".to_string(),
                                },
                                ZhiLingType::OperatingStation(o_s) => match o_s {
                                    simctrl::OperatingStation::JiPang => "操作部位: 机旁".to_string(),
                                    simctrl::OperatingStation::Remote => "操作部位: 遥控".to_string(),
                                    simctrl::OperatingStation::Local => "操作部位: 本地".to_string(),
                                    simctrl::OperatingStation::JiKong => "操作部位: 集控".to_string(),
                                },
                                ZhiLingType::Prio => "优先级".to_string(),
                                ZhiLingType::QiDong => "启动".to_string(),
                                ZhiLingType::TingJi => "停机".to_string(),
                                ZhiLingType::HeZhaBingChe => "合闸/并车".to_string(),
                                ZhiLingType::FenZhaJieLie => "分闸/解列".to_string(),
                                ZhiLingType::FenJiXieZai => "分级卸载".to_string(),
                                ZhiLingType::YueKong => "越控".to_string(),
                                ZhiLingType::ShiDianZiQiDong => "失电自启动".to_string(),
                                ZhiLingType::ZiDongTouWang => "自动投网".to_string(),
                                ZhiLingType::ZiDongGouWang => "自动构网".to_string(),
                                ZhiLingType::GongLvYuanZeZiDongZengJi => "功率原则自动增机".to_string(),
                                ZhiLingType::DianLiuYuanZeZiDongZengJi => "电流原则自动增机".to_string(),
                                ZhiLingType::GongLvYuanZeZiDongJianJi => "功率原则自动减机".to_string(),
                                ZhiLingType::GuZhangZiDongChuLi => "故障自动处理".to_string(),
                                ZhiLingType::WangLuoChongGou => "网络重构".to_string(),
                                ZhiLingType::BeiChe => "备车".to_string(),
                                ZhiLingType::BianZai(p, q) => format!("变载:{{p:{},q:{}}}", p, q),
                                ZhiLingType::ZhongZaiJiaZai(p, q) => format!("重载加载:{{p:{},q:{}}}", p, q),
                                ZhiLingType::AnDianOn => "岸电接通".to_string(),
                                ZhiLingType::AnDianOff => "岸电挂断".to_string(),
                                ZhiLingType::AnDianHeZha => "岸电合闸".to_string(),
                                ZhiLingType::AnDianFenZha => "岸电分闸".to_string(),
                                ZhiLingType::TouRu => "投入".to_string(),
                                ZhiLingType::TuiChu => "退出".to_string(),
                                ZhiLingType::BianSu(zhuan_su) => format!("变速:{{zhuan_su:{}}}", zhuan_su),
                                ZhiLingType::BianYa(u) => format!("变压:{{u:{}}}", u),
                                ZhiLingType::JinJiTingJi => "紧急停机".to_string(),
                                ZhiLingType::XiaoSheng => "消声".to_string(),
                                ZhiLingType::YingDa => "应答".to_string(),

                                ZhiLingType::KaiShiKaoHe => "开始考核".to_string(),
                                ZhiLingType::JieShuKaoHe => "结束考核".to_string(),

                                ZhiLingType::GenerateYiBanGuZhang(fault_type) => format!("产生一般故障:{{fault_type:{:?}}}", fault_type),
                                ZhiLingType::EliminateYiBanGuZhang(fault_type) => format!("消除一般故障:{{fault_type:{:?}}}", fault_type),

                                ZhiLingType::GenerateYiJiGuZhang(fault_type) => format!("产生一级故障:{{fault_type:{:?}}}", fault_type),
                                ZhiLingType::EliminateYiJiGuZhang(fault_type) => format!("消除一级故障:{{fault_type:{:?}}}", fault_type),

                                ZhiLingType::GenerateErJiGuZhang(fault_type) => format!("产生二级故障:{{fault_type:{:?}}}", fault_type),
                                ZhiLingType::EliminateErJiGuZhang(fault_type) => format!("消除二般故障:{{fault_type:{:?}}}", fault_type),

                                ZhiLingType::GenerateQiTaGuZhang(fault_type) => format!("产生其他故障:{{fault_type:{:?}}}", fault_type),
                                ZhiLingType::EliminateQiTaGuZhang(fault_type) => format!("消除其他故障:{{fault_type:{:?}}}", fault_type),
                            },
                            dev_type : match zl.dev_type {
                                simctrl::DevType::JiZu => format!("机组"),
                                simctrl::DevType::DuanLuQi => format!("断路器"),
                                simctrl::DevType::FuZai => format!("负载"),
                                simctrl::DevType::DianZhan => format!("电站"),
                                simctrl::DevType::AnDian => format!("岸电"),
                                simctrl::DevType::Node => format!("节点"),
                                simctrl::DevType::ZhiLu => format!("支路"),
                                simctrl::DevType::Wu => format!("无"),
                            },
                            dev_id : zl.dev_id,
                            actor_id : zl.actor_id,
                            zhan_wei_id : zl.zhan_wei_id,
                            zhan_wei_type : match zl.zhan_wei_type {
                                simctrl::ZhanWeiType::JiPang => format!("机旁"),
                                simctrl::ZhanWeiType::ZhuKongZhiPing => format!("主控制屏"),
                                simctrl::ZhanWeiType::JiZuKongZhiQi => format!("机组控制器"),
                                simctrl::ZhanWeiType::DianZhanKongZhiQi => format!("电站控制器"),
                                simctrl::ZhanWeiType::DianZhanJianKongTai => format!("电站监控台"),
                                simctrl::ZhanWeiType::JiKongTai => format!("主集控台"),
                                simctrl::ZhanWeiType::BeiYongJiKongTai => format!("备用集控台"),
                                simctrl::ZhanWeiType::JiaoLian => format!("教练"),
                                simctrl::ZhanWeiType::Admin => format!("管理员"),
                                simctrl::ZhanWeiType::Internal => format!("内部"),
                                simctrl::ZhanWeiType::Wu => format!("无"),
                            },
                        };
                    }
                    _ => return ZhiLingResponse::new(),
                }
            },
            Err(e) => {
                match e {
                    YingDaErr::ZhongZaiAskFail(zl, d, c) | YingDaErr::BeiCheFail(zl, d, c) | YingDaErr::QiDongFail(zl, d, c) | YingDaErr::HeZhaBingCheFail(zl, d, c) | YingDaErr::FenZhaJieLieFail(zl, d, c) | YingDaErr::TingJiFail(zl, d, c) | YingDaErr::CtrlModeFail(zl, d, c) | YingDaErr::OperatingStationFail(zl, d, c) | YingDaErr::CtrlModeAndOperatingStationFail(zl, d, c) | YingDaErr::AnDianFail(zl, d, c) | YingDaErr::AnDianHeZhaFail(zl, d, c) | YingDaErr::AnDianFenZhaFail(zl, d, c) | YingDaErr::BianSuFail(zl, d, c) | YingDaErr::BianYaFail(zl, d, c) | YingDaErr::JinJiTingJiFail(zl, d, c) | YingDaErr::PrioFail(zl, d, c) | YingDaErr::TouRuFail(zl, d, c) | YingDaErr::TuiChuFail(zl, d, c) | YingDaErr::XiaoShengFail(zl, d, c) | YingDaErr::YingDaFail(zl, d, c) | YingDaErr::GuZhangHuanJiFail(zl, d, c) | YingDaErr::DevNotExist(zl, d, c) | YingDaErr::DevTypeNotMatch(zl, d, c) | YingDaErr::Invalid(zl, d, c) | YingDaErr::IdNotMatch(zl, d, c) => {
                        return ZhiLingResponse {
                            code : 0,
                            response : "指令处理失败".to_string(),
                            description : d.to_string(),
                            cause : c.to_string(),

                            zhi_ling_type : match zl.zhi_ling_type {
                                ZhiLingType::Tick => "心跳".to_string(),
                                ZhiLingType::ZhongZaiAsk => "重载询问".to_string(),
                                ZhiLingType::CtrlMode(ctrl_mode) => match ctrl_mode {
                                    simctrl::CtrlMode::Manual => "控制方式: 手动".to_string(),
                                    simctrl::CtrlMode::SemiAuto => "控制方式: 半自动".to_string(),
                                    simctrl::CtrlMode::Auto => "控制方式: 自动".to_string(),
                                },
                                ZhiLingType::OperatingStation(o_s) => match o_s {
                                    simctrl::OperatingStation::JiPang => "操作部位: 机旁".to_string(),
                                    simctrl::OperatingStation::Remote => "操作部位: 遥控".to_string(),
                                    simctrl::OperatingStation::Local => "操作部位: 本地".to_string(),
                                    simctrl::OperatingStation::JiKong => "操作部位: 集控".to_string(),
                                },
                                ZhiLingType::Prio => "优先级".to_string(),
                                ZhiLingType::QiDong => "启动".to_string(),
                                ZhiLingType::TingJi => "停机".to_string(),
                                ZhiLingType::HeZhaBingChe => "合闸/并车".to_string(),
                                ZhiLingType::FenZhaJieLie => "分闸/解列".to_string(),
                                ZhiLingType::FenJiXieZai => "分级卸载".to_string(),
                                ZhiLingType::YueKong => "越控".to_string(),
                                ZhiLingType::ShiDianZiQiDong => "失电自启动".to_string(),
                                ZhiLingType::ZiDongTouWang => "自动投网".to_string(),
                                ZhiLingType::ZiDongGouWang => "自动构网".to_string(),
                                ZhiLingType::GongLvYuanZeZiDongZengJi => "功率原则自动增机".to_string(),
                                ZhiLingType::DianLiuYuanZeZiDongZengJi => "电流原则自动增机".to_string(),
                                ZhiLingType::GongLvYuanZeZiDongJianJi => "功率原则自动减机".to_string(),
                                ZhiLingType::GuZhangZiDongChuLi => "故障自动处理".to_string(),
                                ZhiLingType::WangLuoChongGou => "网络重构".to_string(),
                                ZhiLingType::BeiChe => "备车".to_string(),
                                ZhiLingType::BianZai(p, q) => format!("变载:{{p:{},q:{}}}", p, q),
                                ZhiLingType::ZhongZaiJiaZai(p, q) => format!("重载加载:{{p:{},q:{}}}", p, q),
                                ZhiLingType::AnDianOn => "岸电接通".to_string(),
                                ZhiLingType::AnDianOff => "岸电挂断".to_string(),
                                ZhiLingType::AnDianHeZha => "岸电合闸".to_string(),
                                ZhiLingType::AnDianFenZha => "岸电分闸".to_string(),
                                ZhiLingType::TouRu => "投入".to_string(),
                                ZhiLingType::TuiChu => "退出".to_string(),
                                ZhiLingType::BianSu(zhuan_su) => format!("变速:{{zhuan_su:{}}}", zhuan_su),
                                ZhiLingType::BianYa(u) => format!("变压:{{u:{}}}", u),
                                ZhiLingType::JinJiTingJi => "紧急停机".to_string(),
                                ZhiLingType::XiaoSheng => "消声".to_string(),
                                ZhiLingType::YingDa => "应答".to_string(),

                                ZhiLingType::KaiShiKaoHe => "开始考核".to_string(),
                                ZhiLingType::JieShuKaoHe => "结束考核".to_string(),

                                ZhiLingType::GenerateYiBanGuZhang(fault_type) => format!("产生一般故障:{{fault_type:{:?}}}", fault_type),
                                ZhiLingType::EliminateYiBanGuZhang(fault_type) => format!("消除一般故障:{{fault_type:{:?}}}", fault_type),

                                ZhiLingType::GenerateYiJiGuZhang(fault_type) => format!("产生一级故障:{{fault_type:{:?}}}", fault_type),
                                ZhiLingType::EliminateYiJiGuZhang(fault_type) => format!("消除一级故障:{{fault_type:{:?}}}", fault_type),

                                ZhiLingType::GenerateErJiGuZhang(fault_type) => format!("产生二级故障:{{fault_type:{:?}}}", fault_type),
                                ZhiLingType::EliminateErJiGuZhang(fault_type) => format!("消除二般故障:{{fault_type:{:?}}}", fault_type),

                                ZhiLingType::GenerateQiTaGuZhang(fault_type) => format!("产生其他故障:{{fault_type:{:?}}}", fault_type),
                                ZhiLingType::EliminateQiTaGuZhang(fault_type) => format!("消除其他故障:{{fault_type:{:?}}}", fault_type),
                            },
                            dev_type : match zl.dev_type {
                                simctrl::DevType::JiZu => format!("机组"),
                                simctrl::DevType::DuanLuQi => format!("断路器"),
                                simctrl::DevType::FuZai => format!("负载"),
                                simctrl::DevType::DianZhan => format!("电站"),
                                simctrl::DevType::AnDian => format!("岸电"),
                                simctrl::DevType::Node => format!("节点"),
                                simctrl::DevType::ZhiLu => format!("支路"),
                                simctrl::DevType::Wu => format!("无"),
                            },
                            dev_id : zl.dev_id,
                            actor_id : zl.actor_id,
                            zhan_wei_id : zl.zhan_wei_id,
                            zhan_wei_type : match zl.zhan_wei_type {
                                simctrl::ZhanWeiType::JiPang => format!("机旁"),
                                simctrl::ZhanWeiType::ZhuKongZhiPing => format!("主控制屏"),
                                simctrl::ZhanWeiType::JiZuKongZhiQi => format!("机组控制器"),
                                simctrl::ZhanWeiType::DianZhanKongZhiQi => format!("电站控制器"),
                                simctrl::ZhanWeiType::DianZhanJianKongTai => format!("电站监控台"),
                                simctrl::ZhanWeiType::JiKongTai => format!("主集控台"),
                                simctrl::ZhanWeiType::BeiYongJiKongTai => format!("备用集控台"),
                                simctrl::ZhanWeiType::JiaoLian => format!("教练"),
                                simctrl::ZhanWeiType::Admin => format!("管理员"),
                                simctrl::ZhanWeiType::Internal => format!("内部"),
                                simctrl::ZhanWeiType::Wu => format!("无"),
                            },
                        };
                    }
                }
            },
        }
    }

}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ZhiLing {
  pub zhi_ling_type : ZhiLingType,
  pub dev_type : simctrl::DevType,
  pub dev_id : usize,
  pub actor_id : usize,
  pub zhan_wei_id : usize,
  pub zhan_wei_type : simctrl::ZhanWeiType,
}

impl ZhiLing {
    pub fn new() -> ZhiLing {
        ZhiLing{
            zhi_ling_type : ZhiLingType::Tick,
            dev_type : simctrl::DevType::Wu,
            dev_id : usize::max_value(),
            actor_id : usize::max_value(),
            zhan_wei_id : usize::max_value(),
            zhan_wei_type : simctrl::ZhanWeiType::Wu,
        }
    }
    pub fn from_params(zhi_ling_type : ZhiLingType, dev_type : simctrl::DevType, dev_id : usize, actor_id : usize, zhan_wei_id : usize, zhan_wei_type : simctrl::ZhanWeiType) -> ZhiLing {
        ZhiLing{
            zhi_ling_type : zhi_ling_type,
            dev_type : dev_type,
            dev_id : dev_id,
            actor_id : actor_id,
            zhan_wei_id : zhan_wei_id,
            zhan_wei_type : zhan_wei_type,
        }
    }
    pub fn zhi_ling_example_generate_yi_ban_gu_zhang_handler(_: &mut Request) -> IronResult<Response> {
        let mut z = ZhiLing::new();
        z.zhi_ling_type = ZhiLingType::GenerateYiBanGuZhang(FaultType::RanYouXieLou);
        z.dev_type = simctrl::DevType::JiZu;
        z.dev_id = 0;
        z.actor_id = 0;
        z.zhan_wei_id = 0;
        z.zhan_wei_type = simctrl::ZhanWeiType::JiaoLian;
        let z_ser = serde_json::to_string(&z).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, z_ser)))
    }
    pub fn zhi_ling_example_eliminate_yi_ban_gu_zhang_handler(_: &mut Request) -> IronResult<Response> {
        let mut z = ZhiLing::new();
        z.zhi_ling_type = ZhiLingType::EliminateYiBanGuZhang(FaultType::RanYouXieLou);
        z.dev_type = simctrl::DevType::JiZu;
        z.dev_id = 0;
        z.actor_id = 0;
        z.zhan_wei_id = 0;
        z.zhan_wei_type = simctrl::ZhanWeiType::JiaoLian;
        let z_ser = serde_json::to_string(&z).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, z_ser)))
    }
    pub fn zhi_ling_example_operating_station_handler(_: &mut Request) -> IronResult<Response> {
        let mut z = ZhiLing::new();
        z.zhi_ling_type = ZhiLingType::OperatingStation(simctrl::OperatingStation::JiKong);
        z.dev_type = simctrl::DevType::DianZhan;
        z.dev_id = 0;
        z.actor_id = 0;
        z.zhan_wei_id = 0;
        z.zhan_wei_type = simctrl::ZhanWeiType::JiaoLian;
        let z_ser = serde_json::to_string(&z).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, z_ser)))
    }
    pub fn zhi_ling_example_ctrl_mode_handler(_: &mut Request) -> IronResult<Response> {
        let mut z = ZhiLing::new();
        z.zhi_ling_type = ZhiLingType::CtrlMode(simctrl::CtrlMode::SemiAuto);
        z.dev_type = simctrl::DevType::DianZhan;
        z.dev_id = 0;
        z.actor_id = 0;
        z.zhan_wei_id = 0;
        z.zhan_wei_type = simctrl::ZhanWeiType::JiaoLian;
        let z_ser = serde_json::to_string(&z).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, z_ser)))
    }
    pub fn zhi_ling_example_prio_handler(_: &mut Request) -> IronResult<Response> {
        let mut z = ZhiLing::new();
        z.zhi_ling_type = ZhiLingType::Prio;
        z.dev_type = simctrl::DevType::JiZu;
        z.dev_id = 0;
        z.actor_id = 0;
        z.zhan_wei_id = 0;
        z.zhan_wei_type = simctrl::ZhanWeiType::JiaoLian;
        let z_ser = serde_json::to_string(&z).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, z_ser)))
    }
    pub fn zhi_ling_example_pretty_handler(_: &mut Request) -> IronResult<Response> {
        let mut z = ZhiLing::new();
        z.zhi_ling_type = ZhiLingType::GenerateYiBanGuZhang(FaultType::RanYouXieLou);
        z.dev_type = simctrl::DevType::FuZai;
        z.dev_id = 0;
        z.actor_id = 0;
        z.zhan_wei_id = 0;
        z.zhan_wei_type = simctrl::ZhanWeiType::JiaoLian;
        let z_ser_pretty = serde_json::to_string_pretty(&z).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, z_ser_pretty)))
    }
    pub fn zhi_ling_handler(_: &mut Request) -> IronResult<Response> {
        let mut z = ZhiLing::new();
        z.zhi_ling_type = ZhiLingType::BeiChe;
        z.dev_type = simctrl::DevType::JiZu;
        z.dev_id = 0;
        z.actor_id = 0;
        z.zhan_wei_id = 0;
        z.zhan_wei_type = simctrl::ZhanWeiType::JiaoLian;
        let z_ser_pretty = serde_json::to_string(&z).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, z_ser_pretty)))
    }
}

impl Condition for ZhiLing {
    fn can_exec(&self, xt : &mut XiTong) -> bool {
        if !xt.is_zhi_ling_valid(self) {
            return false;
        }
        match self.zhi_ling_type {
            ZhiLingType::QiDong => {
                if xt.ji_zu_vec[self.dev_id].common_ji.current_range == JiZuRangeLeiXing::BeiCheWanBi {
                    return true;
                }
                else {
                    return false;
                }
            },
            ZhiLingType::TingJi => {
                if xt.get_duanluqi_from_jizuid(self.dev_id).unwrap().is_off() {
                    return true;
                }
                else {
                    return false;
                }
            },
            ZhiLingType::HeZhaBingChe => {
                if self.dev_type == simctrl::DevType::JiZu && xt.ji_zu_vec[self.dev_id].common_ji.current_range == JiZuRangeLeiXing::Wen {
                    return true;
                }
                else if self.dev_type == simctrl::DevType::DuanLuQi {
                    let mut xt_temp = xt.clone();
                    xt_temp.duan_lu_qi_vec[self.dev_id].set_on();
                    xt_temp.update_node_group_vec();
                    let jizu_vec_vec_bing_che = xt.compare_two_xi_tong_he_zha_ji_zu(&mut xt_temp);
                    if jizu_vec_vec_bing_che.len() == 2 {
                        if jizu_vec_vec_bing_che[0].len() == 0 || jizu_vec_vec_bing_che[1].len() == 0 {
                            return true;
                        }
                        else if jizu_vec_vec_bing_che[0].len() == 1 || jizu_vec_vec_bing_che[1].len() == 1 {
                            let mut is_all_ji_zu_wen = true;
                            for j in jizu_vec_vec_bing_che {
                                for k in j {
                                    if xt_temp.ji_zu_vec[k].common_ji.current_range != JiZuRangeLeiXing::Wen {
                                        is_all_ji_zu_wen = false;
                                        break;
                                    }
                                }
                                if !is_all_ji_zu_wen {
                                    break;
                                }
                            }
                            if is_all_ji_zu_wen {
                                return true;
                            }
                            else {
                                return false;
                            }
                        }
                        else {
                            return false;
                        }
                    }
                    else {
                        return false;
                    }
                }
                else {
                    return false;
                }
            },
            ZhiLingType::FenZhaJieLie => {
                if self.dev_type == simctrl::DevType::JiZu && xt.ji_zu_vec[self.dev_id].common_ji.current_range == JiZuRangeLeiXing::Wen {
                    return true;
                }
                else if self.dev_type == simctrl::DevType::DuanLuQi {
                    let mut xt_temp = xt.clone();
                    xt_temp.duan_lu_qi_vec[self.dev_id].set_off();
                    xt_temp.update_node_group_vec();
                    let jizu_vec_vec_bing_che = xt_temp.compare_two_xi_tong_he_zha_ji_zu(xt);
                    if jizu_vec_vec_bing_che.len() == 2 {
                        if jizu_vec_vec_bing_che[0].len() == 0 || jizu_vec_vec_bing_che[1].len() == 0 {
                            return true;
                        }
                        else if jizu_vec_vec_bing_che[0].len() == 1 || jizu_vec_vec_bing_che[1].len() == 1 {
                            let mut is_all_ji_zu_wen = true;
                            for j in jizu_vec_vec_bing_che {
                                for k in j {
                                    if xt_temp.ji_zu_vec[k].common_ji.current_range != JiZuRangeLeiXing::Wen {
                                        is_all_ji_zu_wen = false;
                                        break;
                                    }
                                }
                                if !is_all_ji_zu_wen {
                                    break;
                                }
                            }
                            if is_all_ji_zu_wen {
                                return true;
                            }
                            else {
                                return false;
                            }
                        }
                        else {
                            return false;
                        }
                    }
                    else {
                        return false;
                    }
                }
                else {
                    return false;
                }
            },
            ZhiLingType::BeiChe => {
                if xt.ji_zu_vec[self.dev_id].common_ji.current_range == JiZuRangeLeiXing::TingJi {
                    return true;
                }
                else {
                    return false;
                }
            },
            ZhiLingType::BianZai(..) | ZhiLingType::ZhongZaiJiaZai(..) => {
                let ji_zu_bing_lian_group = xt.get_ji_zu_group_from_fu_zai_id(self.dev_id);
                let mut is_all_ji_zu_run = true;
                for j in ji_zu_bing_lian_group {
                    if xt.ji_zu_vec[j].common_ji.current_range != JiZuRangeLeiXing::Wen && xt.ji_zu_vec[j].common_ji.current_range != JiZuRangeLeiXing::BianSu {
                        println!("{:?}, {:?}", j, xt.ji_zu_vec[j].common_ji.current_range);
                        is_all_ji_zu_run = false;
                        break;
                    }
                }
                if is_all_ji_zu_run {
                    return true;
                }
                else {
                    return false;
                }
            },
            ZhiLingType::AnDianHeZha => {true},
            ZhiLingType::AnDianFenZha => {true},
            ZhiLingType::BianSu(..) => {
                if xt.ji_zu_vec[self.dev_id].common_ji.current_range == JiZuRangeLeiXing::Wen || xt.ji_zu_vec[self.dev_id].common_ji.current_range == JiZuRangeLeiXing::BianSu {
                    return true;
                }
                else {
                    return false;
                }
            },
            ZhiLingType::BianYa(..) => {
                if xt.ji_zu_vec[self.dev_id].common_ji.current_range == JiZuRangeLeiXing::Wen || xt.ji_zu_vec[self.dev_id].common_ji.current_range == JiZuRangeLeiXing::BianYa {
                    return true;
                }
                else {
                    return false;
                }
            },
            _ => true,
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum YingDaType {
    ACK(ZhiLing),
    Success(ZhiLing),
    Valid(ZhiLing),
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum YingDaErr {
    ZhongZaiAskFail(ZhiLing, String, String),
    BeiCheFail(ZhiLing, String, String),
    QiDongFail(ZhiLing, String, String),
    HeZhaBingCheFail(ZhiLing, String, String),
    FenZhaJieLieFail(ZhiLing, String, String),
    TingJiFail(ZhiLing, String, String),
    CtrlModeFail(ZhiLing, String, String),
    OperatingStationFail(ZhiLing, String, String),
    CtrlModeAndOperatingStationFail(ZhiLing, String, String),
    AnDianFail(ZhiLing, String, String),
    AnDianHeZhaFail(ZhiLing, String, String),
    AnDianFenZhaFail(ZhiLing, String, String),
    BianSuFail(ZhiLing, String, String),
    BianYaFail(ZhiLing, String, String),
    JinJiTingJiFail(ZhiLing, String, String),
    PrioFail(ZhiLing, String, String),
    TouRuFail(ZhiLing, String, String),
    TuiChuFail(ZhiLing, String, String),
    XiaoShengFail(ZhiLing, String, String),
    YingDaFail(ZhiLing, String, String),
    GuZhangHuanJiFail(ZhiLing, String, String),
    DevNotExist(ZhiLing, String, String),
    /// #####DevTypeNotMatch
    /// 表征指令内容与设备类型不匹配,这通常是由于指令发送方的错误造成的
    DevTypeNotMatch(ZhiLing, String, String),
    Invalid(ZhiLing, String, String),
    IdNotMatch(ZhiLing, String, String),
}

pub const CTRL_MODE_AND_OPERATING_STATION_FAIL_DESC : &'static str = "相应战位的操作人员发送的指令无效";
pub const CAUSE_CTRL_MODE_AND_OPERATING_STATION_INVALID : &'static str = "操作部位与控制方式无效";

pub const DEV_TYPE_NOT_MATCH_DESC : &'static str = "指令与设备类型不匹配";
pub const CAUSE_DEV_TYPE_NOT_MATCH : &'static str = "硬件或软件制造方的错误，或者是被黑客入侵";

pub const ID_NOT_MATCH_DESC : &'static str = "此ID号的设备不存在";
pub const CAUSE_ID_NOT_MATCH : &'static str = "硬件或软件制造方的错误，或者教练操控台指令错误，或者是恶意入侵造成";

pub const BEI_CHE_FAIL_DESC : &'static str = "备车失败";
pub const CAUSE_JI_ZU_RANGE_DISMATCH_1 : &'static str = "机组不处于停机状态";

pub const QI_DONG_FAIL_DESC : &'static str = "启动失败";
pub const CAUSE_DUAN_LU_QI_BI_HE_HUO_GU_ZHANG : &'static str = "机组断路器闭合或者故障";
pub const CAUSE_JI_ZU_RANGE_DISMATCH_2 : &'static str = "机组未完成备车或者已经启动";

pub const HE_ZHA_BING_CHE_FAIL_DESC : &'static str = "合闸/并车失败";
pub const CAUSE_JI_ZU_RANGE_DISMATCH_6 : &'static str = "机组不处于稳定运行状态";
pub const CAUSE_JI_ZU_RANGE_DISMATCH_7 : &'static str = "手动情况下应将待并机组调整至合适状态才能并车";
pub const CAUSE_XI_TONG_HUI_LU_EXIST : &'static str = "系统中存在回路";
pub const CAUSE_POWER_FLOW_ERR : &'static str = "系统拓扑分析算法存在问题，请修复";
pub const CAUSE_DUO_JI_ZU_TONG_SHI_BING_LIAN : &'static str = "不能一次同时将多台机组并到电网上";

pub const FEN_ZHA_JIE_LIE_FAIL_DESC : &'static str = "分闸/解列失败";
pub const CAUSE_DUAN_LU_QI_FEN_DUAN_HUO_GU_ZHANG : &'static str = "机组断路器已经分断或者故障";
pub const CAUSE_TUO_PU_FEN_XI_SUAN_FA_CUO_WU : &'static str = "系统拓扑分析算法有bug，请修正";
pub const CAUSE_FEN_ZHA_JIE_LIE_WEI_SHOU_DONG_JIAN_ZAI : &'static str = "应对机组进行手动减载直至额定功率的10%";
pub const CAUSE_FEN_ZHA_JIE_LIE_ZONG_GONG_LV_BU_GOU_YONG : &'static str = "解列后电网总输出功率不满足负载需求";

pub const TING_JI_FAIL_DESC : &'static str = "停机失败";
pub const CAUSE_JI_ZU_RANGE_DISMATCH_3 : &'static str = "机组不处于可停机状态";

pub const OPERATING_STATION_FAIL_DESC : &'static str = "操作部位设置失败";
pub const CAUSE_OPERATING_STATION_INVALID : &'static str = "设备操作部位不能设置为不合法的值";

pub const CTRL_MODE_FAIL_DESC : &'static str = "控制方式设置失败";
pub const CAUSE_CTRL_MODE_INVALID : &'static str = "控制方式不能设置为不合法的值";

pub const PRIO_FAIL_DESC : &'static str = "优先级设置失败";
pub const CAUSE_PRIO_INVALID : &'static str = "优先级不能设置为不合法的值";

pub const AN_DIAN_FAIL_DESC : &'static str = "岸电设置失败";
pub const CAUSE_AN_DIAN_INVALID : &'static str = "岸电指令不能设置为不合法的值";

pub const AN_DIAN_HE_ZHA_FAIL_DESC : &'static str = "岸电合闸失败";
pub const CAUSE_AN_DIAN_HE_ZHA_INVALID : &'static str = "岸电合闸指令不合法";
pub const CAUSE_AN_DIAN_YI_JING_HE_ZHA : &'static str = "此岸电已经合闸";
pub const CAUSE_AN_DIAN_YI_JING_HE_ZHA_IN_SAME_DIAN_ZHAN : &'static str = "同一电站中已经有岸电合闸";

pub const AN_DIAN_FEN_ZHA_FAIL_DESC : &'static str = "岸电分闸失败";
pub const CAUSE_AN_DIAN_FEN_ZHA_INVALID : &'static str = "岸电分闸指令不合法";
pub const CAUSE_AN_DIAN_YI_JING_FEN_ZHA : &'static str = "此岸电已经分闸";

pub const COMMON_INVALID_DESC : &'static str = "指令无效";
pub const CAUSE_COMMON_INVALID : &'static str = "指令类型不合法，或者指令类型与设备类型不匹配，或者指令操作部位不满足当前的控制方式";

pub const BIAN_SU_FAIL_DESC : &'static str = "变速指令执行失败";
pub const CAUSE_JI_ZU_RANGE_DISMATCH_4 : &'static str = "机组不处于可变速状态，一般当且仅当机组处于稳态或者变速状态时才可以变速";
pub const CAUSE_BIAN_SU_FAIL_OUT_OF_LIMIT : &'static str = "机组转速值已达到极限，不可再进行调速";

pub const BIAN_YA_FAIL_DESC : &'static str = "变压指令执行失败";
pub const CAUSE_JI_ZU_RANGE_DISMATCH_5 : &'static str = "机组不处于可变压状态，一般当且仅当机组处于稳态或者变压状态时才可以变压";
pub const CAUSE_BIAN_YA_FAIL_OUT_OF_LIMIT : &'static str = "机组电压值已达到极限，不可再进行调压";

pub const ZHONG_ZAI_ASK_FAIL_DESC : &'static str = "重载询问指令执行失败";
pub const CAUSE_ZHONG_ZAI_ASK_FAIL : &'static str = "加载过大，超出剩余功率裕量";

pub const JIN_JI_TING_JI_FAIL_DESC : &'static str = "紧急停机指令执行失败";
pub const CAUSE_JIN_JI_TING_JI_FAIL : &'static str = "机组已停机或者正在停机";

pub const XIAO_SHENG_FAIL_DESC : &'static str = "消声失败";
pub const CAUSE_XIAO_SHENG_FAIL : &'static str = "未产生报警或者报警声已消除";

pub const YING_DA_FAIL_DESC : &'static str = "应答失败";
pub const CAUSE_YING_DA_FAIL : &'static str = "未产生报警或者已应答";
