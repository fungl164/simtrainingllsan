use simctrl::FANG_ZHEN_BU_CHANG;
use util::zan_tai_linear;
use simctrl;
//use std::string::String;
//电参数集
pub const JI_ZU_QI_DONG_TA:f64 = 6000.0f64;
pub const JI_ZU_QI_DONG_TB:f64 =  11000f64;
pub const DIAN_YA_ZAO_SHENG_XIANG_DUI_FU_ZHI:f64 =  0.05f64;
pub const PIN_LV_ZAO_SHENG_XIANG_DUI_FU_ZHI:f64 =  0.01f64;
pub const JI_ZU_E_DING_DIAN_YA:f64 =  395.0f64;
pub const JI_ZU_E_DING_PIN_LV:f64 =  50.0f64;
pub const JI_ZU_E_DING_GONG_LV:f64 =  1600.0f64;
pub const JI_ZU_E_DING_WU_GONG_GONG_LV:f64 =  1280.0f64;
pub const JI_ZU_E_DING_DIAN_LIU:f64 =  2886.75f64;
pub const JI_ZU_QI_DONG_SHENG_YU_DIAN_YA:f64 =  15.0f64;
pub const JI_ZU_SHU_CHU_GONG_LV_YIN_SHU:f64 =  0.80f64;
pub const JI_ZU_P_FACTOR:f64 =  JI_ZU_SHU_CHU_GONG_LV_YIN_SHU;
pub const JI_ZU_Q_FACTOR:f64 =  0.6f64;
pub const JI_ZU_Q_P:f64 =  0.75f64;
pub const GONG_LV_YIN_SHU_ZAO_SHENG_XIANG_DUI_FU_ZHI:f64 =  0.05f64;
//机组备车完毕后的最大空闲时间，超过此时间将转入停机状��?
pub const JI_ZU_BEI_CHE_WAN_BI_FREE_T : u32 = 900000u32;
//转移负载后机组停机总时��?
pub const JI_ZU_TING_JI_T : u32 = 7000u32;
//机组解列后的变载时间
pub const JI_ZU_JIE_LIE_HOU_BIAN_ZAI_T : u32 = 200u32;

//机组负载变化时的暂态过渡时间参��?
pub const JI_ZU_BIAN_ZAI_T : u32 = 3000u32;

//机组单位时间转速变化率，即调速的速率,每秒钟可增加多少��?
pub const JI_ZU_ZHUAN_SU_BIAN_HUA_LV:f64 = 0.5f64;
//机组单位时间电压变化率，即调压的速率,每秒钟可增加多少电压
pub const JI_ZU_DIAN_YA_BIAN_HUA_LV:f64 = 1.0f64;

//柴油机备车时间参��?
pub const JI_ZU_CHAI_YOU_BEI_CHE_T : u32 = 12000u32;

//汽轮机备车时间参��?
pub const JI_ZU_QI_LUN_BEI_CHE_T : u32 = 30000u32;

//手动并车调频时间到并车时间之间到间隔
pub const JI_ZU_SHOU_DONG_BING_CHE_JIAN_XI_T : u32 = 5000u32;
//手动并车合闸时调频上��?
pub const JI_ZU_SHOU_DONG_BING_CHE_PIN_LV_SHANG_XIAN:f64 = 50.25f64;
//手动并车合闸时调频下��?
pub const JI_ZU_SHOU_DONG_BING_CHE_PIN_LV_XIA_XIAN:f64 = 50.15f64;

//机组变载��? 电压变化��?/变载��? 系数
pub const DIAN_YA_BIAN_HUA_LV_XI_SHU:f64 = 0.05f64;

//机组稳态时频率变化阈��?
pub const JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI:f64 = 0.25f64;

//机组稳态时转速变化阈��?
pub const JI_ZU_ZHUAN_SU_BIAN_HUA_YU_ZHI_WEN_TAI:f64 = 7.5f64;

//电压稳态最小阈��?
pub const JI_ZU_DIAN_YA_WEN_TAI_ZUI_XIAO_YU_ZHI:f64 = 378.0f64;

//电压稳态最大阈��?
pub const JI_ZU_DIAN_YA_WEN_TAI_ZUI_DA_YU_ZHI:f64 = 400.0f64;

//负载从空载至满载，电压压降��?(395-378 = 17.0)
pub const JI_ZU_DIAN_YA_WEN_TAI_YA_BIAN_ZHI:f64 = -17.0f64;

//单次变速指令转速增��?
pub const JI_ZU_ZHUAN_SU_ZENG_LIANG_DAN_CI_ZHI_LING:f64 = 0.1f64;
//单次变压指令电压增量
pub const JI_ZU_DIAN_YA_ZENG_LIANG_DAN_CI_ZHI_LING:f64 = 0.1f64;

//过期参数，最好不��?
pub const V_NOISE_AMPLITUDE:f64 = DIAN_YA_ZAO_SHENG_XIANG_DUI_FU_ZHI;
pub const F_NOISE_AMPLITUDE:f64 = PIN_LV_ZAO_SHENG_XIANG_DUI_FU_ZHI;
pub const GEN_START_T1:f64 = JI_ZU_QI_DONG_TA;
pub const GEN_START_T2:f64 = JI_ZU_QI_DONG_TB;
pub const GEN_V_E_DING:f64 = JI_ZU_E_DING_DIAN_YA;
pub const GEN_F_E_DING:f64 = JI_ZU_E_DING_PIN_LV;
pub const GEN_V_START_REMAINDER:f64 = JI_ZU_QI_DONG_SHENG_YU_DIAN_YA;

//柴油机组机参��?
pub const HAI_SHUI_YA_LI_CHAI_YOU_ZHENG_CHANG:f64 = 0.29f64;
pub const HUA_YOU_WEN_DU_CHAI_YOU_ZHENG_CHANG:f64 = 75.0f64;
pub const HUA_YOU_YA_LI_CHAI_YOU_ZHENG_CHANG:f64 = 0.49f64;
pub const JI_YOU_WEN_DU_CHAI_YOU_ZHENG_CHANG:f64 = 70.0f64;
pub const JI_YOU_YA_LI_CHAI_YOU_ZHENG_CHANG:f64 = 0.8f64;
pub const LENG_QUE_SHUI_WEN_DU_CHAI_YOU_ZHENG_CHANG:f64 = 73.0f64;
pub const PAI_QI_WEN_DU_CHAI_YOU_ZHENG_CHANG:f64 = 300.0f64;
pub const QI_DONG_KONG_QI_YA_LI_CHAI_YOU_ZHENG_CHANG:f64 = 2.5f64;
pub const QU_ZHOU_XIANG_YA_LI_CHAI_YOU_ZHENG_CHANG:f64 = 1.0f64;
pub const RAN_YOU_YA_LI_CHAI_YOU_ZHENG_CHANG:f64 = 1.0f64;
pub const RAO_ZU_WEN_DU_CHAI_YOU_ZHENG_CHANG:f64 = 80.0f64;
pub const ZENG_YA_KONG_QI_WEN_DU_CHAI_YOU_ZHENG_CHANG:f64 = 50.0f64;
pub const ZHOU_CHENG_HUA_YOU_LIU_LIANG_CHAI_YOU_ZHENG_CHANG:f64 = 12.0f64;
pub const ZHOU_CHENG_WEN_DU_CHAI_YOU_ZHENG_CHANG:f64 = 40.0f64;
pub const ZHOU_CHENG_YU_GONG_HUA_YOU_LIU_LIANG_CHAI_YOU_ZHENG_CHANG:f64 = 9.0f64;
pub const ZHUAN_SU_CHAI_YOU_ZHENG_CHANG:f64 = 1500.0f64;
pub const JIN_FENG_WEN_DU_CHAI_YOU_ZHENG_CHANG:f64 = 50.0f64;
pub const CHU_FENG_WEN_DU_CHAI_YOU_ZHENG_CHANG:f64 = PAI_QI_WEN_DU_CHAI_YOU_ZHENG_CHANG;
pub const DAN_SHUI_YA_LI_CHAI_YOU_ZHENG_CHANG:f64 = 0.21f64;
pub const ZHAO_NEI_KONG_QI_WEN_DU_CHAI_YOU_ZHENG_CHANG:f64 = 50.0f64;

