use simctrl;
use std::string::String;
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
pub const CAUSE_ID_NOT_MATCH : &'static str = "硬件或软件制造方的错误，或者指令错误，可能是恶意入侵造成";

pub const BEI_CHE_FAIL_DESC : &'static str = "备车失败";
pub const CAUSE_JI_ZU_RANGE_DISMATCH_1 : &'static str = "机组不处于停机状态";

pub const QI_DONG_FAIL_DESC : &'static str = "启动失败";
pub const CAUSE_DUAN_LU_QI_STATUS_DISMATCH_1 : &'static str = "机组断路器闭合或者故障";
pub const CAUSE_JI_ZU_RANGE_DISMATCH_2 : &'static str = "机组未完成备车或者已经启动";

pub const HE_ZHA_BING_CHE_FAIL_DESC : &'static str = "合闸/并车失败";
pub const CAUSE_JI_ZU_RANGE_DISMATCH_6 : &'static str = "机组不处于稳态";

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

pub const AN_DIAN_FEN_ZHA_FAIL_DESC : &'static str = "岸电分闸失败";
pub const CAUSE_AN_DIAN_FEN_ZHA_INVALID : &'static str = "岸电分闸指令不合法";

pub const COMMON_INVALID_DESC : &'static str = "指令类型无效";
pub const CAUSE_COMMON_INVALID : &'static str = "指令类型不合法或者指令类型与设备类型不匹配";

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
