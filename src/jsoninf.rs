use std::string::String;
use dianzhan;
use duanluqi;
use fuzai;
use jizu;
use node;
use simctrl;
use xitong;
use zhilu;

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct DianZhanInf {
    pub uid : usize,
    pub ctrl_mode : String,
    pub operating_station : String,
    pub ctrl_mode_she_zhi : String,
    pub operating_station_she_zhi : String,
    pub prio : bool,
    pub u : f64,
    pub f : f64,
    pub p : f64,
    pub p_yu_du : f64,
}
impl DianZhanInf {
    pub fn from_ob (ob : &dianzhan::DianZhan) -> DianZhanInf {
        DianZhanInf {
            uid : ob.uid,
            ctrl_mode : match ob.ctrl_mode {
                simctrl::CtrlMode::Manual => "手动".to_string(),
                simctrl::CtrlMode::SemiAuto => "半自动".to_string(),
                simctrl::CtrlMode::Auto => "自动".to_string(),
            },
            operating_station : match ob.operating_station {
                simctrl::OperatingStation::JiPang => "机旁".to_string(),
                simctrl::OperatingStation::Remote => "遥控".to_string(),
                simctrl::OperatingStation::Local => "本地".to_string(),
                simctrl::OperatingStation::JiKong => "集控".to_string(),
            },
            ctrl_mode_she_zhi : match ob.ctrl_mode_she_zhi {
                simctrl::CtrlMode::Manual => "手动".to_string(),
                simctrl::CtrlMode::SemiAuto => "半自动".to_string(),
                simctrl::CtrlMode::Auto => "自动".to_string(),
            },

            operating_station_she_zhi : match ob.operating_station_she_zhi {
                simctrl::OperatingStation::JiPang => "机旁".to_string(),
                simctrl::OperatingStation::Remote => "遥控".to_string(),
                simctrl::OperatingStation::Local => "本地".to_string(),
                simctrl::OperatingStation::JiKong => "集控".to_string(),
            },
            prio : ob.prio,
            u : ob.u,
            f : ob.f,
            p : ob.p,
            p_yu_du : ob.p_yu_du,
        }
    }

}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DuanLuQiInf {
  pub uid : usize,
  pub status : bool,
  pub fault : bool,
  pub ready_to_bing_che : bool,
  pub ready_to_jie_lie : bool,
  pub uab : f64,
  pub ubc : f64,
  pub uca : f64,
  pub ia : f64,
  pub ib : f64,
  pub ic : f64,
  pub f : f64,
  pub gong_lv_yu_bao_jing : bool,
  pub zong_he_gu_zhang : bool,
  pub xiang_jian_bu_ping_heng : bool,
  pub tong_bu_shi_bai : bool,
  pub fen_duan_shi_bai : bool,
  pub guo_dian_liu : bool,
  pub he_zha_shi_bai : bool,
}

impl DuanLuQiInf {
    pub fn from_ob(dlq : &duanluqi::DuanLuQi) -> DuanLuQiInf {
        DuanLuQiInf {
            uid : dlq.uid,
            status : match dlq.status{
                duanluqi::DuanLuQiStatus::On {..} => true,
                duanluqi::DuanLuQiStatus::Off {..} => false,
            },
            fault : match dlq.status{
                duanluqi::DuanLuQiStatus::On{fault, ..} => {
                    if fault {
                        true
                    }
                    else {
                        false
                    }
                }
                duanluqi::DuanLuQiStatus::Off{fault, ..} => {
                    if fault {
                        true
                    }
                    else {
                        false
                    }
                }
            },

            ready_to_bing_che : match dlq.status{
                duanluqi::DuanLuQiStatus::On{..} => false,
                duanluqi::DuanLuQiStatus::Off{ready_to_bing_che, ..} => {
                    if ready_to_bing_che {
                        true
                    }
                    else {
                        false
                    }
                }
            },

            ready_to_jie_lie : match dlq.status{
                duanluqi::DuanLuQiStatus::Off{..} => false,
                duanluqi::DuanLuQiStatus::On{ready_to_jie_lie, ..} => {
                    if ready_to_jie_lie {
                        true
                    }
                    else {
                        false
                    }
                }
            },

            uab : dlq.uab,
            ubc : dlq.ubc,
            uca : dlq.uca,
            ia : dlq.ia,
            ib : dlq.ib,
            ic : dlq.ic,
            f : dlq.f,
            gong_lv_yu_bao_jing : dlq.gong_lv_yu_bao_jing,
            zong_he_gu_zhang : dlq.zong_he_gu_zhang,
            xiang_jian_bu_ping_heng : dlq.xiang_jian_bu_ping_heng,
            tong_bu_shi_bai : dlq.tong_bu_shi_bai,
            fen_duan_shi_bai : dlq.fen_duan_shi_bai,
            guo_dian_liu : dlq.guo_dian_liu,
            he_zha_shi_bai : dlq.he_zha_shi_bai,
        }
    }
}