//用在机组停机��?
pub const WEN_DU_CHANG_WEN:f64 = 25.0f64;
pub const DA_QI_YA_LI : f64 = 1.0f64;

//机组故障阈��?
pub const DIAN_WANG_DIAN_YA_YUE_SHANG_XIAN:f64 = 1.1*JI_ZU_E_DING_DIAN_YA;
pub const DIAN_WANG_DIAN_YA_YUE_XIA_XIAN:f64 = (0.9*JI_ZU_E_DING_DIAN_YA);
pub const DIAN_WANG_PIN_LV_YUE_SHANG_XIAN:f64 = 52.5f64;
pub const DIAN_WANG_PIN_LV_YUE_XIA_XIAN:f64 = 47.5f64;

pub const JI_ZU_DIAN_YA_YUE_SHANG_XIAN:f64 = (1.1*JI_ZU_E_DING_DIAN_YA);
pub const JI_ZU_DIAN_YA_YUE_XIA_XIAN:f64 = (0.9*JI_ZU_E_DING_DIAN_YA);
pub const JI_ZU_PIN_LV_YUE_SHANG_XIAN:f64 = 52.5f64;
pub const JI_ZU_PIN_LV_YUE_XIA_XIAN:f64 = 47.5f64;

pub const KUA_JIE_DUAN_LU_QI_SAN_XIANG_DIAN_LIU_BU_PING_HENG:f64 = (0.15*JI_ZU_E_DING_DIAN_LIU);
pub const KUA_JIE_DUAN_LU_QI_DIAN_LIU_DA:f64 = (1.8*JI_ZU_E_DING_DIAN_LIU);
pub const JI_ZU_SAN_XIANG_DIAN_LIU_BU_PING_HENG:f64 = (0.15*JI_ZU_E_DING_DIAN_LIU);
pub const JI_ZU_NI_GONG_LV:f64 = (-0.07*JI_ZU_E_DING_GONG_LV);

//柴油机组报警阈��?
pub const QU_ZHOU_XIANG_YA_LI_CHAI_YOU_GAO:f64 = 1.1f64;
pub const HUA_YOU_WEN_DU_CHAI_YOU_GAO:f64 = 95.0f64;
pub const HUA_YOU_YA_LI_CHAI_YOU_DI:f64 = 0.30f64;
pub const HUA_YOU_YA_LI_CHAI_YOU_GUO_DI:f64 = 0.20f64;
pub const LENG_QUE_SHUI_WEN_DU_CHAI_YOU_GAO:f64 = 90.0f64;
pub const CHAO_SU_TING_JI:f64 = 1695.0f64;
pub const DAN_SHUI_YA_LI_DI:f64 = 0.025f64;
pub const QI_DONG_KONG_QI_YA_LI_CHAI_YOU_DI:f64 = 1.80f64;
pub const PAI_QI_WEN_DU_CHAI_YOU_GAO:f64 = 540.0f64;
pub const ZENG_YA_KONG_QI_WEN_DU_CHAI_YOU_GAO:f64 = 65.0f64;
pub const HAI_SHUI_YA_LI_CHAI_YOU_DI:f64 = 0.008f64;
pub const RAO_ZU_WEN_DU_CHAI_YOU_GAO:f64 = 160.0f64;
pub const RAO_ZU_WEN_DU_CHAI_YOU_GUO_GAO:f64 = 170.0f64;
pub const QIAN_ZHOU_CHENG_HUA_YOU_LIU_LIANG_CHAI_YOU_DI:f64 = 9.0f64;
pub const HOU_ZHOU_CHENG_HUA_YOU_LIU_LIANG_CHAI_YOU_DI:f64 = 4.0f64;
pub const HUA_YOU_WEN_DU_FA_DIAN_JI_GAO:f64 = 55.0f64;
pub const QIAN_ZHOU_CHENG_WEN_DU_FA_DIAN_JI_GAO:f64 = 80.0f64;
pub const HOU_ZHOU_CHENG_WEN_DU_FA_DIAN_JI_GAO:f64 = 80.0f64;
pub const QIAN_ZHOU_CHENG_YU_GONG_HUA_YOU_LIU_LIANG_CHAI_YOU_DI:f64 = 6.0f64;
pub const HOU_ZHOU_CHENG_YU_GONG_HUA_YOU_LIU_LIANG_CHAI_YOU_DI:f64 = 3.0f64;
pub const ZHAO_NEI_KONG_QI_WEN_DU_CHAI_YOU_GAO:f64 = 65.0f64;
pub const JIN_FENG_WEN_DU_CHAI_YOU_GAO:f64 = 65.0f64;
pub const CHU_FENG_WEN_DU_CHAI_YOU_GAO:f64 = PAI_QI_WEN_DU_CHAI_YOU_GAO;

//汽轮机组机参��?
pub const JIN_FENG_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 50.0f64;
pub const CHU_FENG_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 80.0f64;
pub const ZHUAN_SU_QI_LUN_ZHENG_CHANG:f64 = ZHUAN_SU_CHAI_YOU_ZHENG_CHANG;
pub const RUN_HUA_ZONG_GUAN_HUA_YOU_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 1.5f64;
pub const RUN_HUA_ZONG_GUAN_HUA_YOU_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 70.0f64;
pub const PAI_QI_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 50.0f64;
pub const RAO_ZU_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 50.0f64;
pub const ZHU_QI_MEN_ZHENG_QI_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 2.5f64;
pub const ZHU_QI_MEN_ZHENG_QI_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 100.0f64;
pub const CHOU_QI_QI_ZHENG_QI_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 2.5f64;
pub const CHOU_QI_QI_ZHENG_QI_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 100.0f64;
pub const QI_FENG_XI_TONG_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 2.5f64;
pub const QI_FENG_CHOU_QI_XI_TONG_ZHENG_QI_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 2.5f64;
pub const PAI_QI_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 2.5f64;
pub const JIAN_SU_QI_RUN_HUA_YOU_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 1.5f64;
pub const DIAN_DONG_NING_SHUI_BANG_CHU_KOU_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 1.5f64;
pub const TIAO_JIE_XI_TONG_HUA_YOU_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 1.5f64;
pub const LENG_QUE_HAI_SHUI_JIN_KOU_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = WEN_DU_CHANG_WEN;
pub const QI_LUN_JI_ZHOU_CHENG_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 50.0f64;
pub const FA_DIAN_JI_ZHOU_CHENG_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 50.0f64;
pub const YOU_XIANG_HUA_YOU_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 50.0f64;
pub const JIAN_SU_QI_ZHOU_CHENG_HUI_YOU_WEN_DU_QI_LUN_ZHENG_CHANG:f64 = 50.0f64;
pub const ZHU_YOU_QI_CHU_KOU_HUA_YOU_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 2.5f64;
pub const DI_YA_LV_YOU_QI_QIAN_HUA_YOU_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 1.5f64;
pub const GAO_YA_LV_YOU_QI_HOU_HUA_YOU_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 2.5f64;
pub const DIAN_DONG_YOU_BANG_CHU_KOU_HUA_YOU_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 1.5f64;
pub const ZHU_YOU_BANG_JIN_KOU_HUA_YOU_YA_LI_QI_LUN_ZHENG_CHANG:f64 = 1.5f64;

