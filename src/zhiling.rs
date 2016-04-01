use simctrl;
#[derive(PartialEq, Copy, Clone, Debug)]
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
    JiaZai(f64, f64),
    JianZai(f64, f64),
    ZhongZaiJiaZai(f64, f64),
    AnDianOn,
    AnDianOff,
    AnDianHeZha,
    AnDianFenZha,
    TouRu,
    TuiChu,
    JiaSu,
    JianSu,
    ShengYa,
    JiangYa,
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
#[derive(PartialEq, Copy, Clone, Debug)]
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
#[derive(PartialEq, Copy, Clone, Debug)]
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

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum YingDaType {
    ACK(ZhiLing),
    Success(ZhiLing),
    Valid(ZhiLing),
}
#[derive(PartialEq, Clone, Debug)]
pub enum YingDaErr<'a> {
    ZhongZaiAskFail(ZhiLing, &'a str, &'a str),
    BeiCheFail(ZhiLing, &'a str, &'a str),
    QiDongFail(ZhiLing, &'a str, &'a str),
    HeZhaBingCheFail(ZhiLing, &'a str, &'a str),
    FenZhaJieLieFail(ZhiLing, &'a str, &'a str),
    TingJiFail(ZhiLing, &'a str, &'a str),
    CtrlModeFail(ZhiLing, &'a str, &'a str),
    OperatingStationFail(ZhiLing, &'a str, &'a str),
    CtrlModeAndOperatingStationFail(ZhiLing, &'a str, &'a str),
    AnDianFail(ZhiLing, &'a str, &'a str),
    AnDianHeZhaFail(ZhiLing, &'a str, &'a str),
    AnDianFenZhaFail(ZhiLing, &'a str, &'a str),
    JiaSuFail(ZhiLing, &'a str, &'a str),
    JianSuFail(ZhiLing, &'a str, &'a str),
    ShengYaFail(ZhiLing, &'a str, &'a str),
    JiangYaFail(ZhiLing, &'a str, &'a str),
    JinJiTingJiFail(ZhiLing, &'a str, &'a str),
    PrioFail(ZhiLing, &'a str, &'a str),
    TouRuFail(ZhiLing, &'a str, &'a str),
    TuiChuFail(ZhiLing, &'a str, &'a str),
    XiaoShengFail(ZhiLing, &'a str, &'a str),
    YingDaFail(ZhiLing, &'a str, &'a str),
    GuZhangHuanJiFail(ZhiLing, &'a str, &'a str),
    DevNotExist(ZhiLing, &'a str, &'a str),
    /// #####DevTypeNotMatch
    /// 表征指令内容与设备类型不匹配,这通常是由于指令发送方的错误造成的
    DevTypeNotMatch(ZhiLing, &'a str, &'a str),
    Invalid(ZhiLing, &'a str, &'a str),
    IdNotMatch(ZhiLing, &'a str, &'a str),
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