#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct FuZaiInf {
  pub uid : usize,
  pub u : f64,
  pub p : f64,
  pub q : f64,
  pub i : f64,
  pub p_max : f64,
  pub q_max : f64,
  pub is_online : bool,
}

impl FuZaiInf {
    pub fn from_ob(ob : &fuzai::FuZai) -> FuZaiInf {
        FuZaiInf{
            uid : ob.uid,
            u : ob.u,
            p : ob.p,
            q : ob.q,
            i : ob.i,
            p_max : ob.p_max,
            q_max : ob.q_max,
            is_online: ob.is_online,
        }
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct JiZuInf {
    pub uid : usize,
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

    pub ctrl_mode : String,
    pub jia_su : bool,
    pub jian_su : bool,
    pub yun_xing_zhuang_tai : String,
    pub bei_che_wan_bi : bool,
    pub tong_bu : bool,
    pub bing_che : bool,
    pub qi_dong : bool,
    pub ting_ji : bool,
    pub operating_station : String,
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
    pub ctrl_mode_she_zhi : String,
    pub operating_station_she_zhi : String,
    pub is_online : bool, // 指示机组是否在网

    pub gu_zhang_lei_xing : String,
    pub ran_you_xie_lou_err : bool,
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

    pub current_range : String,
    pub bian_zai_lei_xing : String,
    pub f_ref: f64,
    pub u_ref: f64,
    pub bian_su_t : f64,
    pub bian_ya_t : f64,
    pub bian_zai_t : f64,
    pub t_current_range : f64,
    pub bei_che_t : f64,
    pub qi_dong_t : f64,
    pub i_delta : f64,
    pub u_delta : f64,
    pub f_delta : f64,
    pub p_delta : f64,
    pub zhuan_su_delta : f64,
    pub is_bian_zai : bool,

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

    pub run_hua_zong_guan_hua_you_ya_li : f64,
    pub run_hua_zong_guan_hua_you_wen_du : f64,
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
impl JiZuInf {
    pub fn from_ob(ob : &jizu::JiZu<jizu::JiJi>) -> JiZuInf {
        let mut j = JiZuInf {
            uid : ob.uid,
            ia_ext : ob.common_ji.ia_ext,
            ib_ext : ob.common_ji.ib_ext,
            ic_ext : ob.common_ji.ic_ext,
            f_ext : ob.common_ji.f_ext,
            ia_in : ob.common_ji.ia_in,
            ib_in : ob.common_ji.ib_in,
            ic_in : ob.common_ji.ic_in,
            f_in : ob.common_ji.f_in,
            p : ob.common_ji.p,
            q : ob.common_ji.q,
            p_factor : ob.common_ji.p_factor,
            q_factor : ob.common_ji.q_factor,
            uab_ext : ob.common_ji.uab_ext,
            ubc_ext : ob.common_ji.ubc_ext,
            uca_ext : ob.common_ji.uca_ext,
            uab_in : ob.common_ji.uab_in,
            ubc_in : ob.common_ji.ubc_in,
            uca_in : ob.common_ji.uca_in,
            zhuan_su : ob.common_ji.zhuan_su,

            ctrl_mode : match ob.common_ji.ctrl_mode {
                simctrl::CtrlMode::Manual => "手动".to_string(),
                simctrl::CtrlMode::SemiAuto => "半自动".to_string(),
                simctrl::CtrlMode::Auto => "自动".to_string(),
            },
            jia_su : ob.common_ji.jia_su,
            jian_su : ob.common_ji.jian_su,
            yun_xing_zhuang_tai : match ob.common_ji.yun_xing_zhuang_tai {
                jizu::JiZuYunXingZhuangTai::YunXing => "运行".to_string(),
                jizu::JiZuYunXingZhuangTai::TingJi => "停机".to_string(),
                jizu::JiZuYunXingZhuangTai::JiuXu => "就绪".to_string(),
                jizu::JiZuYunXingZhuangTai::GuZhang => "故障".to_string(),
            },
            bei_che_wan_bi : ob.common_ji.bei_che_wan_bi,
            tong_bu : ob.common_ji.tong_bu,
            bing_che : ob.common_ji.bing_che,
            qi_dong : ob.common_ji.qi_dong,
            ting_ji : ob.common_ji.ting_ji,
            operating_station : match ob.common_ji.operating_station {
                simctrl::OperatingStation::JiPang => "机旁".to_string(),
                simctrl::OperatingStation::Remote => "遥控".to_string(),
                simctrl::OperatingStation::Local => "本地".to_string(),
                simctrl::OperatingStation::JiKong => "集控".to_string(),
            },
            zhuang_tai_chuan_kou_1 : ob.common_ji.zhuang_tai_chuan_kou_1,
            zhuang_tai_chuan_kou_2 : ob.common_ji.zhuang_tai_chuan_kou_2,
            zhuang_tai_can_1 : ob.common_ji.zhuang_tai_can_1,
            zhuang_tai_can_2 : ob.common_ji.zhuang_tai_can_2,
            zhuang_tai_zhi_neng_io : ob.common_ji.zhuang_tai_zhi_neng_io,
            sheng_ya : ob.common_ji.sheng_ya,
            jiang_ya : ob.common_ji.jiang_ya,
            he_zha : ob.common_ji.he_zha,
            fen_duan : ob.common_ji.fen_duan,
            prio : ob.common_ji.prio,
            ctrl_mode_she_zhi : match ob.common_ji.ctrl_mode_she_zhi {
                simctrl::CtrlMode::Manual => "手动".to_string(),
                simctrl::CtrlMode::SemiAuto => "半自动".to_string(),
                simctrl::CtrlMode::Auto => "自动".to_string(),
            },
            operating_station_she_zhi : match ob.common_ji.operating_station_she_zhi {
                simctrl::OperatingStation::JiPang => "机旁".to_string(),
                simctrl::OperatingStation::Remote => "遥控".to_string(),
                simctrl::OperatingStation::Local => "本地".to_string(),
                simctrl::OperatingStation::JiKong => "集控".to_string(),
            },

            is_online : ob.common_ji.is_online, // 指示机组是否在网

            gu_zhang_lei_xing : match ob.common_ji.gu_zhang_lei_xing {
                jizu::GuZhangLeiXing::YiBan => "一般".to_string(),
                jizu::GuZhangLeiXing::YiJi => "一级".to_string(),
                jizu::GuZhangLeiXing::ErJi => "二级".to_string(),
                jizu::GuZhangLeiXing::QiTa => "其他".to_string(),
                jizu::GuZhangLeiXing::Wu => "无".to_string(),
            },
            ran_you_xie_lou_err : ob.common_ji.ran_you_xie_lou,
            qu_zhou_xiang_ya_li_gao : ob.common_ji.qu_zhou_xiang_ya_li_gao,
            qu_zhou_xiang_kong_qi_ya_li_di : ob.common_ji.qu_zhou_xiang_kong_qi_ya_li_di,
            fa_dong_ji_leng_que_qi_hai_shui_xie_lou : ob.common_ji.fa_dong_ji_leng_que_qi_hai_shui_xie_lou,
            pai_qi_wen_du_gao : ob.common_ji.pai_qi_wen_du_gao,
            zeng_ya_kong_qi_wen_du_gao : ob.common_ji.zeng_ya_kong_qi_wen_du_gao,
            qian_zhou_cheng_hua_you_liu_liang_di : ob.common_ji.qian_zhou_cheng_hua_you_liu_liang_di,
            hou_zhou_cheng_hua_you_liu_liang_di : ob.common_ji.hou_zhou_cheng_hua_you_liu_liang_di,
            hua_you_wen_du_gao : ob.common_ji.hua_you_wen_du_gao,
            hai_shui_ya_li_di : ob.common_ji.hai_shui_ya_li_di,
            peng_zhang_shui_xiang_ye_wei_di : ob.common_ji.peng_zhang_shui_xiang_ye_wei_di,
            qian_zhou_cheng_wen_du_gao : ob.common_ji.qian_zhou_cheng_wen_du_gao,
            hou_zhou_cheng_wen_du_gao : ob.common_ji.hou_zhou_cheng_wen_du_gao,
            pin_lv_yue_xian : ob.common_ji.pin_lv_yue_xian,
            ni_gong_lv : ob.common_ji.ni_gong_lv,
            guo_dian_ya : ob.common_ji.guo_dian_ya,
            qian_dian_ya : ob.common_ji.qian_dian_ya,
            guo_dian_liu : ob.common_ji.guo_dian_liu,
            san_xiang_dian_ya_bu_ping_heng : ob.common_ji.san_xiang_dian_ya_bu_ping_heng,
            wai_bu_duan_lu : ob.common_ji.wai_bu_duan_lu,
            duan_lu_qi_yu_bao_jing : ob.common_ji.duan_lu_qi_yu_bao_jing,
            qi_dong_shi_bai : ob.common_ji.qi_dong_shi_bai,
            ting_ji_shi_bai : ob.common_ji.ting_ji_shi_bai,
            duan_lu_qi_zong_he_gu_zhang : ob.common_ji.duan_lu_qi_zong_he_gu_zhang,
            nei_bu_duan_lu : ob.common_ji.nei_bu_duan_lu,
            hua_you_ya_li_guo_di : ob.common_ji.hua_you_ya_li_guo_di,
            chao_su_ting_ji : ob.common_ji.chao_su_ting_ji,
            qian_zhou_cheng_yu_gong_hua_you_liu_liang_di : ob.common_ji.qian_zhou_cheng_yu_gong_hua_you_liu_liang_di,
            hou_zhou_cheng_yu_gong_hua_you_liu_liang_di : ob.common_ji.hou_zhou_cheng_yu_gong_hua_you_liu_liang_di,
            chai_you_ji_jin_ji_ting_ji : ob.common_ji.chai_you_ji_jin_ji_ting_ji,
            ji_pang_shou_dong_ying_ji_ting_ji : ob.common_ji.ji_pang_shou_dong_ying_ji_ting_ji,
            pai_qi_ya_li_guo_gao : ob.common_ji.pai_qi_ya_li_guo_gao,
            bing_che_shi_bai : ob.common_ji.bing_che_shi_bai,
            rao_zu_wen_du_gao : ob.common_ji.rao_zu_wen_du_gao,
            hua_you_ya_li_di : ob.common_ji.hua_you_ya_li_di,
            leng_que_shui_wen_du_gao : ob.common_ji.leng_que_shui_wen_du_gao,
            rao_zu_wen_du_gao_tiao_zha_ting_ji : ob.common_ji.rao_zu_wen_du_gao_tiao_zha_ting_ji,
            pai_qi_ya_li_gao : ob.common_ji.pai_qi_ya_li_gao,
            jian_su_qi_run_hua_you_ya_li_di : ob.common_ji.jian_su_qi_run_hua_you_ya_li_di,
            qi_lun_ji_hou_zhou_cheng_wen_du_gao : ob.common_ji.qi_lun_ji_hou_zhou_cheng_wen_du_gao,
            fa_dian_ji_hou_zhou_cheng_wen_du_gao : ob.common_ji.fa_dian_ji_hou_zhou_cheng_wen_du_gao,
            you_xiang_wen_du_gao : ob.common_ji.you_xiang_wen_du_gao,
            fa_dian_ji_leng_que_shui_xie_lou : ob.common_ji.fa_dian_ji_leng_que_shui_xie_lou,
            zhao_nei_shi_huo_jin_ji_ting_ji : ob.common_ji.zhao_nei_shi_huo_jin_ji_ting_ji,
            qi_nang_ge_zhen_er_ji_gu_zhang : ob.common_ji.qi_nang_ge_zhen_er_ji_gu_zhang,
            zhao_nei_huo_jing : ob.common_ji.zhao_nei_huo_jing,
            dan_shui_ya_li_di : ob.common_ji.dan_shui_ya_li_di,
            wu_zhuo_cao_ye_wei_gao : ob.common_ji.wu_zhuo_cao_ye_wei_gao,
            zhao_nei_feng_ji_gu_zhang : ob.common_ji.zhao_nei_feng_ji_gu_zhang,
            xiao_fang_she_bei_gu_zhang : ob.common_ji.xiao_fang_she_bei_gu_zhang,
            qi_nang_ge_zhen_yi_ban_gu_zhang : ob.common_ji.qi_nang_ge_zhen_yi_ban_gu_zhang,
            ji_dai_hua_you_bang_gu_zhang : ob.common_ji.ji_dai_hua_you_bang_gu_zhang,
            jin_feng_wen_du_gao : ob.common_ji.jin_feng_wen_du_gao,
            chu_feng_wen_du_gao : ob.common_ji.chu_feng_wen_du_gao,
            wu_you_cao_ye_wei_gao : ob.common_ji.wu_you_cao_ye_wei_gao,
            zong_he_gu_zhang_bao_jing : ob.common_ji.zong_he_gu_zhang_bao_jing,

            current_range : match ob.common_ji.current_range {
                jizu::JiZuRangeLeiXing::TingJi => "停机".to_string(),
                jizu::JiZuRangeLeiXing::BeiCheZanTai => "备车暂态".to_string(),
                jizu::JiZuRangeLeiXing::BeiCheWanBi => "备车完毕".to_string(),
                jizu::JiZuRangeLeiXing::QiDong => "启动".to_string(),
                jizu::JiZuRangeLeiXing::Wen => "稳态".to_string(),
                jizu::JiZuRangeLeiXing::TingJiZanTai => "停机暂态".to_string(),
                jizu::JiZuRangeLeiXing::BianSu => "变速".to_string(),
                jizu::JiZuRangeLeiXing::BianYa => "变压".to_string(),
                jizu::JiZuRangeLeiXing::JinJiTingJiZanTai => "紧急停机暂态".to_string(),
                jizu::JiZuRangeLeiXing::JinJiGuZhang => "紧急故障".to_string(),
            },
            bian_zai_lei_xing : match ob.common_ji.bian_zai_lei_xing {
                jizu::JiZuBianZaiLeiXing::HeZha => "合闸".to_string(),
                jizu::JiZuBianZaiLeiXing::FenZha => "分闸".to_string(),
                jizu::JiZuBianZaiLeiXing::JieLie => "解列".to_string(),
                jizu::JiZuBianZaiLeiXing::ZhongZai => "重载".to_string(),
                jizu::JiZuBianZaiLeiXing::PuTong => "普通".to_string(),
                jizu::JiZuBianZaiLeiXing::Wu => "无".to_string(),
            },
            f_ref: ob.common_ji.f_ref,
            u_ref: ob.common_ji.u_ref,
            bian_su_t : ob.common_ji.bian_su_t,
            bian_ya_t : ob.common_ji.bian_ya_t,
            bian_zai_t : ob.common_ji.bian_zai_t,
            t_current_range : ob.common_ji.t_current_range,
            bei_che_t : ob.common_ji.bei_che_t,
            qi_dong_t : ob.common_ji.qi_dong_t,
            i_delta : ob.common_ji.i_delta,
            u_delta : ob.common_ji.u_delta,
            f_delta : ob.common_ji.f_delta,
            p_delta : ob.common_ji.p_delta,
            zhuan_su_delta : ob.common_ji.zhuan_su_delta,
            is_bian_zai : ob.common_ji.is_bian_zai,

            leng_que_shui_wen_du_a_lie : 0.0,
            leng_que_shui_wen_du_b_lie : 0.0,
            zhou_cheng_wen_du_qian : 0.0,
            zhou_cheng_wen_du_hou : 0.0,
            jin_feng_wen_du : 0.0,
            chu_feng_wen_du : 0.0,
            hua_you_ya_li : 0.0,
            hua_you_wen_du : 0.0,
            zhou_cheng_hua_you_liu_liang_qian : 0.0,
            zhou_cheng_hua_you_liu_liang_hou : 0.0,
            zeng_ya_kong_qi_wen_du_a_lie : 0.0,
            zeng_ya_kong_qi_wen_du_b_lie : 0.0,
            pai_qi_wen_du_a_lie : 0.0,
            pai_qi_wen_du_b_lie : 0.0,
            qu_zhou_xiang_ya_li : 0.0,
            ran_you_ya_li : 0.0,
            hai_shui_ya_li : 0.0,
            ji_you_ya_li : 0.0,
            ji_you_wen_du : 0.0,
            qi_dong_kong_qi_ya_li : 0.0,
            rao_zu_wen_du_a : 0.0,
            rao_zu_wen_du_b : 0.0,
            rao_zu_wen_du_c : 0.0,
            dan_shui_wen_du_a_lie : 0.0,
            dan_shui_wen_du_b_lie : 0.0,
            dan_shui_ya_li : 0.0,
            hua_you_xiang_wen_du : 0.0,
            zhao_nei_kong_qi_wen_du : 0.0,

            run_hua_zong_guan_hua_you_ya_li : 0.0,
            run_hua_zong_guan_hua_you_wen_du : 0.0,
            zhu_qi_men_zheng_qi_ya_li : 0.0,
            zhu_qi_men_zheng_qi_wen_du : 0.0,
            chou_qi_qi_zheng_qi_ya_li : 0.0,
            chou_qi_qi_zheng_qi_wen_du : 0.0,
            qi_feng_xi_tong_ya_li : 0.0,
            qi_feng_chou_qi_xi_tong_zheng_qi_ya_li : 0.0,
            pai_qi_ya_li : 0.0,
            jian_su_qi_run_hua_you_ya_li : 0.0,
            dian_dong_ning_shui_bang_chu_kou_ya_li_1 : 0.0,
            dian_dong_ning_shui_bang_chu_kou_ya_li_2 : 0.0,
            tiao_jie_xi_tong_hua_you_ya_li : 0.0,
            leng_que_hai_shui_jin_kou_wen_du : 0.0,
            qi_lun_ji_a_lie_zhou_cheng_wen_du : 0.0,
            qi_lun_ji_b_lie_zhou_cheng_wen_du : 0.0,
            fa_dian_ji_a_lie_zhou_cheng_wen_du : 0.0,
            fa_dian_ji_b_lie_zhou_cheng_wen_du : 0.0,
            you_xiang_hua_you_wen_du : 0.0,
            jian_su_qi_zhou_cheng_hui_you_wen_du : 0.0,
            zhu_you_qi_chu_kou_hua_you_ya_li : 0.0,
            di_ya_lv_you_qi_qian_hua_you_ya_li : 0.0,
            gao_ya_lv_you_qi_hou_hua_you_ya_li : 0.0,
            dian_dong_you_bang_chu_kou_hua_you_ya_li : 0.0,
            zhu_you_bang_jin_kou_hua_you_ya_li : 0.0,
        };
        match ob.ji_can_shu_ji {
            jizu::JiJi::CY(ji) => {
                j.leng_que_shui_wen_du_a_lie = ji.leng_que_shui_wen_du_a_lie;
                j.leng_que_shui_wen_du_b_lie = ji.leng_que_shui_wen_du_b_lie;
                j.zhou_cheng_wen_du_qian = ji.zhou_cheng_wen_du_qian;
                j.zhou_cheng_wen_du_hou = ji.zhou_cheng_wen_du_hou;
                j.jin_feng_wen_du = ji.jin_feng_wen_du;
                j.chu_feng_wen_du = ji.chu_feng_wen_du;
                j.hua_you_ya_li = ji.hua_you_ya_li;
                j.hua_you_wen_du = ji.hua_you_wen_du;
                j.zhou_cheng_hua_you_liu_liang_qian = ji.zhou_cheng_hua_you_liu_liang_qian;
                j.zhou_cheng_hua_you_liu_liang_hou = ji.zhou_cheng_hua_you_liu_liang_hou;
                j.zeng_ya_kong_qi_wen_du_a_lie = ji.zeng_ya_kong_qi_wen_du_a_lie;
                j.zeng_ya_kong_qi_wen_du_b_lie = ji.zeng_ya_kong_qi_wen_du_b_lie;
                j.pai_qi_wen_du_a_lie = ji.pai_qi_wen_du_a_lie;
                j.pai_qi_wen_du_b_lie = ji.pai_qi_wen_du_b_lie;
                j.qu_zhou_xiang_ya_li = ji.qu_zhou_xiang_ya_li;
                j.ran_you_ya_li = ji.ran_you_ya_li;
                j.hai_shui_ya_li = ji.hai_shui_ya_li;
                j.ji_you_ya_li = ji.ji_you_ya_li;
                j.ji_you_wen_du = ji.ji_you_wen_du;
                j.qi_dong_kong_qi_ya_li = ji.qi_dong_kong_qi_ya_li;
                j.rao_zu_wen_du_a = ji.rao_zu_wen_du_a;
                j.rao_zu_wen_du_b = ji.rao_zu_wen_du_b;
                j.rao_zu_wen_du_c = ji.rao_zu_wen_du_c;
                j.dan_shui_wen_du_a_lie = ji.dan_shui_wen_du_a_lie;
                j.dan_shui_wen_du_b_lie = ji.dan_shui_wen_du_b_lie;
                j.dan_shui_ya_li = ji.dan_shui_ya_li;
                j.hua_you_xiang_wen_du = ji.hua_you_xiang_wen_du;
                j.zhao_nei_kong_qi_wen_du = ji.zhao_nei_kong_qi_wen_du;
            }
            jizu::JiJi::QL(ji) => {
                j.jin_feng_wen_du = ji.jin_feng_wen_du;
                j.chu_feng_wen_du = ji.chu_feng_wen_du;
                j.pai_qi_wen_du_b_lie = ji.pai_qi_wen_du_b_lie;
                j.rao_zu_wen_du_a = ji.rao_zu_wen_du_a;
                j.rao_zu_wen_du_b = ji.rao_zu_wen_du_b;
                j.rao_zu_wen_du_c = ji.rao_zu_wen_du_c;
                j.run_hua_zong_guan_hua_you_ya_li = ji.run_hua_zong_guan_hua_you_ya_li;
                j.run_hua_zong_guan_hua_you_wen_du = ji.run_hua_zong_guan_hua_you_wen_du;
                j.zhu_qi_men_zheng_qi_ya_li = ji.zhu_qi_men_zheng_qi_ya_li;
                j.zhu_qi_men_zheng_qi_wen_du = ji.zhu_qi_men_zheng_qi_wen_du;
                j.chou_qi_qi_zheng_qi_ya_li = ji.chou_qi_qi_zheng_qi_ya_li;
                j.chou_qi_qi_zheng_qi_wen_du = ji.chou_qi_qi_zheng_qi_wen_du;
                j.qi_feng_xi_tong_ya_li = ji.qi_feng_xi_tong_ya_li;
                j.qi_feng_chou_qi_xi_tong_zheng_qi_ya_li = ji.qi_feng_chou_qi_xi_tong_zheng_qi_ya_li;
                j.pai_qi_ya_li = ji.pai_qi_ya_li;
                j.jian_su_qi_run_hua_you_ya_li = ji.jian_su_qi_run_hua_you_ya_li;
                j.dian_dong_ning_shui_bang_chu_kou_ya_li_1 = ji.dian_dong_ning_shui_bang_chu_kou_ya_li_1;
                j.dian_dong_ning_shui_bang_chu_kou_ya_li_2 = ji.dian_dong_ning_shui_bang_chu_kou_ya_li_2;
                j.tiao_jie_xi_tong_hua_you_ya_li = ji.tiao_jie_xi_tong_hua_you_ya_li;
                j.leng_que_hai_shui_jin_kou_wen_du = ji.leng_que_hai_shui_jin_kou_wen_du;
                j.qi_lun_ji_a_lie_zhou_cheng_wen_du = ji.qi_lun_ji_a_lie_zhou_cheng_wen_du;
                j.qi_lun_ji_b_lie_zhou_cheng_wen_du = ji.qi_lun_ji_b_lie_zhou_cheng_wen_du;
                j.fa_dian_ji_a_lie_zhou_cheng_wen_du = ji.fa_dian_ji_a_lie_zhou_cheng_wen_du;
                j.fa_dian_ji_b_lie_zhou_cheng_wen_du = ji.fa_dian_ji_b_lie_zhou_cheng_wen_du;
                j.you_xiang_hua_you_wen_du = ji.you_xiang_hua_you_wen_du;
                j.jian_su_qi_zhou_cheng_hui_you_wen_du = ji.jian_su_qi_zhou_cheng_hui_you_wen_du;
                j.zhu_you_qi_chu_kou_hua_you_ya_li = ji.zhu_you_qi_chu_kou_hua_you_ya_li;
                j.di_ya_lv_you_qi_qian_hua_you_ya_li = ji.di_ya_lv_you_qi_qian_hua_you_ya_li;
                j.gao_ya_lv_you_qi_hou_hua_you_ya_li = ji.gao_ya_lv_you_qi_hou_hua_you_ya_li;
                j.dian_dong_you_bang_chu_kou_hua_you_ya_li = ji.dian_dong_you_bang_chu_kou_hua_you_ya_li;
                j.zhu_you_bang_jin_kou_hua_you_ya_li = ji.zhu_you_bang_jin_kou_hua_you_ya_li;
            }
            _ => {}
        }
        j
    }
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct NodeInf {
  pub uid : usize,
  pub u : f64,
  pub f : f64,
  pub status : bool,
}

impl NodeInf {
    pub fn from_ob(ob : &node::Node) -> NodeInf {
        NodeInf{
            uid : ob.uid,
            u : ob.u,
            f : ob.f,
            status : match ob.status {
                node::NodeStatus::On => true,
                node::NodeStatus::Off => false,
            },
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ZhiLuInf {
  pub uid : usize,
  pub p : f64,
  pub q : f64,
  pub i : f64,
  pub status : bool,
}

impl ZhiLuInf {
    pub fn from_ob(ob : &zhilu::ZhiLu) -> ZhiLuInf {
        ZhiLuInf {
            uid : ob.uid,
            p : ob.p,
            q : ob.q,
            i : ob.i,
            status : match ob.status {
                zhilu::ZhiLuStatus::On => true,
                zhilu::ZhiLuStatus::Off => false,
            },
        }
    }
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct XiTongInf {
  pub sec : i64,
  pub nsec : i32,
  pub uid : usize,
  pub yue_kong : bool,
  pub p_shou_ti_dui : f64,
  pub u_shou_ti_dui : f64,
  pub f_shou_ti_dui : f64,
  pub pd_shou_ti_dui : f64,
  pub p_wei_ti_dui : f64,
  pub u_wei_ti_dui : f64,
  pub f_wei_ti_dui : f64,
  pub pd_wei_ti_dui : f64,
  pub p_quan_jian : f64,

  pub is_xiao_sheng : bool,
  pub is_ying_da : bool,

  pub ji_zu_vec : Vec<JiZuInf >,
  pub dian_zhan_vec : Vec<DianZhanInf>,
  pub duan_lu_qi_vec : Vec<DuanLuQiInf>,
  pub fu_zai_vec : Vec<FuZaiInf>,
  pub node_vec : Vec<NodeInf>,
  pub zhi_lu_vec : Vec<ZhiLuInf>,
}
impl XiTongInf {
    pub fn from_ob(ob : &xitong::XiTong) -> XiTongInf {
        XiTongInf {
            sec : ob.sec,
            nsec : ob.nsec,
            uid : ob.uid,
            yue_kong : ob.yue_kong,
            p_shou_ti_dui : ob.p_shou_ti_dui,
            u_shou_ti_dui : ob.u_shou_ti_dui,
            f_shou_ti_dui : ob.f_shou_ti_dui,
            pd_shou_ti_dui : ob.pd_shou_ti_dui,
            p_wei_ti_dui : ob.p_wei_ti_dui,
            u_wei_ti_dui : ob.u_wei_ti_dui,
            f_wei_ti_dui : ob.f_wei_ti_dui,
            pd_wei_ti_dui : ob.pd_wei_ti_dui,
            p_quan_jian : ob.p_quan_jian,

            is_xiao_sheng : ob.is_xiao_sheng,
            is_ying_da : ob.is_ying_da,

            ji_zu_vec : ob.ji_zu_vec.iter().map(|ji_zu|JiZuInf::from_ob(ji_zu)).collect(),
            dian_zhan_vec : ob.dian_zhan_vec.iter().map(|dian_zhan|DianZhanInf::from_ob(dian_zhan)).collect(),
            duan_lu_qi_vec : ob.duan_lu_qi_vec.iter().map(|duan_lu_qi|DuanLuQiInf::from_ob(duan_lu_qi)).collect(),
            fu_zai_vec : ob.fu_zai_vec.iter().map(|fu_zai|FuZaiInf::from_ob(fu_zai)).collect(),
            node_vec : ob.node_vec.iter().map(|node|NodeInf::from_ob(node)).collect(),
            zhi_lu_vec : ob.zhi_lu_vec.iter().map(|zhi_lu|ZhiLuInf::from_ob(zhi_lu)).collect(),
        }
    }
}