pub trait JiZuCtrl {
    fn update_bei_che_wan_bi_range<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn update_bei_che_zan_tai_range<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn update_bian_su_range<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn update_bian_ya_range<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn update_qi_dong_range<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn update_ting_ji_range<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn update_ting_ji_zan_tai_range<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn update_wen_range<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn transit_range<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn update_gu_zhang<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn gu_zhang_yi_ban_chan_sheng<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn gu_zhang_yi_ban_xiao_chu<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn gu_zhang_yi_ji_chan_sheng<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn gu_zhang_yi_ji_xiao_chu<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn gu_zhang_er_ji_chan_sheng<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn gu_zhang_er_ji_xiao_chu<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn gu_zhang_qi_ta_chan_sheng<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn gu_zhang_qi_ta_xiao_chu<'a>(&'a mut self, t:u32) -> &'a mut Self;
    fn update<'a>(&'a mut self, t:u32) -> &'a mut Self;
}
pub trait AttrSetter {
    fn ting_ji_setter(&mut self, t:u32);
    fn wen_setter(&mut self, t:u32);
    fn bei_che_wan_bi_setter(&mut self, t:u32);
    fn bei_che_zan_tai_setter(&mut self, t:u32);
    fn bian_su_setter(&mut self, t:u32);
    fn bian_ya_setter(&mut self, t:u32);
    fn qi_dong_setter(&mut self, t:u32);
    fn ting_ji_zan_tai_setter(&mut self, t:u32);
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct JiZuCommonJi {
    pub ia_ext : f64,
    pub ib_ext : f64,
    pub ic_ext : f64,
    pub f_ext : f64,
    pub ia_in : f64,
    pub ib_in : f64,
    pub ic_in : f64,
    pub f_in : f64,
    pub p : f64,
    pub q : f64,
    pub p_factor : f64,
    pub q_factor : f64,
    pub uab_ext : f64,
    pub ubc_ext : f64,
    pub uca_ext : f64,
    pub uab_in : f64,
    pub ubc_in : f64,
    pub uca_in : f64,
    pub zhuan_su : f64,

    pub ctrl_mode : simctrl::CtrlMode,
    pub jia_su : bool,
    pub jian_su : bool,
    pub yun_xing_zhuang_tai : JiZuYunXingZhuangTai,
    pub bei_che_wan_bi : bool,
    pub tong_bu : bool,
    pub bing_che : bool,
    pub qi_dong : bool,
    pub ting_ji : bool,
    pub operating_station : simctrl::OperatingStation,
    pub zhuang_tai_chuan_kou_1 : bool,
    pub zhuang_tai_chuan_kou_2 : bool,
    pub zhuang_tai_can_1 : bool,
    pub zhuang_tai_can_2 : bool,
    pub zhuang_tai_zhi_neng_io : bool,
    pub sheng_ya : bool,
    pub jiang_ya : bool,
    pub he_zha : bool,
    pub fen_duan : bool,
    pub prio : bool,
    pub ctrl_mode_she_zhi : simctrl::CtrlMode,
    pub operating_station_she_zhi : simctrl::OperatingStation,
    pub is_online : bool, // 指示机组是否在网

    pub gu_zhang_lei_xing : GuZhangLeiXing,
    pub ran_you_xie_lou : bool,
    pub qu_zhou_xiang_ya_li_gao : bool,
    pub qu_zhou_xiang_kong_qi_ya_li_di : bool,
    pub fa_dong_ji_leng_que_qi_hai_shui_xie_lou : bool,
    pub pai_qi_wen_du_gao : bool,
    pub zeng_ya_kong_qi_wen_du_gao : bool,
    pub qian_zhou_cheng_hua_you_liu_liang_di : bool,
    pub hou_zhou_cheng_hua_you_liu_liang_di : bool,
    pub hua_you_wen_du_gao : bool,
    pub hai_shui_ya_li_di : bool,
    pub peng_zhang_shui_xiang_ye_wei_di : bool,
    pub qian_zhou_cheng_wen_du_gao : bool,
    pub hou_zhou_cheng_wen_du_gao : bool,
    pub pin_lv_yue_xian : bool,
    pub ni_gong_lv : bool,
    pub guo_dian_ya : bool,
    pub qian_dian_ya : bool,
    pub guo_dian_liu : bool,
    pub san_xiang_dian_ya_bu_ping_heng : bool,
    pub wai_bu_duan_lu : bool,
    pub duan_lu_qi_yu_bao_jing : bool,
    pub qi_dong_shi_bai : bool,
    pub ting_ji_shi_bai : bool,
    pub duan_lu_qi_zong_he_gu_zhang : bool,
    pub nei_bu_duan_lu : bool,
    pub hua_you_ya_li_guo_di : bool,
    pub chao_su_ting_ji : bool,
    pub qian_zhou_cheng_yu_gong_hua_you_liu_liang_di : bool,
    pub hou_zhou_cheng_yu_gong_hua_you_liu_liang_di : bool,
    pub chai_you_ji_jin_ji_ting_ji : bool,
    pub ji_pang_shou_dong_ying_ji_ting_ji : bool,
    pub pai_qi_ya_li_guo_gao : bool,
    pub bing_che_shi_bai : bool,
    pub rao_zu_wen_du_gao : bool,
    pub hua_you_ya_li_di : bool,
    pub leng_que_shui_wen_du_gao : bool,
    pub rao_zu_wen_du_gao_tiao_zha_ting_ji : bool,
    pub pai_qi_ya_li_gao : bool,
    pub jian_su_qi_run_hua_you_ya_li_di : bool,
    pub qi_lun_ji_hou_zhou_cheng_wen_du_gao : bool,
    pub fa_dian_ji_hou_zhou_cheng_wen_du_gao : bool,
    pub you_xiang_wen_du_gao : bool,
    pub fa_dian_ji_leng_que_shui_xie_lou : bool,
    pub zhao_nei_shi_huo_jin_ji_ting_ji : bool,
    pub qi_nang_ge_zhen_er_ji_gu_zhang : bool,
    pub zhao_nei_huo_jing : bool,
    pub dan_shui_ya_li_di : bool,
    pub wu_zhuo_cao_ye_wei_gao : bool,
    pub zhao_nei_feng_ji_gu_zhang : bool,
    pub xiao_fang_she_bei_gu_zhang : bool,
    pub qi_nang_ge_zhen_yi_ban_gu_zhang : bool,
    pub ji_dai_hua_you_bang_gu_zhang : bool,
    pub jin_feng_wen_du_gao : bool,
    pub chu_feng_wen_du_gao : bool,
    pub wu_you_cao_ye_wei_gao : bool,
    pub zong_he_gu_zhang_bao_jing : bool,

    pub current_range : JiZuRangeLeiXing,
    pub bian_zai_lei_xing : JiZuBianZaiLeiXing,
    pub f_ref: f64,
    pub u_ref: f64,
    pub bian_su_t : u32,
    pub bian_ya_t : u32,
    pub bian_zai_t : u32,
    pub t_current_range : u32,
    pub i_delta : f64,
    pub u_delta : f64,
    pub f_delta : f64,
}
impl JiZuCommonJi {
    pub fn new() -> JiZuCommonJi {
        JiZuCommonJi {
            ia_ext : 0.0f64,
            ib_ext : 0.0f64,
            ic_ext : 0.0f64,
            f_ext : 0.0f64,
            ia_in : 0.0f64,
            ib_in : 0.0f64,
            ic_in : 0.0f64,
            f_in : 0.0f64,
            p : 0.0f64,
            q : 0.0f64,
            p_factor : 0.0f64,
            q_factor : 1.0f64,
            uab_ext : 0.0f64,
            ubc_ext : 0.0f64,
            uca_ext : 0.0f64,
            uab_in : 0.0f64,
            ubc_in : 0.0f64,
            uca_in : 0.0f64,
            zhuan_su : 0.0f64,

            f_ref : 0.0f64,
            u_ref : 0.0f64,
            bian_su_t : u32::max_value(),
            bian_ya_t : u32::max_value(),
            bian_zai_t : u32::max_value(),
            t_current_range : 0u32,
            i_delta : 0.0f64,
            u_delta : 0.0f64,
            f_delta : 0.0f64,
            ctrl_mode : simctrl::CtrlMode::Manual,
            jia_su : false,
            jian_su : false,
            yun_xing_zhuang_tai : JiZuYunXingZhuangTai::TingJi,
            bei_che_wan_bi : false,
            tong_bu : false,
            bing_che : false,
            qi_dong : false,
            ting_ji : false,
            operating_station : simctrl::OperatingStation::Remote,
            zhuang_tai_chuan_kou_1 : false,
            zhuang_tai_chuan_kou_2 : false,
            zhuang_tai_can_1 : false,
            zhuang_tai_can_2 : false,
            zhuang_tai_zhi_neng_io : false,
            sheng_ya : false,
            jiang_ya : false,
            he_zha : false,
            fen_duan : false,
            prio : false,
            ctrl_mode_she_zhi : simctrl::CtrlMode::Manual,
            operating_station_she_zhi : simctrl::OperatingStation::Remote,
            is_online : false,
            gu_zhang_lei_xing : GuZhangLeiXing::Wu,
            ran_you_xie_lou : false,
            qu_zhou_xiang_ya_li_gao : false,
            qu_zhou_xiang_kong_qi_ya_li_di : false,
            fa_dong_ji_leng_que_qi_hai_shui_xie_lou : false,
            pai_qi_wen_du_gao : false,
            zeng_ya_kong_qi_wen_du_gao : false,
            qian_zhou_cheng_hua_you_liu_liang_di : false,
            hou_zhou_cheng_hua_you_liu_liang_di : false,
            hua_you_wen_du_gao : false,
            hai_shui_ya_li_di : false,
            peng_zhang_shui_xiang_ye_wei_di : false,
            qian_zhou_cheng_wen_du_gao : false,
            hou_zhou_cheng_wen_du_gao : false,
            pin_lv_yue_xian : false,
            ni_gong_lv : false,
            guo_dian_ya : false,
            qian_dian_ya : false,
            guo_dian_liu : false,
            san_xiang_dian_ya_bu_ping_heng : false,
            wai_bu_duan_lu : false,
            duan_lu_qi_yu_bao_jing : false,
            qi_dong_shi_bai : false,
            ting_ji_shi_bai : false,
            duan_lu_qi_zong_he_gu_zhang : false,
            nei_bu_duan_lu : false,
            hua_you_ya_li_guo_di : false,
            chao_su_ting_ji : false,
            qian_zhou_cheng_yu_gong_hua_you_liu_liang_di : false,
            hou_zhou_cheng_yu_gong_hua_you_liu_liang_di : false,
            chai_you_ji_jin_ji_ting_ji : false,
            ji_pang_shou_dong_ying_ji_ting_ji : false,
            pai_qi_ya_li_guo_gao : false,
            bing_che_shi_bai : false,
            rao_zu_wen_du_gao : false,
            hua_you_ya_li_di : false,
            leng_que_shui_wen_du_gao : false,
            rao_zu_wen_du_gao_tiao_zha_ting_ji : false,
            pai_qi_ya_li_gao : false,
            jian_su_qi_run_hua_you_ya_li_di : false,
            qi_lun_ji_hou_zhou_cheng_wen_du_gao : false,
            fa_dian_ji_hou_zhou_cheng_wen_du_gao : false,
            you_xiang_wen_du_gao : false,
            fa_dian_ji_leng_que_shui_xie_lou : false,
            zhao_nei_shi_huo_jin_ji_ting_ji : false,
            qi_nang_ge_zhen_er_ji_gu_zhang : false,
            zhao_nei_huo_jing : false,
            dan_shui_ya_li_di : false,
            wu_zhuo_cao_ye_wei_gao : false,
            zhao_nei_feng_ji_gu_zhang : false,
            xiao_fang_she_bei_gu_zhang : false,
            qi_nang_ge_zhen_yi_ban_gu_zhang : false,
            ji_dai_hua_you_bang_gu_zhang : false,
            jin_feng_wen_du_gao : false,
            chu_feng_wen_du_gao : false,
            wu_you_cao_ye_wei_gao : false,
            zong_he_gu_zhang_bao_jing : false,
            current_range:JiZuRangeLeiXing::TingJi,
            bian_zai_lei_xing:JiZuBianZaiLeiXing::Wu,
         }
    }
}
impl AttrSetter for JiZuCommonJi {
    fn ting_ji_setter(&mut self, _t:u32) {
        self.ia_ext = 0.0f64;
        self.ib_ext = 0.0f64;
        self.ic_ext = 0.0f64;
        self.f_ext = 0.0f64;
        self.ia_in = 0.0f64;
        self.ib_in = 0.0f64;
        self.ic_in = 0.0f64;
        self.f_in = 0.0f64;
        self.p = 0.0f64;
        self.q = 0.0f64;
        self.p_factor = 0.0f64;
        self.q_factor = 1.0f64;
        self.uab_ext = 0.0f64;
        self.ubc_ext = 0.0f64;
        self.uca_ext = 0.0f64;
        self.uab_in = 0.0f64;
        self.ubc_in = 0.0f64;
        self.uca_in = 0.0f64;
        self.f_ref = 0.0f64;
        self.u_ref = 0.0f64;
        self.bian_su_t = u32::max_value();
        self.bian_ya_t = u32::max_value();
        self.bian_zai_t = u32::max_value();
        self.i_delta = 0.0f64;
        self.u_delta = 0.0f64;
        self.f_delta = 0.0f64;
    }
    fn wen_setter(&mut self, _t:u32) {
        self.ia_ext = 0.0f64;
        self.ib_ext = 0.0f64;
        self.ic_ext = 0.0f64;
        self.f_ext = JI_ZU_E_DING_PIN_LV;
        self.ia_in = 0.0f64;
        self.ib_in = 0.0f64;
        self.ic_in = 0.0f64;
        self.f_in = JI_ZU_E_DING_PIN_LV;
        self.p = 0.0f64;
        self.q = 0.0f64;
        self.p_factor = JI_ZU_SHU_CHU_GONG_LV_YIN_SHU;
        self.q_factor = (1.0f64-JI_ZU_SHU_CHU_GONG_LV_YIN_SHU*
            JI_ZU_SHU_CHU_GONG_LV_YIN_SHU).sqrt();
        self.uab_ext = JI_ZU_E_DING_DIAN_YA;
        self.ubc_ext = JI_ZU_E_DING_DIAN_YA;
        self.uca_ext = JI_ZU_E_DING_DIAN_YA;
        self.uab_in = JI_ZU_E_DING_DIAN_YA;
        self.ubc_in = JI_ZU_E_DING_DIAN_YA;
        self.uca_in = JI_ZU_E_DING_DIAN_YA;
        self.f_ref = 0.0f64;
        self.u_ref = 0.0f64;
        self.bian_su_t = u32::max_value();
        self.bian_ya_t = u32::max_value();
        self.bian_zai_t = u32::max_value();
        self.i_delta = 0.0f64;
        self.u_delta = 0.0f64;
        self.f_delta = 0.0f64;
    }
    fn bei_che_wan_bi_setter(&mut self, t:u32){
        self.ting_ji_setter(t);
    }
    fn bei_che_zan_tai_setter(&mut self, t:u32){
        self.ting_ji_setter(t);
    }
    fn bian_su_setter(&mut self, _t:u32){
        self.ia_ext += self.i_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.ia_in += self.i_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.ib_ext += self.i_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.ib_in += self.i_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.ic_ext += self.i_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.ic_in += self.i_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        //电压变化与负载相反，负载增加时电压会有一个短时跌落，反之亦然
        self.uab_ext += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.ubc_ext += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.uca_ext += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.uab_in += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.ubc_in += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.uca_in += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.p_factor = JI_ZU_SHU_CHU_GONG_LV_YIN_SHU;
        self.q_factor = (1.0f64-self.p_factor*self.p_factor).sqrt();
        self.p += 3.0f64.sqrt()*self.uab_ext*self.ia_ext*self.p_factor/1000.0f64;
        self.q += 3.0f64.sqrt()*self.uab_ext*self.ia_ext*self.q_factor/1000.0f64;
        self.f_ext += self.f_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
        self.f_in += self.f_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_su_t as f64;
    }
    fn bian_ya_setter(&mut self, _t:u32){
        self.uab_ext += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_ya_t as f64;
        self.ubc_ext += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_ya_t as f64;
        self.uca_ext += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_ya_t as f64;
        self.uab_in += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_ya_t as f64;
        self.ubc_in += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_ya_t as f64;
        self.uca_in += self.u_delta * FANG_ZHEN_BU_CHANG as f64/self.bian_ya_t as f64;
    }
    fn qi_dong_setter(&mut self, t:u32){
        //启动时电流功率均为零
        self.p_factor = JI_ZU_SHU_CHU_GONG_LV_YIN_SHU;
        if t as f64 <= JI_ZU_QI_DONG_TA {
            self.uab_ext = GEN_V_START_REMAINDER;
            self.ubc_ext = GEN_V_START_REMAINDER;
            self.uca_ext = GEN_V_START_REMAINDER;
            self.uab_in = GEN_V_START_REMAINDER;
            self.ubc_in = GEN_V_START_REMAINDER;
            self.uca_in = GEN_V_START_REMAINDER;
            self.f_ext = zan_tai_linear(t as f64, 0.0,
                JI_ZU_E_DING_PIN_LV-JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI,
                0.0, JI_ZU_QI_DONG_TA);
            self.f_in = zan_tai_linear(t as f64, 0.0,
                JI_ZU_E_DING_PIN_LV-JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI,
                0.0, JI_ZU_QI_DONG_TA);
        }
        else if (t as f64) > JI_ZU_QI_DONG_TA && (t as f64) < JI_ZU_QI_DONG_TB {
            self.uab_ext = zan_tai_linear(t as f64,
                GEN_V_START_REMAINDER, GEN_V_E_DING,
                JI_ZU_QI_DONG_TA, JI_ZU_QI_DONG_TB);
            self.ubc_ext = zan_tai_linear(t as f64,
                GEN_V_START_REMAINDER, GEN_V_E_DING,
                JI_ZU_QI_DONG_TA, JI_ZU_QI_DONG_TB);
            self.uca_ext = zan_tai_linear(t as f64,
                GEN_V_START_REMAINDER, GEN_V_E_DING,
                JI_ZU_QI_DONG_TA, JI_ZU_QI_DONG_TB);
            self.uab_in = zan_tai_linear(t as f64,
                GEN_V_START_REMAINDER, GEN_V_E_DING,
                JI_ZU_QI_DONG_TA, JI_ZU_QI_DONG_TB);
            self.ubc_in = zan_tai_linear(t as f64,
                GEN_V_START_REMAINDER, GEN_V_E_DING,
                JI_ZU_QI_DONG_TA, JI_ZU_QI_DONG_TB);
            self.uca_in = zan_tai_linear(t as f64,
                GEN_V_START_REMAINDER, GEN_V_E_DING,
                JI_ZU_QI_DONG_TA, JI_ZU_QI_DONG_TB);
            self.f_ext = JI_ZU_E_DING_PIN_LV-JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI;
            self.f_in = JI_ZU_E_DING_PIN_LV-JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI;
        }
    }
    fn ting_ji_zan_tai_setter(&mut self, t:u32){
        self.sheng_ya = false;
        self.jiang_ya = true;
        self.jian_su = true;
        self.jia_su = false;
        self.bing_che = false;
        self.tong_bu = false;
        self.fen_duan = true;
        self.he_zha = false;
        self.p_factor = JI_ZU_SHU_CHU_GONG_LV_YIN_SHU;
        self.uab_ext = zan_tai_linear(t as f64,
            GEN_V_E_DING, 0.0, 0.0, JI_ZU_TING_JI_T as f64);
        self.ubc_ext = self.uab_ext;
        self.uca_ext = self.uab_ext;
        self.uab_in = self.uab_ext;
        self.ubc_in = self.uab_ext;
        self.uca_in = self.uab_ext;

        self.f_ext = zan_tai_linear(t as f64,
            JI_ZU_E_DING_PIN_LV, 0.0, 0.0, JI_ZU_TING_JI_T as f64);
        self.f_in = self.f_ext;
        self.zhuan_su = ZHUAN_SU_CHAI_YOU_ZHENG_CHANG /
        JI_ZU_E_DING_PIN_LV * self.f_ext;
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum JiZuYunXingZhuangTai{
    TingJi,
    YunXing,
    JiuXu,
    GuZhang,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum GuZhangLeiXing {
    YiBan,
    YiJi,
    ErJi,
    QiTa,
    Wu,
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct ChaiYouJiJi{
    pub leng_que_shui_wen_du_a_lie : f64,
    pub leng_que_shui_wen_du_b_lie : f64,
    pub zhou_cheng_wen_du_qian : f64,
    pub zhou_cheng_wen_du_hou : f64,
    pub jin_feng_wen_du : f64,
    pub chu_feng_wen_du : f64,
    pub hua_you_ya_li : f64,
    pub hua_you_wen_du : f64,
    pub zhou_cheng_hua_you_liu_liang_qian : f64,
    pub zhou_cheng_hua_you_liu_liang_hou : f64,
    pub zeng_ya_kong_qi_wen_du_a_lie : f64,
    pub zeng_ya_kong_qi_wen_du_b_lie : f64,
    pub pai_qi_wen_du_a_lie : f64,
    pub pai_qi_wen_du_b_lie : f64,
    pub qu_zhou_xiang_ya_li : f64,
    pub ran_you_ya_li : f64,
    pub hai_shui_ya_li : f64,
    pub ji_you_ya_li : f64,
    pub ji_you_wen_du : f64,
    pub qi_dong_kong_qi_ya_li : f64,
    pub rao_zu_wen_du_a : f64,
    pub rao_zu_wen_du_b : f64,
    pub rao_zu_wen_du_c : f64,
    pub dan_shui_wen_du_a_lie : f64,
    pub dan_shui_wen_du_b_lie : f64,
    pub dan_shui_ya_li : f64,
    pub hua_you_xiang_wen_du : f64,
    pub zhao_nei_kong_qi_wen_du : f64,
}
impl ChaiYouJiJi {
    pub fn new() -> ChaiYouJiJi {
        ChaiYouJiJi{
            hai_shui_ya_li : HAI_SHUI_YA_LI_CHAI_YOU_ZHENG_CHANG,
            hua_you_wen_du : WEN_DU_CHANG_WEN,
            hua_you_ya_li : HUA_YOU_YA_LI_CHAI_YOU_ZHENG_CHANG,
            ji_you_wen_du : WEN_DU_CHANG_WEN,
            ji_you_ya_li : JI_YOU_YA_LI_CHAI_YOU_ZHENG_CHANG,
            leng_que_shui_wen_du_a_lie : WEN_DU_CHANG_WEN,
            leng_que_shui_wen_du_b_lie : WEN_DU_CHANG_WEN,
            pai_qi_wen_du_a_lie :  WEN_DU_CHANG_WEN,
            pai_qi_wen_du_b_lie : WEN_DU_CHANG_WEN,
            qi_dong_kong_qi_ya_li : 0.0f64,
            qu_zhou_xiang_ya_li : QU_ZHOU_XIANG_YA_LI_CHAI_YOU_ZHENG_CHANG,
            ran_you_ya_li : RAN_YOU_YA_LI_CHAI_YOU_ZHENG_CHANG,
            rao_zu_wen_du_a : WEN_DU_CHANG_WEN,
            rao_zu_wen_du_b : WEN_DU_CHANG_WEN,
            rao_zu_wen_du_c : WEN_DU_CHANG_WEN,
            zeng_ya_kong_qi_wen_du_a_lie : WEN_DU_CHANG_WEN,
            zeng_ya_kong_qi_wen_du_b_lie : WEN_DU_CHANG_WEN,
            zhou_cheng_hua_you_liu_liang_hou : ZHOU_CHENG_HUA_YOU_LIU_LIANG_CHAI_YOU_ZHENG_CHANG,
            zhou_cheng_hua_you_liu_liang_qian : ZHOU_CHENG_HUA_YOU_LIU_LIANG_CHAI_YOU_ZHENG_CHANG,
            zhou_cheng_wen_du_hou : WEN_DU_CHANG_WEN,
            zhou_cheng_wen_du_qian : WEN_DU_CHANG_WEN,
            jin_feng_wen_du : WEN_DU_CHANG_WEN,
            chu_feng_wen_du : WEN_DU_CHANG_WEN,
            dan_shui_wen_du_a_lie : WEN_DU_CHANG_WEN,
            dan_shui_wen_du_b_lie : WEN_DU_CHANG_WEN,
            dan_shui_ya_li : DAN_SHUI_YA_LI_CHAI_YOU_ZHENG_CHANG,
            hua_you_xiang_wen_du : WEN_DU_CHANG_WEN,
            zhao_nei_kong_qi_wen_du : WEN_DU_CHANG_WEN,
        }
    }
}
impl AttrSetter for ChaiYouJiJi {
    fn ting_ji_setter(&mut self, _t:u32) {
    }
    fn wen_setter(&mut self, _t:u32) {
    }
    fn bei_che_wan_bi_setter(&mut self, _t:u32){
        self.ting_ji_setter(_t);
        self.ran_you_ya_li = RAN_YOU_YA_LI_CHAI_YOU_ZHENG_CHANG;
        self.qi_dong_kong_qi_ya_li = QI_DONG_KONG_QI_YA_LI_CHAI_YOU_ZHENG_CHANG;
    }
    fn bei_che_zan_tai_setter(&mut self, _t:u32){

    }
    fn bian_su_setter(&mut self, _t:u32){

    }
    fn bian_ya_setter(&mut self, _t:u32){

    }
    fn qi_dong_setter(&mut self, _t:u32){

    }
    fn ting_ji_zan_tai_setter(&mut self, _t:u32){

    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct QiLunJiJi{
    pub jin_feng_wen_du : f64,
    pub chu_feng_wen_du : f64,
    pub run_hua_zong_guan_hua_you_ya_li : f64,
    pub run_hua_zong_guan_hua_you_wen_du : f64,
    pub pai_qi_wen_du_a_lie : f64,
    pub pai_qi_wen_du_b_lie : f64,
    pub rao_zu_wen_du_a : f64,
    pub rao_zu_wen_du_b : f64,
    pub rao_zu_wen_du_c : f64,
    pub zhu_qi_men_zheng_qi_ya_li : f64,
    pub zhu_qi_men_zheng_qi_wen_du : f64,
    pub chou_qi_qi_zheng_qi_ya_li : f64,
    pub chou_qi_qi_zheng_qi_wen_du : f64,
    pub qi_feng_xi_tong_ya_li : f64,
    pub qi_feng_chou_qi_xi_tong_zheng_qi_ya_li : f64,
    pub pai_qi_ya_li : f64,
    pub jian_su_qi_run_hua_you_ya_li : f64,
    pub dian_dong_ning_shui_bang_chu_kou_ya_li_1 : f64,
    pub dian_dong_ning_shui_bang_chu_kou_ya_li_2 : f64,
    pub tiao_jie_xi_tong_hua_you_ya_li : f64,
    pub leng_que_hai_shui_jin_kou_wen_du : f64,
    pub qi_lun_ji_a_lie_zhou_cheng_wen_du : f64,
    pub qi_lun_ji_b_lie_zhou_cheng_wen_du : f64,
    pub fa_dian_ji_a_lie_zhou_cheng_wen_du : f64,
    pub fa_dian_ji_b_lie_zhou_cheng_wen_du : f64,
    pub you_xiang_hua_you_wen_du : f64,
    pub jian_su_qi_zhou_cheng_hui_you_wen_du : f64,
    pub zhu_you_qi_chu_kou_hua_you_ya_li : f64,
    pub di_ya_lv_you_qi_qian_hua_you_ya_li : f64,
    pub gao_ya_lv_you_qi_hou_hua_you_ya_li : f64,
    pub dian_dong_you_bang_chu_kou_hua_you_ya_li : f64,
    pub zhu_you_bang_jin_kou_hua_you_ya_li : f64,
}
impl QiLunJiJi {
    pub fn new() -> QiLunJiJi {
        QiLunJiJi{
            jin_feng_wen_du : WEN_DU_CHANG_WEN,
            chu_feng_wen_du : WEN_DU_CHANG_WEN,
            run_hua_zong_guan_hua_you_ya_li : DA_QI_YA_LI,
            run_hua_zong_guan_hua_you_wen_du : WEN_DU_CHANG_WEN,
            pai_qi_wen_du_a_lie : WEN_DU_CHANG_WEN,
            pai_qi_wen_du_b_lie : WEN_DU_CHANG_WEN,
            rao_zu_wen_du_a : WEN_DU_CHANG_WEN,
            rao_zu_wen_du_b : WEN_DU_CHANG_WEN,
            rao_zu_wen_du_c : WEN_DU_CHANG_WEN,
            zhu_qi_men_zheng_qi_ya_li : DA_QI_YA_LI,
            zhu_qi_men_zheng_qi_wen_du : WEN_DU_CHANG_WEN,
            chou_qi_qi_zheng_qi_wen_du : WEN_DU_CHANG_WEN,
            chou_qi_qi_zheng_qi_ya_li : DA_QI_YA_LI,
            qi_feng_xi_tong_ya_li : DA_QI_YA_LI,
            qi_feng_chou_qi_xi_tong_zheng_qi_ya_li : DA_QI_YA_LI,
            pai_qi_ya_li : DA_QI_YA_LI,
            jian_su_qi_run_hua_you_ya_li : DA_QI_YA_LI,
            dian_dong_ning_shui_bang_chu_kou_ya_li_1 : DA_QI_YA_LI,
            dian_dong_ning_shui_bang_chu_kou_ya_li_2 : DA_QI_YA_LI,
            tiao_jie_xi_tong_hua_you_ya_li : DA_QI_YA_LI,
            leng_que_hai_shui_jin_kou_wen_du : WEN_DU_CHANG_WEN,
            qi_lun_ji_a_lie_zhou_cheng_wen_du : WEN_DU_CHANG_WEN,
            qi_lun_ji_b_lie_zhou_cheng_wen_du : WEN_DU_CHANG_WEN,
            you_xiang_hua_you_wen_du : WEN_DU_CHANG_WEN,
            jian_su_qi_zhou_cheng_hui_you_wen_du : WEN_DU_CHANG_WEN,
            fa_dian_ji_a_lie_zhou_cheng_wen_du : WEN_DU_CHANG_WEN,
            fa_dian_ji_b_lie_zhou_cheng_wen_du : WEN_DU_CHANG_WEN,
            zhu_you_qi_chu_kou_hua_you_ya_li : DA_QI_YA_LI,
            di_ya_lv_you_qi_qian_hua_you_ya_li : DA_QI_YA_LI,
            gao_ya_lv_you_qi_hou_hua_you_ya_li : DA_QI_YA_LI,
            dian_dong_you_bang_chu_kou_hua_you_ya_li : DA_QI_YA_LI,
            zhu_you_bang_jin_kou_hua_you_ya_li : DA_QI_YA_LI,

        }
    }
}
impl AttrSetter for QiLunJiJi {
    fn ting_ji_setter(&mut self, _t:u32) {
    }
    fn wen_setter(&mut self, _t:u32) {
    }
    fn bei_che_wan_bi_setter(&mut self, t:u32){
        self.ting_ji_setter(t);
    }
    fn bei_che_zan_tai_setter(&mut self, _t:u32){

    }
    fn bian_su_setter(&mut self, _t:u32){

    }
    fn bian_ya_setter(&mut self, _t:u32){

    }
    fn qi_dong_setter(&mut self, _t:u32){

    }
    fn ting_ji_zan_tai_setter(&mut self, _t:u32){

    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct AnDianJiJi;
impl AttrSetter for AnDianJiJi {
    fn ting_ji_setter(&mut self, _t:u32) {
    }
    fn wen_setter(&mut self, _t:u32) {
    }
    fn bei_che_wan_bi_setter(&mut self, _t:u32){
    }
    fn bei_che_zan_tai_setter(&mut self, _t:u32){
    }
    fn bian_su_setter(&mut self, _t:u32){
    }
    fn bian_ya_setter(&mut self, _t:u32){
    }
    fn qi_dong_setter(&mut self, _t:u32){
    }
    fn ting_ji_zan_tai_setter(&mut self, _t:u32){
    }
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum JiZuRangeLeiXing {
    TingJi,
    BeiCheZanTai,
    BeiCheWanBi,
    QiDong,
    Wen,
    TingJiZanTai,
    BianSu,
    BianYa,
    JinJiTingJiZanTai,
    JinJiGuZhang,
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum JiZuBianZaiLeiXing{
    HeZha,
    FenZha,
    JieLie,
    ZhongZai,
    PuTong,
    Wu,
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum JiJi{
    CY(ChaiYouJiJi),
    QL(QiLunJiJi),
    AD(AnDianJiJi),
}
impl AttrSetter for JiJi {
    fn ting_ji_setter(&mut self, _t:u32) {
    }
    fn wen_setter(&mut self, _t:u32) {
    }
    fn bei_che_wan_bi_setter(&mut self, _t:u32){
        match self{
            &mut JiJi::CY(ref mut j) => {
                j.ting_ji_setter(_t);
                j.ran_you_ya_li = RAN_YOU_YA_LI_CHAI_YOU_ZHENG_CHANG;
                j.qi_dong_kong_qi_ya_li = QI_DONG_KONG_QI_YA_LI_CHAI_YOU_ZHENG_CHANG;
            }
            &mut JiJi::QL(ref mut j) => j.ting_ji_setter(_t),
            _ => {}
        }
    }
    fn bei_che_zan_tai_setter(&mut self, _t:u32){

    }
    fn bian_su_setter(&mut self, _t:u32){

    }
    fn bian_ya_setter(&mut self, _t:u32){

    }
    fn qi_dong_setter(&mut self, _t:u32){

    }
    fn ting_ji_zan_tai_setter(&mut self, _t:u32){

    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct JiZu<J> {
    pub id : usize,
    pub common_ji : JiZuCommonJi,
    pub ji_can_shu_ji: J,
}
impl<J> JiZu<J> {
    pub fn new(_id:usize, ji:J) -> JiZu<J> {
        JiZu {
            id : _id,
            common_ji : JiZuCommonJi::new(),
            ji_can_shu_ji : ji,
        }
    }
}
fn ji_zu_ting_ji_setter<J:AttrSetter>(ji_zu:&mut JiZu<J>, t:u32){
    ji_zu.common_ji.ting_ji_setter(t);
    ji_zu.ji_can_shu_ji.ting_ji_setter(t);
}
fn ji_zu_bei_che_wan_bi_setter<J:AttrSetter>(ji_zu:&mut JiZu<J>, t:u32){
    if t < JI_ZU_BEI_CHE_WAN_BI_FREE_T{
        ji_zu.common_ji.bei_che_wan_bi_setter(t);
        ji_zu.ji_can_shu_ji.bei_che_wan_bi_setter(t);
    }
    else {
        ji_zu.common_ji.current_range = JiZuRangeLeiXing::TingJi;
        ji_zu.common_ji.yun_xing_zhuang_tai = JiZuYunXingZhuangTai::TingJi;
        ji_zu.common_ji.t_current_range = 0u32;
    }
}
fn ji_zu_wen_setter<J:AttrSetter>(ji_zu:&mut JiZu<J>, t:u32){
    ji_zu.common_ji.wen_setter(t);
    ji_zu.ji_can_shu_ji.wen_setter(t);
}
fn ji_zu_bei_che_zan_tai_setter<J:AttrSetter>(ji_zu:&mut JiZu<J>, t:u32){
    if t < JI_ZU_CHAI_YOU_BEI_CHE_T {
        ji_zu.common_ji.bei_che_zan_tai_setter(t);
        ji_zu.ji_can_shu_ji.bei_che_zan_tai_setter(t);
    }
    else{
        ji_zu.common_ji.current_range = JiZuRangeLeiXing::BeiCheWanBi;
        ji_zu.common_ji.yun_xing_zhuang_tai = JiZuYunXingZhuangTai::JiuXu;
        ji_zu.common_ji.t_current_range = 0u32;
    }
}
fn ji_zu_bian_su_setter<J:AttrSetter>(ji_zu:&mut JiZu<J>, t:u32){
    if t < ji_zu.common_ji.bian_su_t {
        ji_zu.common_ji.bian_su_setter(t);
        ji_zu.ji_can_shu_ji.bian_su_setter(t);
    }
    else {
        ji_zu.common_ji.current_range = JiZuRangeLeiXing::Wen;
        ji_zu.common_ji.yun_xing_zhuang_tai = JiZuYunXingZhuangTai::YunXing;
        ji_zu.common_ji.t_current_range = 0u32;
    }
}
fn ji_zu_bian_ya_setter<J:AttrSetter>(ji_zu:&mut JiZu<J>, t:u32){
    if t < ji_zu.common_ji.bian_ya_t {
        ji_zu.common_ji.bian_ya_setter(t);
        ji_zu.ji_can_shu_ji.bian_ya_setter(t);
    }
    else {
        ji_zu.common_ji.current_range = JiZuRangeLeiXing::Wen;
        ji_zu.common_ji.yun_xing_zhuang_tai = JiZuYunXingZhuangTai::YunXing;
        ji_zu.common_ji.t_current_range = 0u32;
    }
}
fn ji_zu_qi_dong_setter<J:AttrSetter>(ji_zu:&mut JiZu<J>, t:u32){
    if (t as f64) < JI_ZU_QI_DONG_TB {
        ji_zu.common_ji.qi_dong_setter(t);
        ji_zu.ji_can_shu_ji.qi_dong_setter(t);
    }
    else {
        ji_zu.common_ji.current_range = JiZuRangeLeiXing::Wen;
        ji_zu.common_ji.yun_xing_zhuang_tai = JiZuYunXingZhuangTai::YunXing;
        ji_zu.common_ji.t_current_range = 0u32;
    }
}
fn ji_zu_ting_ji_zan_tai_setter<J:AttrSetter>(ji_zu:&mut JiZu<J>, t:u32){
    if t < JI_ZU_TING_JI_T {
        ji_zu.common_ji.ting_ji_zan_tai_setter(t);
        ji_zu.ji_can_shu_ji.ting_ji_zan_tai_setter(t);
    }
    else {
        ji_zu.common_ji.current_range = JiZuRangeLeiXing::BeiCheWanBi;
        ji_zu.common_ji.yun_xing_zhuang_tai = JiZuYunXingZhuangTai::JiuXu;
        ji_zu.common_ji.t_current_range = 0u32;
    }
}

impl<J:AttrSetter> JiZuCtrl for JiZu<J> {
    fn update<'a>(&'a mut self, t:u32) -> &'a mut Self{
        match self.common_ji.current_range {
            JiZuRangeLeiXing::TingJi | JiZuRangeLeiXing::JinJiGuZhang => JiZuCtrl::update_ting_ji_range(self, t),
            JiZuRangeLeiXing::BeiCheZanTai => self.update_bei_che_zan_tai_range(t),
            JiZuRangeLeiXing::BeiCheWanBi =>
                self.update_bei_che_wan_bi_range(t),
            JiZuRangeLeiXing::QiDong =>
                self.update_qi_dong_range(t),
            JiZuRangeLeiXing::Wen =>
                self.update_wen_range(t),
            JiZuRangeLeiXing::TingJiZanTai |
                JiZuRangeLeiXing::JinJiTingJiZanTai =>
                self.update_ting_ji_zan_tai_range(t),
            JiZuRangeLeiXing::BianSu =>
                self.update_bian_su_range(t),
            JiZuRangeLeiXing::BianYa =>
                self.update_bian_ya_range(t),
        }
    }
    fn update_bei_che_wan_bi_range<'a>(&'a mut self, t:u32) -> &'a mut Self{
        ji_zu_bei_che_wan_bi_setter(self, t);
        self.update_gu_zhang(t);
        self
    }
    fn update_bei_che_zan_tai_range<'a>(&'a mut self, t:u32) -> &'a mut Self{
        ji_zu_bei_che_zan_tai_setter(self, t);
        self.update_gu_zhang(t);
        self
    }
    fn update_bian_su_range<'a>(&'a mut self, t:u32) -> &'a mut Self{
        ji_zu_bian_su_setter(self, t);
        self.update_gu_zhang(t);
        self
    }
    fn update_bian_ya_range<'a>(&'a mut self, t:u32) -> &'a mut Self{
        ji_zu_bian_ya_setter(self, t);
        self.update_gu_zhang(t);
        self
    }
    fn update_qi_dong_range<'a>(&'a mut self, t:u32) -> &'a mut Self{
        ji_zu_qi_dong_setter(self, t);
        self.update_gu_zhang(t);
        self
    }
    fn update_ting_ji_range<'a>(&'a mut self, t:u32) -> &'a mut Self{
        ji_zu_ting_ji_setter(self, t);
        self.update_gu_zhang(t);
        self
    }
    fn update_ting_ji_zan_tai_range<'a>(&'a mut self, t:u32) -> &'a mut Self{
        ji_zu_ting_ji_zan_tai_setter(self, t);
        self.update_gu_zhang(t);
        self
    }
    fn update_wen_range<'a>(&'a mut self, t:u32) -> &'a mut Self{
        ji_zu_wen_setter(self, t);
        self.update_gu_zhang(t);
        self
    }
    fn transit_range<'a>(&'a mut self, _t:u32) -> &'a mut Self{
        self
    }
    fn update_gu_zhang<'a>(&'a mut self, _t:u32) -> &'a mut Self{
        self
    }
    fn gu_zhang_yi_ban_chan_sheng<'a>(&'a mut self, _t:u32) -> &'a mut Self{
        self
    }
    fn gu_zhang_yi_ban_xiao_chu<'a>(&'a mut self, _t:u32) -> &'a mut Self{
        self
    }
    fn gu_zhang_yi_ji_chan_sheng<'a>(&'a mut self, _t:u32) -> &'a mut Self{
        self
    }
    fn gu_zhang_yi_ji_xiao_chu<'a>(&'a mut self, _t:u32) -> &'a mut Self{
        self
    }
    fn gu_zhang_er_ji_chan_sheng<'a>(&'a mut self, _t:u32) -> &'a mut Self{
        self
    }
    fn gu_zhang_er_ji_xiao_chu<'a>(&'a mut self, _t:u32) -> &'a mut Self{
        self
    }
    fn gu_zhang_qi_ta_chan_sheng<'a>(&'a mut self, _t:u32) -> &'a mut Self{
        self
    }
    fn gu_zhang_qi_ta_xiao_chu<'a>(&'a mut self, _t:u32) -> &'a mut Self{
        self
    }
}

use ::libevent_sys::evutil_socket_t;

pub extern fn timeout_cb_ji_zu_chai_you(_fd : evutil_socket_t, _what : ::libc::c_short, arg : * mut ::libc::c_void)
{
	unsafe{
		let mut ji_zu = arg as * mut JiZu<ChaiYouJiJi>;
        (*ji_zu).update((*ji_zu).common_ji.t_current_range);
        (*ji_zu).common_ji.t_current_range += ::simctrl::FANG_ZHEN_BU_CHANG;
	}
}


#[cfg(test)]
mod tests {
    use ::libc;
    use ::util;
    #[cfg(windows)]
    use ::winapi::winsock2::timeval;
    #[cfg(unix)]
    use ::libc::timeval;
    use ::libevent_sys;
    use ::libevent_sys::evutil_socket_t;
    use ::simctrl::FANG_ZHEN_BU_CHANG;
    use super::ChaiYouJiJi;
    use super::JiZu;
    use ::util::zan_tai_linear;
    use super::JiZuRangeLeiXing;
    use super::GEN_V_START_REMAINDER;
    use super::JI_ZU_E_DING_PIN_LV;
    use super::JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI;
    use super::JI_ZU_QI_DONG_TA;
    use super::JI_ZU_E_DING_DIAN_YA;
    use super::GEN_V_E_DING;
    use super::JI_ZU_TING_JI_T;
    use jizu::JiZuCtrl;

    extern fn timeout_cb_ji_zu_chai_you(_fd : evutil_socket_t, _what : ::libc::c_short, arg : * mut ::libc::c_void)
    {
    	unsafe{
    		let mut ji_zu = arg as * mut JiZu<ChaiYouJiJi>;
            (*ji_zu).common_ji.u_delta = 10.0;
            (*ji_zu).common_ji.f_delta = 0.2;
            (*ji_zu).common_ji.bian_su_t = 2000;
            (*ji_zu).common_ji.bian_ya_t = 3000;

            (*ji_zu).update( (*ji_zu).common_ji.t_current_range);
            // cargo test -- --nocapture
            match (*ji_zu).common_ji.current_range {
                JiZuRangeLeiXing::TingJi => {
                    assert_eq!((*ji_zu).common_ji.uab_ext, 0.0f64);
                    println!("停机阶段: {:#?}", (*ji_zu));
                },
                JiZuRangeLeiXing::BeiCheZanTai => {
                    assert_eq!((*ji_zu).common_ji.uab_ext, 0.0f64);
                    assert_eq!((*ji_zu).common_ji.bei_che_wan_bi, false);
                    println!("暂态备车阶段: {:#?}", (*ji_zu));
                },
                JiZuRangeLeiXing::BeiCheWanBi => {
                    println!("备车完毕阶段: {:#?}", (*ji_zu));
                    assert_eq!((*ji_zu).common_ji.uab_ext, 0.0f64);
                    assert_eq!((*ji_zu).common_ji.current_range, JiZuRangeLeiXing::BeiCheWanBi);
                },
                JiZuRangeLeiXing::QiDong => {
                    assert_eq!( (*ji_zu).common_ji.uab_ext, GEN_V_START_REMAINDER);
                    assert_eq!( (*ji_zu).common_ji.f_ext, zan_tai_linear( (*ji_zu).common_ji.t_current_range as f64, 0.0, JI_ZU_E_DING_PIN_LV-JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI, 0.0, JI_ZU_QI_DONG_TA) );
                    println!("启动阶段: {:#?}", (*ji_zu));
                },
                JiZuRangeLeiXing::Wen => {
                    assert_eq!((*ji_zu).common_ji.uab_ext, JI_ZU_E_DING_DIAN_YA);
                    println!("稳态阶段: {:#?}", (*ji_zu));
                },
                JiZuRangeLeiXing::BianSu => {
                    assert_eq!((*ji_zu).common_ji.uab_ext, (*ji_zu).common_ji.u_delta * FANG_ZHEN_BU_CHANG as f64/(*ji_zu).common_ji.bian_su_t as f64);
                    assert_eq!((*ji_zu).common_ji.f_ext, (*ji_zu).common_ji.f_delta * FANG_ZHEN_BU_CHANG as f64/(*ji_zu).common_ji.bian_su_t as f64);
                    println!("变速阶段: {:#?}", (*ji_zu));
                },
                JiZuRangeLeiXing::BianYa => {
                    assert_eq!((*ji_zu).common_ji.uab_ext, (*ji_zu).common_ji.u_delta * FANG_ZHEN_BU_CHANG as f64/(*ji_zu).common_ji.bian_ya_t as f64);
                    println!("变压阶段: {:#?}", (*ji_zu));
                },
                JiZuRangeLeiXing::TingJiZanTai | JiZuRangeLeiXing::JinJiTingJiZanTai => {
                    assert_eq!((*ji_zu).common_ji.uab_ext, zan_tai_linear( (*ji_zu).common_ji.t_current_range as f64, GEN_V_E_DING, 0.0, 0.0, JI_ZU_TING_JI_T as f64));
                    assert_eq!((*ji_zu).common_ji.f_ext, zan_tai_linear( (*ji_zu).common_ji.t_current_range as f64, JI_ZU_E_DING_PIN_LV, 0.0, 0.0, JI_ZU_TING_JI_T as f64));
                    println!("{:?}: {:#?}", (*ji_zu).common_ji.current_range, (*ji_zu));
                },
                _ => assert!(false),
            }
    	}
    }

    #[test]
    fn test_ji_zu_chai_you() {
        util::ws_init();
        //let flags = libevent_sys::EV_PERSIST;
        let flags = 0;
        let mut tv = [timeval{
            tv_sec : 0,
            tv_usec : 0,
        }; 9];
        let mut ji_zu = [JiZu::new(0, ChaiYouJiJi::new()); 9];
    	unsafe{
    		let base = libevent_sys::event_base_new();
            for i in 0..9 {
                tv[i].tv_usec = 100 + i as i64 * 10;
                ji_zu[i].id = i as u32;
                ji_zu[i].common_ji.t_current_range = tv[i].tv_usec as u32;
                match i {
                    0 => {
                        ji_zu[i].common_ji.current_range = JiZuRangeLeiXing::TingJi;
                    },
                    1 => {
                        ji_zu[i].common_ji.current_range = JiZuRangeLeiXing::BeiCheZanTai;
                    },
                    2 => {
                        ji_zu[i].common_ji.current_range = JiZuRangeLeiXing::BeiCheWanBi;
                    },
                    3 => {
                        ji_zu[i].common_ji.current_range = JiZuRangeLeiXing::QiDong;
                    },
                    4 => {
                        ji_zu[i].common_ji.current_range = JiZuRangeLeiXing::Wen;
                    },
                    5 => {
                        ji_zu[i].common_ji.current_range = JiZuRangeLeiXing::BianSu;
                    },
                    6 => {
                        ji_zu[i].common_ji.current_range = JiZuRangeLeiXing::BianYa;
                    },
                    7 => {
                        ji_zu[i].common_ji.current_range = JiZuRangeLeiXing::TingJiZanTai;
                    },
                    8 => {
                        ji_zu[i].common_ji.current_range = JiZuRangeLeiXing::JinJiTingJiZanTai;
                    },
                    _ => assert!(false),
                }
                let timeout = libevent_sys::event_new(base, -1, flags, timeout_cb_ji_zu_chai_you, &mut (ji_zu[i]) as *mut JiZu<ChaiYouJiJi> as *mut libc::c_void);
        		libevent_sys::event_add(timeout, &(tv[i]));
            }
    		libevent_sys::event_base_dispatch(base);
    		libevent_sys::event_base_free(base);
    	}
    }
}
