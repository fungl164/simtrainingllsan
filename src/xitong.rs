use std::vec::Vec;
use std::string::String;
use std::thread;
use std::time::Duration;
use std::sync::Arc;
use std::sync::RwLock;
use iron;
use iron::prelude::*;
use iron::mime::Mime;
use iron::status;
use serde_json;
use time;
// use mio;
// use mio::{ EventLoop };
use jizu::{JiZu, JiJi, JiZuCtrl, JiZuRangeLeiXing};
use dianzhan::DianZhan;
use duanluqi::{DuanLuQi};
use fuzai::FuZai;
use node::Node;
use zhilu::ZhiLu;
use zhilu::ZhiLuStatus;
use zhiling::{ZhiLing, ZhiLingType, YingDaType, YingDaErr};
// use simctrl::{ Update };

use jizu;
use dianzhan;
use duanluqi;
use fuzai;
use node;
use zhilu;
use simctrl;
use zhiling;
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum PowerFlowResult {
    Success,
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum PowerFlowErr {
    HuiLuExist,
}
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct XiTong {
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

  pub ji_zu_vec : Vec<JiZu<JiJi> >,
  pub dian_zhan_vec : Vec<DianZhan>,
  pub duan_lu_qi_vec : Vec<DuanLuQi>,
  pub fu_zai_vec : Vec<FuZai>,
  pub node_vec : Vec<Node>,
  pub zhi_lu_vec : Vec<ZhiLu>,
  /*支路与节点之间的映射，key为支路，value为节点*/
  pub zhilu_node_vec : Vec<(usize, usize)>,
  /*支路与断路器映射，key为支路， value为断路器*/
  pub zhilu_duanluqi_vec : Vec<(usize, usize)>,
  /*构建发电机与发电机断路器之间的映射，key为发电机，value为机组断路器*/
  pub jizu_duanluqi_vec : Vec<(usize, usize)>,
  /*构建发电机与发电机断路器之间的映射，key为发电机，value为机组断路器*/
  pub jizuchaiyou_duanluqi_vec : Vec<(usize, usize)>,
  /*构建发电机与发电机断路器之间的映射，key为发电机，value为机组断路器*/
  pub jizuqilun_duanluqi_vec : Vec<(usize, usize)>,
  /*构建发电机与发电机断路器之间的映射，key为发电机，value为机组断路器*/
  pub jizuandian_duanluqi_vec : Vec<(usize, usize)>,
  /*节点与发电机之间的映射，key为节点，value为发电机*/
  pub node_jizu_vec : Vec<(usize, usize)>,
  /*节点与负载之间的映射，key为节点，value为负载*/
  pub node_fuzai_vec : Vec<(usize, usize)>,
  pub jizu_dianzhan_vec : Vec<(usize, usize)>,
  pub fuzai_dianzhan_vec : Vec<(usize, usize)>,
  pub duanluqi_dianzhan_vec : Vec<(usize, usize)>,
  pub node_dianzhan_vec : Vec<(usize, usize)>,
  /*节点临时链表，用于计算潮流*/
  pub temp_node_vec : Vec<usize>,
  /*所有相关节点放在一个group中，所有group链表放在_NodeGroupList中*/
  pub node_group_vec : Vec<Vec<usize>>,
  //获取所有与机组相关节点的uid集合
  pub jizunode_vec : Vec<usize>,
  pub jizuduanluqi_vec : Vec<usize>,
}

impl XiTong {
    pub fn build_zhilu_node_vec(&mut self) {
        /*构建支路――节点之间的映射，key为支路，查找支路对应的节点*/
        self.zhilu_node_vec.push((0,1));
        self.zhilu_node_vec.push((0,2));

        self.zhilu_node_vec.push((1,0));
        self.zhilu_node_vec.push((1,1));

        self.zhilu_node_vec.push((2,2));
        self.zhilu_node_vec.push((2,3));

        self.zhilu_node_vec.push((3,6));
        self.zhilu_node_vec.push((3,7));

        self.zhilu_node_vec.push((4,5));
        self.zhilu_node_vec.push((4,6));

        self.zhilu_node_vec.push((5,7));
        self.zhilu_node_vec.push((5,8));

        self.zhilu_node_vec.push((6,14));
        self.zhilu_node_vec.push((6,15));

        self.zhilu_node_vec.push((7,13));
        self.zhilu_node_vec.push((7,14));

        self.zhilu_node_vec.push((8,15));
        self.zhilu_node_vec.push((8,16));

        self.zhilu_node_vec.push((9,18));
        self.zhilu_node_vec.push((9,19));

        self.zhilu_node_vec.push((10,19));
        self.zhilu_node_vec.push((10,20));

        self.zhilu_node_vec.push((11,20));
        self.zhilu_node_vec.push((11,21));

        self.zhilu_node_vec.push((12,21));
        self.zhilu_node_vec.push((12,22));

        self.zhilu_node_vec.push((13,4));
        self.zhilu_node_vec.push((13,5));

        self.zhilu_node_vec.push((14,0));
        self.zhilu_node_vec.push((14,4));

        self.zhilu_node_vec.push((15,4));
        self.zhilu_node_vec.push((15,10));

        self.zhilu_node_vec.push((16,12));
        self.zhilu_node_vec.push((16,13));

        self.zhilu_node_vec.push((17,10));
        self.zhilu_node_vec.push((17,12));

        self.zhilu_node_vec.push((18,12));
        self.zhilu_node_vec.push((18,18));

        self.zhilu_node_vec.push((19,8));
        self.zhilu_node_vec.push((19,9));

        self.zhilu_node_vec.push((20,3));
        self.zhilu_node_vec.push((20,9));

        self.zhilu_node_vec.push((21,9));
        self.zhilu_node_vec.push((21,11));

        self.zhilu_node_vec.push((22,16));
        self.zhilu_node_vec.push((22,17));

        self.zhilu_node_vec.push((23,11));
        self.zhilu_node_vec.push((23,17));

        self.zhilu_node_vec.push((24,17));
        self.zhilu_node_vec.push((24,22));

    }
    pub fn build_zhilu_duanluqi_vec(&mut self){
        /*构建支路与支路断路器之间的映射，key为支路uid，value为支路断路器uid*/
        self.zhilu_duanluqi_vec.push((0,2));
        self.zhilu_duanluqi_vec.push((1,3));
        self.zhilu_duanluqi_vec.push((2,4));
        self.zhilu_duanluqi_vec.push((3,7));

        self.zhilu_duanluqi_vec.push((4,8));
        self.zhilu_duanluqi_vec.push((5,9));
        self.zhilu_duanluqi_vec.push((6,14));
        self.zhilu_duanluqi_vec.push((7,15));

        self.zhilu_duanluqi_vec.push((8,16));
        self.zhilu_duanluqi_vec.push((9,22));
        self.zhilu_duanluqi_vec.push((10,23));
        self.zhilu_duanluqi_vec.push((11,24));

        self.zhilu_duanluqi_vec.push((12,25));
        self.zhilu_duanluqi_vec.push((13,26));
        self.zhilu_duanluqi_vec.push((14,27));
        self.zhilu_duanluqi_vec.push((15,28));

        self.zhilu_duanluqi_vec.push((16,29));
        self.zhilu_duanluqi_vec.push((17,30));
        self.zhilu_duanluqi_vec.push((18,31));
        self.zhilu_duanluqi_vec.push((19,32));

        self.zhilu_duanluqi_vec.push((20,33));
        self.zhilu_duanluqi_vec.push((21,34));
        self.zhilu_duanluqi_vec.push((22,35));
        self.zhilu_duanluqi_vec.push((23,36));

        self.zhilu_duanluqi_vec.push((24,37));
    }
    pub fn build_jizu_duanluqi_vec(&mut self){
        self.jizu_duanluqi_vec.push((0,0));
        self.jizu_duanluqi_vec.push((1,1));
        self.jizu_duanluqi_vec.push((2,5));
        self.jizu_duanluqi_vec.push((3,6));
        self.jizu_duanluqi_vec.push((4,12));

        self.jizu_duanluqi_vec.push((5,13));
        self.jizu_duanluqi_vec.push((6,19));
        self.jizu_duanluqi_vec.push((7,20));
        self.jizu_duanluqi_vec.push((8,21));

        self.jizu_duanluqi_vec.push((9,10));
        self.jizu_duanluqi_vec.push((10,11));
        self.jizu_duanluqi_vec.push((11,17));
        self.jizu_duanluqi_vec.push((12,18));

    }
    pub fn build_jizuandian_duanluqi_vec(&mut self){
        self.jizuandian_duanluqi_vec.push((9,10));
        self.jizuandian_duanluqi_vec.push((10,11));
        self.jizuandian_duanluqi_vec.push((11,17));
        self.jizuandian_duanluqi_vec.push((12,18));
    }
    pub fn build_jizuchaiyou_duanluqi_vec(&mut self){
        /*6个柴油机与柴油机断路器*/
        self.jizuchaiyou_duanluqi_vec.push((0,0));
        self.jizuchaiyou_duanluqi_vec.push((1,1));
        self.jizuchaiyou_duanluqi_vec.push((6,19));
        self.jizuchaiyou_duanluqi_vec.push((7,20));

        self.jizuchaiyou_duanluqi_vec.push((8,21));
    }
    pub fn build_jizuqilun_duanluqi_vec(&mut self){
        self.jizuqilun_duanluqi_vec.push((2,5));
        self.jizuqilun_duanluqi_vec.push((3,6));
        self.jizuqilun_duanluqi_vec.push((4,12));
        self.jizuqilun_duanluqi_vec.push((5,13));
    }
    pub fn build_node_jizu_vec(&mut self){
        self.node_jizu_vec.push((1,0));
        self.node_jizu_vec.push((2,1));
        self.node_jizu_vec.push((6,2));
        self.node_jizu_vec.push((7,3));
        self.node_jizu_vec.push((14,4));

        self.node_jizu_vec.push((15,5));
        self.node_jizu_vec.push((19,6));
        self.node_jizu_vec.push((20,7));
        self.node_jizu_vec.push((21,8));
        self.node_jizu_vec.push((5,9));

        self.node_jizu_vec.push((8,10));
        self.node_jizu_vec.push((13,11));
        self.node_jizu_vec.push((16,12));

    }
    pub fn build_node_fuzai_vec(&mut self){
        self.node_fuzai_vec.push((1,0));
        self.node_fuzai_vec.push((1,1));
        self.node_fuzai_vec.push((2,2));
        self.node_fuzai_vec.push((2,3));
        self.node_fuzai_vec.push((6,4));

        self.node_fuzai_vec.push((6,5));
        self.node_fuzai_vec.push((6,6));
        self.node_fuzai_vec.push((7,7));
        self.node_fuzai_vec.push((7,8));
        self.node_fuzai_vec.push((14,9));

        self.node_fuzai_vec.push((14,10));
        self.node_fuzai_vec.push((14,11));
        self.node_fuzai_vec.push((15,12));
        self.node_fuzai_vec.push((15,13));
        self.node_fuzai_vec.push((15,14));

        self.node_fuzai_vec.push((19,15));
        self.node_fuzai_vec.push((19,16));
        self.node_fuzai_vec.push((19,17));
        self.node_fuzai_vec.push((20,18));
        self.node_fuzai_vec.push((21,19));

        self.node_fuzai_vec.push((21,20));
    }
    pub fn build_jizunode_vec(&mut self){
        self.jizunode_vec =  self.node_jizu_vec.iter().cloned().map(|node_jizu|node_jizu.0).collect()
    }
    pub fn build_jizuduanluqi_vec(&mut self){
        self.jizuduanluqi_vec =  self.jizu_duanluqi_vec.iter().cloned().map(|jizu_duanluqi|jizu_duanluqi.1).collect()
    }
    pub fn build_jizu_dianzhan_vec(&mut self) {
        self.jizu_dianzhan_vec.push((0, 0));
        self.jizu_dianzhan_vec.push((1, 0));

        self.jizu_dianzhan_vec.push((2, 1));
        self.jizu_dianzhan_vec.push((3, 1));

        self.jizu_dianzhan_vec.push((4, 2));
        self.jizu_dianzhan_vec.push((5, 2));

        self.jizu_dianzhan_vec.push((6, 3));
        self.jizu_dianzhan_vec.push((7, 3));
        self.jizu_dianzhan_vec.push((8, 3));
    }
    pub fn build_fuzai_dianzhan_vec(&mut self) {
        self.fuzai_dianzhan_vec.push((0, 0));
        self.fuzai_dianzhan_vec.push((1, 0));
        self.fuzai_dianzhan_vec.push((2, 0));
        self.fuzai_dianzhan_vec.push((3, 0));

        self.fuzai_dianzhan_vec.push((4, 1));
        self.fuzai_dianzhan_vec.push((5, 1));
        self.fuzai_dianzhan_vec.push((6, 1));
        self.fuzai_dianzhan_vec.push((7, 1));
        self.fuzai_dianzhan_vec.push((8, 1));

        self.fuzai_dianzhan_vec.push((9, 2));
        self.fuzai_dianzhan_vec.push((10, 2));
        self.fuzai_dianzhan_vec.push((11, 2));
        self.fuzai_dianzhan_vec.push((12, 2));
        self.fuzai_dianzhan_vec.push((13, 2));
        self.fuzai_dianzhan_vec.push((14, 2));

        self.fuzai_dianzhan_vec.push((15, 3));
        self.fuzai_dianzhan_vec.push((16, 3));
        self.fuzai_dianzhan_vec.push((17, 3));
        self.fuzai_dianzhan_vec.push((18, 3));
        self.fuzai_dianzhan_vec.push((19, 3));
        self.fuzai_dianzhan_vec.push((20, 3));
    }
    pub fn build_duanluqi_dianzhan_vec(&mut self) {
        self.duanluqi_dianzhan_vec.push((0, 0));
        self.duanluqi_dianzhan_vec.push((1, 0));
        self.duanluqi_dianzhan_vec.push((2, 0));
        self.duanluqi_dianzhan_vec.push((3, 0));
        self.duanluqi_dianzhan_vec.push((4, 0));

        self.duanluqi_dianzhan_vec.push((5, 1));
        self.duanluqi_dianzhan_vec.push((6, 1));
        self.duanluqi_dianzhan_vec.push((7, 1));
        self.duanluqi_dianzhan_vec.push((8, 1));
        self.duanluqi_dianzhan_vec.push((9, 1));
        self.duanluqi_dianzhan_vec.push((10, 1));
        self.duanluqi_dianzhan_vec.push((11, 1));
        self.duanluqi_dianzhan_vec.push((26, 1));
        self.duanluqi_dianzhan_vec.push((27, 1));
        self.duanluqi_dianzhan_vec.push((28, 1));
        self.duanluqi_dianzhan_vec.push((32, 1));
        self.duanluqi_dianzhan_vec.push((33, 1));
        self.duanluqi_dianzhan_vec.push((34, 1));

        self.duanluqi_dianzhan_vec.push((12, 2));
        self.duanluqi_dianzhan_vec.push((13, 2));
        self.duanluqi_dianzhan_vec.push((14, 2));
        self.duanluqi_dianzhan_vec.push((15, 2));
        self.duanluqi_dianzhan_vec.push((16, 2));
        self.duanluqi_dianzhan_vec.push((17, 2));
        self.duanluqi_dianzhan_vec.push((18, 2));
        self.duanluqi_dianzhan_vec.push((29, 2));
        self.duanluqi_dianzhan_vec.push((30, 2));
        self.duanluqi_dianzhan_vec.push((31, 2));
        self.duanluqi_dianzhan_vec.push((35, 2));
        self.duanluqi_dianzhan_vec.push((36, 2));
        self.duanluqi_dianzhan_vec.push((37, 2));

        self.duanluqi_dianzhan_vec.push((19, 3));
        self.duanluqi_dianzhan_vec.push((20, 3));
        self.duanluqi_dianzhan_vec.push((21, 3));
        self.duanluqi_dianzhan_vec.push((22, 3));
        self.duanluqi_dianzhan_vec.push((23, 3));
        self.duanluqi_dianzhan_vec.push((24, 3));
        self.duanluqi_dianzhan_vec.push((25, 3));
    }
    pub fn build_node_dianzhan_vec(&mut self) {
        self.node_dianzhan_vec.push((0, 0));
        self.node_dianzhan_vec.push((1, 0));
        self.node_dianzhan_vec.push((2, 0));
        self.node_dianzhan_vec.push((3, 0));

        self.node_dianzhan_vec.push((4, 1));
        self.node_dianzhan_vec.push((5, 1));
        self.node_dianzhan_vec.push((6, 1));
        self.node_dianzhan_vec.push((7, 1));
        self.node_dianzhan_vec.push((8, 1));
        self.node_dianzhan_vec.push((9, 1));

        self.node_dianzhan_vec.push((12, 2));
        self.node_dianzhan_vec.push((13, 2));
        self.node_dianzhan_vec.push((14, 2));
        self.node_dianzhan_vec.push((15, 2));
        self.node_dianzhan_vec.push((16, 2));
        self.node_dianzhan_vec.push((17, 2));

        self.node_dianzhan_vec.push((18, 3));
        self.node_dianzhan_vec.push((19, 3));
        self.node_dianzhan_vec.push((20, 3));
        self.node_dianzhan_vec.push((21, 3));
        self.node_dianzhan_vec.push((22, 3));

    }
    pub fn new(_id : usize) -> XiTong {
        let mut x = XiTong{
            sec : time::get_time().sec,
            nsec : time::get_time().nsec,
            uid : _id,
            yue_kong : false,
            p_shou_ti_dui : 0.0f64,
            u_shou_ti_dui : 0.0f64,
            f_shou_ti_dui : 0.0f64,
            pd_shou_ti_dui : 0.0f64,
            p_wei_ti_dui : 0.0f64,
            u_wei_ti_dui : 0.0f64,
            f_wei_ti_dui : 0.0f64,
            pd_wei_ti_dui : 0.0f64,
            p_quan_jian : 0.0f64,
            is_xiao_sheng : true,
            is_ying_da : true,
            ji_zu_vec : {
                let mut vec1 = vec![JiZu::new(0, JiJi::CY(jizu::ChaiYouJiJi::new())); 2];
                let mut vec2 = vec![JiZu::new(0, JiJi::QL(jizu::QiLunJiJi::new())); 4];
                let mut vec3 = vec![JiZu::new(0, JiJi::CY(jizu::ChaiYouJiJi::new())); 3];
                let mut vec4 = vec![JiZu::new(0, JiJi::AD(jizu::AnDianJiJi)); 4];
                vec1.append(&mut vec2);
                vec1.append(&mut vec3);
                vec1.append(&mut vec4);
                vec1
            },
            dian_zhan_vec : vec![DianZhan::new(0); simctrl::ZONG_SHU_DIAN_ZHAN],
            duan_lu_qi_vec : vec![DuanLuQi::new(0); simctrl::ZONG_SHU_DUAN_LU_QI],
            fu_zai_vec : vec![FuZai::new(0); simctrl::ZONG_SHU_FU_ZAI],
            node_vec : vec![Node::new(0); simctrl::ZONG_SHU_NODE],
            zhi_lu_vec : vec![ZhiLu::new(0); simctrl::ZONG_SHU_ZHI_LU],
            zhilu_node_vec : Vec::new(),
            zhilu_duanluqi_vec : Vec::new(),
            jizu_duanluqi_vec : Vec::new(),
            jizuchaiyou_duanluqi_vec : Vec::new(),
            jizuandian_duanluqi_vec : Vec::new(),
            jizuqilun_duanluqi_vec : Vec::new(),
            node_jizu_vec : Vec::new(),
            node_fuzai_vec : Vec::new(),
            jizu_dianzhan_vec : Vec::new(),
            fuzai_dianzhan_vec : Vec::new(),
            duanluqi_dianzhan_vec : Vec::new(),
            node_dianzhan_vec : Vec::new(),
            temp_node_vec : Vec::new(),
            node_group_vec : Vec::new(),
            jizunode_vec : Vec::new(),
            jizuduanluqi_vec : Vec::new(),
        };
        for i in 0..simctrl::ZONG_SHU_JI_ZU {
            x.ji_zu_vec[i].uid = i;
            match x.ji_zu_vec[i].ji_can_shu_ji {
                JiJi::CY(_) => x.ji_zu_vec[i].common_ji.bei_che_t = jizu::JI_ZU_CHAI_YOU_BEI_CHE_T,
                JiJi::QL(_) => x.ji_zu_vec[i].common_ji.bei_che_t = jizu::JI_ZU_QI_LUN_BEI_CHE_T,
                _ => x.ji_zu_vec[i].common_ji.bei_che_t = 0.0,
            }
        }
        x.ji_zu_vec.push(JiZu::new(simctrl::ZONG_SHU_JI_ZU_CHAI_YOU, JiJi::AD(jizu::AnDianJiJi)));
        for i in 0..simctrl::ZONG_SHU_DIAN_ZHAN {
            x.dian_zhan_vec[i].uid = i;
            x.dian_zhan_vec[i].ji_zu_num = 2;
            if i==3 {
                x.dian_zhan_vec[i].ji_zu_num = 3;
            }
        }
        for i in 0..simctrl::ZONG_SHU_DUAN_LU_QI {
            x.duan_lu_qi_vec[i].uid = i;
        }
        for i in 0..simctrl::ZONG_SHU_FU_ZAI {
            x.fu_zai_vec[i].uid = i;
        }
        for i in 0..simctrl::ZONG_SHU_NODE {
            x.node_vec[i].uid = i;
        }
        for i in 0..simctrl::ZONG_SHU_ZHI_LU {
            x.zhi_lu_vec[i].uid = i;
        }
        x.build_zhilu_node_vec();
        x.build_zhilu_duanluqi_vec();
        x.build_jizu_duanluqi_vec();
        x.build_jizuandian_duanluqi_vec();
        x.build_jizuchaiyou_duanluqi_vec();
        x.build_jizuqilun_duanluqi_vec();
        x.build_node_jizu_vec();
        x.build_node_fuzai_vec();
        x.build_jizu_dianzhan_vec();
        x.build_fuzai_dianzhan_vec();
        x.build_duanluqi_dianzhan_vec();
        x.build_node_dianzhan_vec();
        x.build_jizunode_vec();
        x.build_jizuduanluqi_vec();
        x.update_node_group_vec();
        // x.run(simctrl::FANG_ZHEN_BU_CHANG as u64, simctrl::FANG_ZHEN_BU_CHANG as u64);
        x
    }
    pub fn get_zhiluid_group_from_nodeid(&mut self, node_id : usize)->Option<Vec<usize>>{
        let mut zhiluid_group = Vec::new();
        for zhilu_node in self.zhilu_node_vec.to_vec() {
            if zhilu_node.1 == node_id {
                zhiluid_group.push(zhilu_node.0);
            }
        }
        if zhiluid_group.is_empty() {
            return None;
        }
        else {
            zhiluid_group.sort();
            zhiluid_group.dedup();
            return Some(zhiluid_group);
        }
    }

    pub fn get_zhiluid_group_from_nodeid_group(&mut self, node_id_group : Vec<usize>)->Option<Vec<usize>>{
        let mut zhiluid_group = Vec::new();
        for zhilu_node in self.zhilu_node_vec.to_vec() {
            for node_id in node_id_group.to_vec() {
                if zhilu_node.1 == node_id {
                    zhiluid_group.push(zhilu_node.0);
                }
            }
        }
        if zhiluid_group.is_empty() {
            return None;
        }
        else {
            zhiluid_group.sort();
            zhiluid_group.dedup();
            return Some(zhiluid_group);
        }
    }

    pub fn get_nodeid_group_from_zhiluid(&mut self, zhilu_id : usize)->Option<Vec<usize>>{
        let mut nodeid_group = Vec::new();
        for zhilu_node in self.zhilu_node_vec.to_vec() {
            if zhilu_node.0 == zhilu_id {
                nodeid_group.push(zhilu_node.1);
            }
        }
        if nodeid_group.is_empty() {
            return None;
        }
        else {
            nodeid_group.sort();
            nodeid_group.dedup();
            return Some(nodeid_group);
        }
    }

    pub fn get_nodeid_group_from_zhiluid_group(&mut self, zhilu_id_group : Vec<usize>)->Option<Vec<usize>>{
        let mut nodeid_group = Vec::new();
        for zhilu_node in self.zhilu_node_vec.to_vec() {
            for zhilu_id in zhilu_id_group.iter().cloned() {
                if zhilu_node.0 == zhilu_id {
                    nodeid_group.push(zhilu_node.1);
                }
            }
        }
        if nodeid_group.is_empty() {
            return None;
        }
        else {
            nodeid_group.sort();
            nodeid_group.dedup();
            return Some(nodeid_group);
        }
    }

    pub fn get_jizu_from_node(&mut self, _node : &node::Node)->Option<&mut jizu::JiZu<jizu::JiJi>>{
        match self.node_jizu_vec.iter()
        .cloned()
        .find(|node_jizu|node_jizu.0 == _node.uid) {
            Some(node_jizu) => return Some(&mut self.ji_zu_vec[node_jizu.1]),
            None => return None,
        }
    }

    pub fn get_jizu_from_nodeid(&mut self, _node_id : usize)->Option<&mut jizu::JiZu<jizu::JiJi>>{
        match self.node_jizu_vec.iter()
        .cloned()
        .find(|node_jizu|node_jizu.0 == _node_id) {
            Some(node_jizu) => return Some(&mut self.ji_zu_vec[node_jizu.1]),
            None => return None,
        }
    }

    pub fn get_jizuid_from_nodeid(&mut self, _node_id : usize)->Option<usize>{
        match self.node_jizu_vec.iter()
        .cloned()
        .find(|node_jizu|node_jizu.0 == _node_id) {
            Some(node_jizu) => return Some(node_jizu.1),
            None => return None,
        }
    }

    pub fn get_node_from_jizu(&mut self, ji_zu : &JiZu<JiJi>)->Option<&mut node::Node>{
        match self.node_jizu_vec.iter()
        .cloned()
        .find(|node_jizu|node_jizu.1 == ji_zu.uid) {
            Some(node_jizu) => return Some(&mut self.node_vec[node_jizu.0]),
            None => return None,
        }
    }

    pub fn get_nodeid_from_jizuid(&mut self, ji_zu_id : usize)->Option<usize>{
        match self.node_jizu_vec.iter()
        .cloned()
        .find(|node_jizu|node_jizu.1 == ji_zu_id) {
            Some(node_jizu) => return Some(node_jizu.0),
            None => return None,
        }
    }

    pub fn get_fuzaiid_vec_from_nodeid(&mut self, node_id : usize) -> Vec<usize>{
        self.node_fuzai_vec.iter()
        .cloned()
        .filter(|node_fuzai|node_fuzai.0 == node_id)
        .map(|node_fuzai|node_fuzai.1).collect()
    }

    pub fn get_nodeid_from_fuzaiid(&mut self, fu_zai_id : usize) -> Option<usize>{
        self.node_fuzai_vec.iter()
        .cloned()
        .find(|node_fuzai|node_fuzai.1 == fu_zai_id).map(|node_fuzai|node_fuzai.0)
    }

    pub fn get_pd_from_nodeid(&mut self, node_id : usize) -> f64 {
        self.node_fuzai_vec.to_vec().iter()
        .cloned()
        .filter(|node_fuzai|node_fuzai.0 == node_id)
        .map(|node_fuzai|self.fu_zai_vec[node_fuzai.1].p).sum()
    }

    pub fn get_pd_from_nodeid_vec(&mut self, node_id_vec : Vec<usize>) -> f64 {
        node_id_vec.iter().map(|&id|self.get_pd_from_nodeid(id)).sum()
    }

    pub fn get_pd_from_jizuid(&mut self, ji_zu_id : usize) -> f64 {
        let node_id = self.get_nodeid_from_jizuid(ji_zu_id).unwrap();
        self.node_fuzai_vec.to_vec().iter()
        .cloned()
        .filter(|node_fuzai|node_fuzai.0 == node_id)
        .map(|node_fuzai|self.fu_zai_vec[node_fuzai.1].p).sum()
    }

    pub fn get_pd_from_jizuid_vec(&mut self, ji_zu_id_vec : Vec<usize>) -> f64 {
        ji_zu_id_vec.iter().map(|&id|self.get_pd_from_jizuid(id)).sum()
    }

    //获取发电机断路器状态
    pub fn get_jizuduanluqi_status(&self, ji_zu : &JiZu<JiJi>) -> Option<duanluqi::DuanLuQiStatus> {
        self.jizu_duanluqi_vec.iter()
        .cloned()
        .find(|jizu_duanluqi|jizu_duanluqi.0 == ji_zu.uid)
        .map(|jizu_duanluqi|self.duan_lu_qi_vec[jizu_duanluqi.1].status)
    }
    //根据支路断路器状态判断支路通断情况
    pub fn update_zhilu_status(&mut self) {
        for zhi_lu in self.zhi_lu_vec.iter_mut() {
            match self.zhilu_duanluqi_vec.iter()
            .find(|zhilu_duanluqi|zhilu_duanluqi.0 == zhi_lu.uid) {
                Some(zhilu_duanluqi) => {
                    match self.duan_lu_qi_vec[zhilu_duanluqi.1].status {
                        duanluqi::DuanLuQiStatus::On{..} => zhi_lu.status = zhilu::ZhiLuStatus::On,
                        duanluqi::DuanLuQiStatus::Off{..} => zhi_lu.status = zhilu::ZhiLuStatus::Off,
                    }
                }
                None => {}
            }
        }
    }

    pub fn get_dianzhan_from_jizu(&mut self, ji_zu : &JiZu<JiJi>) -> Option<&mut dianzhan::DianZhan> {
        match self.jizu_dianzhan_vec.iter()
        .cloned()
        .find(|jizu_dianzhan|jizu_dianzhan.0 == ji_zu.uid) {
            Some(jizu_dianzhan) => return Some(&mut self.dian_zhan_vec[jizu_dianzhan.1]),
            None => return None,
        }
    }
    pub fn get_dianzhan_from_jizuid(&mut self, ji_zu_id : usize) -> Option<&mut dianzhan::DianZhan> {
        match self.jizu_dianzhan_vec.iter()
        .cloned()
        .find(|jizu_dianzhan|jizu_dianzhan.0 == ji_zu_id) {
            Some(jizu_dianzhan) => return Some(&mut self.dian_zhan_vec[jizu_dianzhan.1]),
            None => return None,
        }
    }
    pub fn get_jizuid_vec_from_dianzhanid(&mut self, dianzhanid : usize) -> Vec<usize> {
        self.jizu_dianzhan_vec.iter().filter(|jizu_dianzhan|jizu_dianzhan.1 == dianzhanid).map(|jizu_dianzhan|jizu_dianzhan.0).collect()
    }
    pub fn get_dianzhan_from_fuzai(&mut self, fu_zai : &fuzai::FuZai) -> Option<&mut dianzhan::DianZhan> {
        match self.fuzai_dianzhan_vec.iter()
        .cloned()
        .find(|fuzai_dianzhan|fuzai_dianzhan.0 == fu_zai.uid) {
            Some(fuzai_dianzhan) => return Some(&mut self.dian_zhan_vec[fuzai_dianzhan.1]),
            None => return None,
        }
    }
    pub fn get_dianzhan_from_duanluqi(&mut self, duan_lu_qi : &duanluqi::DuanLuQi) -> Option<&mut dianzhan::DianZhan> {
        match self.duanluqi_dianzhan_vec.iter()
        .cloned()
        .find(|duanluqi_dianzhan|duanluqi_dianzhan.0 == duan_lu_qi.uid) {
            Some(duanluqi_dianzhan) => return Some(&mut self.dian_zhan_vec[duanluqi_dianzhan.1]),
            None => return None,
        }
    }
    pub fn get_dianzhanid_from_duanluqiid(&mut self, duanluqiid:usize) -> Option<usize> {
        match self.duanluqi_dianzhan_vec.iter()
        .cloned()
        .find(|duanluqi_dianzhan|duanluqi_dianzhan.0 == duanluqiid) {
            Some(duanluqi_dianzhan) => return Some(duanluqi_dianzhan.1),
            None => return None,
        }
    }
    pub fn get_dianzhan_from_node(&mut self, _node : &node::Node) -> Option<&mut dianzhan::DianZhan> {
        match self.node_dianzhan_vec.iter()
        .cloned()
        .find(|node_dianzhan|node_dianzhan.0 == _node.uid) {
            Some(node_dianzhan) => return Some(&mut self.dian_zhan_vec[node_dianzhan.1]),
            None => return None,
        }
    }

    pub fn get_dianzhan_from_nodeid(&mut self, node_id : usize) -> Option<&mut dianzhan::DianZhan> {
        match self.node_dianzhan_vec.iter()
        .cloned()
        .find(|node_dianzhan|node_dianzhan.0 == node_id) {
            Some(node_dianzhan) => return Some(&mut self.dian_zhan_vec[node_dianzhan.1]),
            None => return None,
        }
    }

    pub fn get_dianzhanid_from_nodeid(&mut self, node_id : usize) -> Option<usize> {
        match self.node_dianzhan_vec.iter()
        .cloned()
        .find(|node_dianzhan|node_dianzhan.0 == node_id) {
            Some(node_dianzhan) => return Some(node_dianzhan.1),
            None => return None,
        }
    }

    pub fn get_jizu_from_duanluqi(&mut self, duan_lu_qi : &duanluqi::DuanLuQi) -> Option<&mut JiZu<JiJi>> {
        match self.jizu_duanluqi_vec.iter()
        .cloned()
        .find(|jizu_duanluqi|jizu_duanluqi.1 == duan_lu_qi.uid) {
            Some(jizu_duanluqi) => return Some(&mut self.ji_zu_vec[jizu_duanluqi.0]),
            None => return None,

        }
    }

    pub fn get_duanluqi_from_jizu(&mut self, ji_zu : &JiZu<JiJi>) -> Option<&mut duanluqi::DuanLuQi> {
        match self.jizu_duanluqi_vec.iter()
        .cloned()
        .find(|jizu_duanluqi|jizu_duanluqi.0 == ji_zu.uid) {
            Some(jizu_duanluqi) => return Some(&mut self.duan_lu_qi_vec[jizu_duanluqi.1]),
            None => return None,
        }
    }

    pub fn get_duanluqi_from_jizuid(&mut self, ji_zu_id : usize) -> Option<&mut duanluqi::DuanLuQi> {
        match self.jizu_duanluqi_vec.iter()
        .cloned()
        .find(|jizu_duanluqi|jizu_duanluqi.0 == ji_zu_id) {
            Some(jizu_duanluqi) => {
                return Some(&mut self.duan_lu_qi_vec[jizu_duanluqi.1]);
            },
            None => {
                return None;
            },
        }
    }

    pub fn get_duanluqiid_from_jizuid(&mut self, ji_zu_id : usize) -> Option<usize> {
        match self.jizu_duanluqi_vec.iter()
        .cloned()
        .find(|jizu_duanluqi|jizu_duanluqi.0 == ji_zu_id) {
            Some(jizu_duanluqi) => return Some(jizu_duanluqi.1),
            None => return None,
        }
    }

    pub fn get_duanluqi_from_zhilu(&mut self, zhi_lu : &zhilu::ZhiLu) -> Option<&mut duanluqi::DuanLuQi> {
        match self.zhilu_duanluqi_vec.iter().cloned()
        .find(|zhilu_duanluqi|zhilu_duanluqi.0 == zhi_lu.uid) {
            Some(zhilu_duanluqi) => return Some(&mut self.duan_lu_qi_vec[zhilu_duanluqi.1]),
            None => return None,
        }
    }
    pub fn get_duanluqiid_from_zhiluid(&mut self, zhi_lu_id : usize) -> Option<usize> {
        self.zhilu_duanluqi_vec.iter().cloned()
        .find(|zhilu_duanluqi|zhilu_duanluqi.0 == zhi_lu_id)
        .map(|zhilu_duanluqi|zhilu_duanluqi.1)
    }
    pub fn get_duanluqi_from_zhiluid(&mut self, zhi_lu_id : usize) -> Option<&mut duanluqi::DuanLuQi> {
        match self.zhilu_duanluqi_vec.iter().cloned()
        .find(|zhilu_duanluqi|zhilu_duanluqi.0 == zhi_lu_id) {
            Some(zhilu_duanluqi) => return Some(&mut self.duan_lu_qi_vec[zhilu_duanluqi.1]),
            None => return None,
        }
    }
    //判断两个节点是否连通
    pub fn is_nodes_connected(&mut self, n1 : &node::Node, n2 : &node::Node) -> bool {
        for group in self.node_group_vec.iter(){
            if group.contains(&(n1.uid)) && group.contains(&(n2.uid)) {
                return true;
            }
        }
        return false;
    }
    pub fn is_nodes_id_connected(&mut self, n1 : usize, n2 : usize) -> bool {
        for group in self.node_group_vec.iter(){
            if group.contains(&n1) && group.contains(&n2) {
                return true;
            }
        }
        return false;
    }

    //指定机组是否与其他机组并联运行
    pub fn is_ji_zu_bing_lian(&mut self, ji_zu : &JiZu<JiJi>) -> bool {
        let status = self.get_duanluqi_from_jizu(ji_zu).unwrap().status;
        match status {
            duanluqi::DuanLuQiStatus::On{..} => {
                let n1_id = self.get_node_from_jizu(ji_zu).unwrap().uid;
                for n2_id in self.jizunode_vec.to_vec(){
                    if n1_id != n2_id && self.is_nodes_id_connected(n1_id, n2_id) {
                        return true;
                    }
                }
                return false;
            }
            duanluqi::DuanLuQiStatus::Off{..} => return false,
        }
    }
    //获取与指定机组并联的所有机组，不包括指定机组
    pub fn get_jizuid_vec_bing_lian(&mut self, ji_zu : &JiZu<JiJi>) -> Vec<usize> {
        let status = self.get_duanluqi_from_jizu(ji_zu).unwrap().status;
        let mut jizuid_vec = Vec::new();
        match status {
            duanluqi::DuanLuQiStatus::On{..} => {
                let n1_id = self.get_node_from_jizu(ji_zu).unwrap().uid;
                for n2_id in 0..simctrl::ZONG_SHU_NODE{
                    if n1_id != n2_id && self.is_nodes_id_connected(n1_id, n2_id) {
                        let jizuid_option = self.get_jizuid_from_nodeid(n2_id);
                        if jizuid_option != None &&  self.get_duanluqi_from_jizuid(jizuid_option.unwrap()).unwrap().is_on() {
                            jizuid_vec.push(jizuid_option.unwrap());
                        }
                    }
                }

            }
            _ => {}
        }
        return jizuid_vec;
    }

    //获取与指定机组并联的所有机组，不包括指定机组
    pub fn get_jizuid_vec_bing_lian_from_id(&mut self, ji_zu_id : usize) -> Vec<usize> {
        let status = self.get_duanluqi_from_jizuid(ji_zu_id).unwrap().status;
        let mut jizuid_vec = Vec::new();
        match status {
            duanluqi::DuanLuQiStatus::On{..} => {
                let n1_id = self.get_nodeid_from_jizuid(ji_zu_id).unwrap();
                for n2_id in 0..simctrl::ZONG_SHU_NODE{
                    if n1_id != n2_id && self.is_nodes_id_connected(n1_id, n2_id) {
                        let jizuid_option = self.get_jizuid_from_nodeid(n2_id);
                        if jizuid_option != None &&  self.get_duanluqi_from_jizuid(jizuid_option.unwrap()).unwrap().is_on() {
                            jizuid_vec.push(jizuid_option.unwrap());
                        }
                    }
                }

            }
            _ => {}
        }
        return jizuid_vec;
    }

    ///获取与机组相关的节点组对应的所有非机组断路器
    pub fn get_duanluqiid_vec_related_to_jizu(&mut self, ji_zu : &JiZu<JiJi>) -> Option<Vec<usize>> {
        let mut duanluqiid_vec = Vec::new();
        let n1_id = self.get_node_from_jizu(ji_zu).unwrap().uid;
        let mut zhiluid_group = Vec::new();
        for group in self.node_group_vec.to_vec(){
            if group.contains(&n1_id) {
                zhiluid_group  = self.get_zhiluid_group_from_nodeid_group(group).unwrap();
            }
        }
        if zhiluid_group.is_empty() {
            return None;
        }
        else {
            for zhiluid in zhiluid_group {
                match self.get_duanluqiid_from_zhiluid(zhiluid) {
                    Some(duanluqiid) => duanluqiid_vec.push(duanluqiid),
                    None => return None,
                }
            }
            if duanluqiid_vec.is_empty() {
                return None;
            }
            else {
                return Some(duanluqiid_vec);
            }
        }
    }

    //比较一个非机组断路器合闸后系统拓扑变化情况，返回合并的节点组
    pub fn compare_two_xi_tong_he_zha_node(&mut self, xt_hou : &mut XiTong) -> Vec<Vec<usize>> {
        let mut node_id_group_vec = Vec::new();
        let mut is_finded = false;
        for i in 0..self.node_group_vec.len() {
            for j in 0..self.node_group_vec.len() {
                if xt_hou.is_nodes_id_connected(self.node_group_vec[i][0], self.node_group_vec[j][0]) {
                    is_finded = true;
                    node_id_group_vec.push(self.node_group_vec[i].to_vec());
                    node_id_group_vec.push(self.node_group_vec[j].to_vec());
                    break;
                }
            }
            if is_finded {
                break;
            }
        }
        return node_id_group_vec;
    }

    //比较一个非机组断路器合闸后系统拓扑变化情况,返回合并的电机组
    pub fn compare_two_xi_tong_he_zha_ji_zu(&mut self, xt_hou : &mut XiTong) -> Vec<Vec<usize>> {
        let node_id_group_vec_bing_che = self.compare_two_xi_tong_he_zha_node(xt_hou);
        let mut ji_zu_group_vec_bing_che = Vec::new();
        for i in node_id_group_vec_bing_che {
            let mut ji_zu_id_group = Vec::new();
            for j in i {
                let ji_zu_id_option = self.get_jizuid_from_nodeid(j);
                match ji_zu_id_option {
                    Some(ji_zu_id) =>{
                        let duanluqi_id = self.get_duanluqiid_from_jizuid(ji_zu_id).unwrap();
                        if self.duan_lu_qi_vec[duanluqi_id].is_off() {
                            ji_zu_id_group.push(ji_zu_id);
                        }
                    }
                    None =>{}
                }
            }
            ji_zu_group_vec_bing_che.push(ji_zu_id_group);
        }
        return ji_zu_group_vec_bing_che;
    }

    // QList<QList<uint> > compareTwoXiTongHeZhaFuZai(
    //         XiTong * xiTongQian, XiTong * xiTongHou);
    pub fn compare_two_xi_tong_he_zha_fu_zai(&mut self, xt_hou : &mut XiTong) -> Vec<Vec<usize>> {
        let node_id_group_vec_bing_che = self.compare_two_xi_tong_he_zha_node(xt_hou);
        let mut fu_zai_group_vec_bing_che = Vec::new();
        for i in node_id_group_vec_bing_che {
            let mut fu_zai_id_group = Vec::new();
            for j in i {
                let mut fu_zai_id = self.get_fuzaiid_vec_from_nodeid(j);
                fu_zai_id_group.append(&mut fu_zai_id);
            }
            fu_zai_group_vec_bing_che.push(fu_zai_id_group);
        }
        return fu_zai_group_vec_bing_che;
    }

    //由某一节点获取与此节点相关联的节点组
    pub fn get_node_group_from_node_id(&mut self, node_id : usize) -> Option<Vec<usize>> {
        for group in self.node_group_vec.to_vec() {
            if group.contains(&node_id) {
                return Some(group);
            }
        }
        return None;
    }

    //由某一负载获取与此负载相关联的所有并联机组
    pub fn get_ji_zu_group_from_fu_zai_id(&mut self, fu_zai_id : usize) -> Vec<usize> {
        let node_id = self.get_nodeid_from_fuzaiid(fu_zai_id).unwrap();
        let node_group = self.get_node_group_from_node_id(node_id).unwrap();
        let mut ji_zu_id_group = Vec::new();
        for node_id in node_group {
            let option_ji_zu_id = self.get_jizuid_from_nodeid(node_id);
            match option_ji_zu_id {
                Some(ji_zu_id) => {
                    if self.get_duanluqi_from_jizuid(ji_zu_id).unwrap().is_on() {
                        ji_zu_id_group.push(ji_zu_id);
                    }
                }
                None => {}
            }
        }
        return ji_zu_id_group;
    }

    /// 更新node_group_vec
    /// 1、利用节点uid列表集合node_id_vec
    /// 2、如果node_id_vec中存在元素，取出一个节点，构造一个新列表group
    /// 3、查找此节点的所有支路，判断支路是否闭合，若闭合则将支路另一端节点加入group
    /// 4、group循环元素递增1后，继续3、否则跳出，将其加入node_group_vec
    /// 5、继续2，否则完成构造。

    pub fn update_node_group_vec(&mut self){
        let mut node_id_vec : Vec<usize> = (0..simctrl::ZONG_SHU_NODE).collect();
        self.node_group_vec.clear();
        while !node_id_vec.is_empty() {
            let mut group : Vec<usize> = Vec::new();
            match node_id_vec.pop() {
                Some(node_id) => group.push(node_id),
                None => {},
            }
            let mut cur_i = 0;
            while cur_i < group.len() {
                let cur_node_id = group[cur_i];
                //找到当前节点关联的所有支路
                let zhilus = self.get_zhiluid_group_from_nodeid(cur_node_id).unwrap();
                for zhilu_id in zhilus {
                    //查找每条支路上的断路器
                    let duanluqi_id = self.get_duanluqiid_from_zhiluid(zhilu_id).unwrap();
                    //判断支路是否连通
                    if self.duan_lu_qi_vec[duanluqi_id].is_on() {
                        let mut nodes : Vec<usize> = self.get_nodeid_group_from_zhiluid(zhilu_id).unwrap();
                        nodes.retain(|&node_id| node_id != cur_node_id);
                        for node_id in nodes {
                            node_id_vec.retain(|&uid| uid != node_id);
                            if !group.contains(&node_id){
                                group.push(node_id);
                            }
                        }
                    }
                }
                cur_i += 1;
            }
            self.node_group_vec.push(group);
        }
    }
    ///判断某个子系统是否有回路
    /// 由于每个子系统均为连通图，所以连通图没有回路的充要条件是连通图为树，即连通图的支路数=节点数-1
    pub fn is_hui_lu_one_path(&mut self, node_id_group : &Vec<usize>)->bool {
        let node_group = (*node_id_group).to_vec();
        let node_group_len = node_group.len();
        let zhilu_group : Vec<usize> = self.get_zhiluid_group_from_nodeid_group(node_group).unwrap();
        if zhilu_group.len() < node_group_len {
            return true;
        }
        return false;
    }

    ///判断系统是否有回路
    pub fn is_hui_lu(&mut self) -> bool {
        let node_group_vec = self.node_group_vec.to_vec();
        for  group in node_group_vec {
            if self.is_hui_lu_one_path(&group) {
                return true;
            }
        }
        return false;
    }

    /// 判断断路器是否为机组断路器
    pub fn is_ji_zu_duan_lu_qi(&mut self, duanluqi_id : usize) -> bool {
        if self.jizuchaiyou_duanluqi_vec.to_vec().into_iter().find(|&x|x.1==duanluqi_id) != None || self.jizuqilun_duanluqi_vec.to_vec().into_iter().find(|&x|x.1==duanluqi_id) != None {
            return true;
        }
        return false;
    }

    /// 判断断路器是否为岸电断路器
    pub fn is_an_dian_duan_lu_qi(&mut self, duanluqi_id : usize) -> bool {
        if self.jizuandian_duanluqi_vec.to_vec().into_iter().find(|&x|x.1==duanluqi_id) != None {
            return true;
        }
        return false;
    }

    /// 根据电站uid得到电站中的所有节点
    pub fn get_node_vec_from_dian_zhan_id(&mut self, dianzhan_id : usize) -> Vec<usize> {
        self.node_dianzhan_vec.to_vec().iter().filter(|&t|t.1 == dianzhan_id).map(|&t|t.0).collect()
    }

    /// 根据电站uid得到与电站相关联的其他所有电站
    /// 1. 根据电站找到电站中的所有节点uid
    /// 2. 根据节点uid在node_group_vec中找到所有的相关节点uid
    /// 2. 根据所有相关节点uid找到对应电站uid
    /// 3. 电站uid排序去重，并去掉给定的电站
    pub fn get_dian_zhan_guan_lian_vec_from_dian_zhan_id(&mut self, dianzhan_id : usize) -> Vec<usize> {
        let n_vec : Vec<usize> = self.get_node_vec_from_dian_zhan_id(dianzhan_id);
        let node_group_vec = self.node_group_vec.to_vec();
        let mut node_vec_r : Vec<usize> = Vec::new();
        for node_group in node_group_vec {
            for n in n_vec.to_vec() {
                if node_group.contains(&n) {
                    node_vec_r.extend(node_group.iter().cloned());
                    break;
                }
            }
        }
        let mut d_vec_r : Vec<usize> = node_vec_r.iter().map(|&n|self.get_dianzhanid_from_nodeid(n).unwrap()).collect();
        d_vec_r.sort();
        d_vec_r.dedup();
        d_vec_r.iter().filter(|d|**d != dianzhan_id).map(|&x|x).collect()
    }
    /// 电站母联断路器是否全部连通
    /// 由电站uid获取节点uid组
    /// 去和node_group_vec中的元素一一对比，如果有相同的则返回true，否则返回false
    pub fn is_mu_lian_lian_tong(&mut self, dianzhan_id : usize) -> bool {
        let n_group : Vec<usize> = self.node_dianzhan_vec.to_vec().iter().filter(|n_d|n_d.1 == dianzhan_id).map(|n_d|n_d.0).collect();
        for ng in self.node_group_vec.to_vec() {
            if n_group == ng {
                return true;
            }
        }
        return false;
    }

    ///计算一个相互连通的电网络路径潮流,与CalculateBranch功能相同，
    ///返回值为1，说明潮流计算成功，返回值为0，说明有回路存在
    pub fn compute_path_pf(&mut self, i_group_list : Vec<usize>, pf_fang_fa : u32) {
        //当前负载在网有功,无功按(FU_ZAI_Q_P * 有功)计算，
        //key为负载uid，value为负载在网值
        let mut pd_online_vec : Vec<(usize, f64)> = Vec::new();
        //当前电源注入节点有功，key为节点uid, value为注入值，当无电源时，为0
        let mut pg_node_vec : Vec<(usize, f64)>  = Vec::new();
        //当前负载流出节点有功，key为节点uid, value为流出值，当无负载时，为0
        let mut pd_node_vec : Vec<(usize, f64)> = Vec::new();
        //路径中所有节点列表
        let all_node_in_path_vec = i_group_list.to_vec();
        //未计算节点列表
        let mut node_not_computed_vec : Vec<usize> = i_group_list.to_vec();
        let mut node_not_computed_0_zhi_lu_vec : Vec<usize> = Vec::new();
        let mut node_not_computed_1_zhi_lu_vec : Vec<usize> = Vec::new();
        let mut node_not_computed_2_zhi_lu_vec : Vec<usize> = Vec::new();
        let mut node_not_computed_duo_zhi_lu_vec : Vec<usize> = Vec::new();
        //所有已计算节点列表
        let mut node_computed_vec : Vec<usize> = Vec::new();
        //系统所有支路通断情况更新
        self.update_zhilu_status();
        //系统所有电机在网情况更新
        for i in 0..simctrl::ZONG_SHU_JI_ZU {
            if self.get_duanluqi_from_jizuid(i).unwrap().is_on() && self.ji_zu_vec[i].common_ji.uab_ext > 0.0 {
                self.ji_zu_vec[i].common_ji.is_online = true;
            }
            else {
                self.ji_zu_vec[i].common_ji.is_online = false;
            }
        }
        //路径中所有在网支路Map，key为节点，value为支路
        let mut node_zhi_lu_online_vec : Vec<(usize, usize)> = Vec::new();
        let mut node_zhi_lu_offline_vec : Vec<(usize, usize)> = Vec::new();
        let mut zhi_lu_online_vec : Vec<usize> = Vec::new();
        let mut zhi_lu_offline_vec : Vec<usize> = Vec::new();
        for z_n in self.zhilu_node_vec.to_vec() {
            if all_node_in_path_vec.contains(&(z_n.1)) {
                if self.zhi_lu_vec[z_n.0].status == ZhiLuStatus::On {
                    node_zhi_lu_online_vec.push((z_n.1, z_n.0));
                    if !zhi_lu_online_vec.contains(&(z_n.0)) {
                        zhi_lu_online_vec.push(z_n.0);
                    }
                }
                else {
                    node_zhi_lu_offline_vec.push((z_n.1, z_n.0));
                    if !zhi_lu_offline_vec.contains(&(z_n.0)) {
                        zhi_lu_offline_vec.push(z_n.0);
                    }
                }
            }
        }
        //路径中所有在网电机Map，key为节点，value为电机
        let mut node_ji_zu_online_vec : Vec<(usize, usize)> = Vec::new();
        let mut node_ji_zu_offline_vec : Vec<(usize, usize)> = Vec::new();
        let mut ji_zu_online_vec : Vec<usize> = Vec::new();
        let mut ji_zu_offline_vec : Vec<usize> = Vec::new();
        for n_j in self.node_jizu_vec.to_vec() {
            if all_node_in_path_vec.contains(&(n_j.0)) {
                if self.ji_zu_vec[n_j.1].common_ji.is_online {
                    node_ji_zu_online_vec.push(n_j);
                    if !ji_zu_online_vec.contains(&(n_j.1)) {
                        ji_zu_online_vec.push(n_j.1);
                    }
                }
                else {
                    node_ji_zu_offline_vec.push(n_j);
                    if !ji_zu_offline_vec.contains(&(n_j.1)) {
                        ji_zu_offline_vec.push(n_j.1);
                    }
                }
            }
        }

        let mut zhi_lu_computed_vec : Vec<usize> = Vec::new();
        let mut zhi_lu_not_computed_vec : Vec<usize> = zhi_lu_online_vec.to_vec();
        //构造节点各列表
        for n in all_node_in_path_vec.to_vec() {
            let iter = node_zhi_lu_online_vec.iter().filter(|n_z|n_z.0 == n);
            match iter.count(){
                0 => node_not_computed_0_zhi_lu_vec.push(n),
                1 => node_not_computed_1_zhi_lu_vec.push(n),
                2 => node_not_computed_2_zhi_lu_vec.push(n),
                _ => node_not_computed_duo_zhi_lu_vec.push(n),
            }
        }
        //所有不在网的电机、支路、断路器值为0
        for j in ji_zu_offline_vec {
            self.ji_zu_vec[j].common_ji.p = 0.0;
            self.ji_zu_vec[j].common_ji.q = 0.0;
            self.ji_zu_vec[j].common_ji.ia_ext = 0.0;
            self.ji_zu_vec[j].common_ji.ib_ext = 0.0;
            self.ji_zu_vec[j].common_ji.ic_ext = 0.0;
            self.ji_zu_vec[j].common_ji.ia_in = 0.0;
            self.ji_zu_vec[j].common_ji.ib_in = 0.0;
            self.ji_zu_vec[j].common_ji.ic_in = 0.0;
            let mut duanluqi = self.get_duanluqi_from_jizuid(j).unwrap();
            duanluqi.uab = 0.0;
            duanluqi.ubc = 0.0;
            duanluqi.uca = 0.0;
            duanluqi.ia = 0.0;
            duanluqi.ib = 0.0;
            duanluqi.ic = 0.0;
            duanluqi.f = 0.0;
        }
        for z in zhi_lu_offline_vec.to_vec() {
            self.zhi_lu_vec[z].p = 0.0;
            self.zhi_lu_vec[z].q = 0.0;
            self.zhi_lu_vec[z].i = 0.0;
            let mut duanluqi = self.get_duanluqi_from_zhiluid(z).unwrap();
            duanluqi.uab = 0.0;
            duanluqi.ubc = 0.0;
            duanluqi.uca = 0.0;
            duanluqi.ia = 0.0;
            duanluqi.ib = 0.0;
            duanluqi.ic = 0.0;
            duanluqi.f = 0.0;
        }
        //1. 是否存在在网机组，若不存在，所有参数为0，并返回1，若存在，所有节点值为电网值，转入2
        if ji_zu_online_vec.is_empty() {
            //所有节点参数为0
            for n in all_node_in_path_vec {
                self.node_vec[n].u = 0.0;
                self.node_vec[n].f = 0.0;
            }
            //所有支路参数为0
            //所有断路器参数为0
            for z in zhi_lu_online_vec {
                self.zhi_lu_vec[z].p = 0.0;
                self.zhi_lu_vec[z].p = 0.0;
                self.zhi_lu_vec[z].p = 0.0;
                let mut duanluqi = self.get_duanluqi_from_zhiluid(z).unwrap();
                duanluqi.uab = 0.0;
                duanluqi.ubc = 0.0;
                duanluqi.uca = 0.0;
                duanluqi.ia = 0.0;
                duanluqi.ib = 0.0;
                duanluqi.ic = 0.0;
                duanluqi.f = 0.0;
            }
            return ;
        }

        //确定所有节点参数
        for n in all_node_in_path_vec.to_vec() {
            self.node_vec[n].u = jizu::JI_ZU_E_DING_DIAN_YA;
            self.node_vec[n].f = jizu::JI_ZU_E_DING_PIN_LV;
        }
        //2. 由_chaoLiuFangFa选择潮流计算方法
        //3. _chaoLiuFangFa==0，由机组确定负载值，_chaoLiuFangFa==1，由负载确定机组值
        let fang_fa = pf_fang_fa;
        // for j_d in jizuandian_duanluqi_vec {
        //     if self.duan_lu_qi_vec[j_d.1].is_on() {
        //         fang_fa = 1;
        //         break;
        //     }
        // }
        if fang_fa == 0 {
            let mut all_ji_zu_p = 0.0f64;
            let mut all_fu_zai_p = 0.0f64;
            for j in ji_zu_online_vec.to_vec() {
                all_ji_zu_p += self.ji_zu_vec[j].common_ji.p;
            }
            for n in all_node_in_path_vec.to_vec() {
                let fuzaiid_vec = self.get_fuzaiid_vec_from_nodeid(n);
                for i in fuzaiid_vec {
                    all_fu_zai_p += self.fu_zai_vec[i].p;
                }
            }
            let p_delta = (all_fu_zai_p - all_ji_zu_p)/(ji_zu_online_vec.to_vec().len()) as f64;
            for j in ji_zu_online_vec.to_vec() {
                self.ji_zu_vec[j].common_ji.p += p_delta;
                self.ji_zu_vec[j].common_ji.q = self.ji_zu_vec[j].common_ji.p * jizu::JI_ZU_Q_P;
                self.ji_zu_vec[j].common_ji.p_factor = jizu::JI_ZU_P_FACTOR;
                let i : f64 = self.ji_zu_vec[j].common_ji.p * 1000.0 / (3.0f64.sqrt() * self.ji_zu_vec[j].common_ji.uab_ext * jizu::JI_ZU_P_FACTOR);
                self.ji_zu_vec[j].common_ji.ia_ext = i;
                self.ji_zu_vec[j].common_ji.ib_ext = i;
                self.ji_zu_vec[j].common_ji.ic_ext = i;
                self.ji_zu_vec[j].common_ji.ia_in = i;
                self.ji_zu_vec[j].common_ji.ib_in = i;
                self.ji_zu_vec[j].common_ji.ic_in = i;
            }
        }
        //3. _chaoLiuFangFa==1，由负载确定机组值
        else if fang_fa == 1 {
            let mut all_fu_zai_p = 0.0;
            //将负载值放入实时负载Map中
            for n_f in self.node_fuzai_vec.to_vec() {
                if all_node_in_path_vec.to_vec().contains(&(n_f.0)) {
                    all_fu_zai_p += self.fu_zai_vec[n_f.1].p;
                    pd_online_vec.push((n_f.1, self.fu_zai_vec[n_f.1].p));
                }
            }
            //机组均分功率
            let ji_zu_p_average = all_fu_zai_p/(ji_zu_online_vec.len()) as f64;
            for j in ji_zu_online_vec.to_vec() {
                let u : f64 = self.ji_zu_vec[j].common_ji.uab_ext;
                let i : f64 = ji_zu_p_average * 1000.0 / (3.0f64.sqrt() * u * jizu::JI_ZU_P_FACTOR);
                self.ji_zu_vec[j].common_ji.p = ji_zu_p_average;
                self.ji_zu_vec[j].common_ji.q = ji_zu_p_average * jizu::JI_ZU_Q_P;
                self.ji_zu_vec[j].common_ji.p_factor = jizu::JI_ZU_P_FACTOR;
                self.ji_zu_vec[j].common_ji.ia_ext = i;
                self.ji_zu_vec[j].common_ji.ib_ext = i;
                self.ji_zu_vec[j].common_ji.ic_ext = i;
                self.ji_zu_vec[j].common_ji.ia_in = i;
                self.ji_zu_vec[j].common_ji.ib_in = i;
                self.ji_zu_vec[j].common_ji.ic_in = i;
            }
        }
        else {
            return ;
        }
        for j in ji_zu_online_vec.to_vec() {
            let u : f64 = self.ji_zu_vec[j].common_ji.uab_ext;
            let i : f64 = self.ji_zu_vec[j].common_ji.ia_ext;
            let f : f64 = self.ji_zu_vec[j].common_ji.f_ext;
            let mut duanluqi = self.get_duanluqi_from_jizuid(j).unwrap();
            duanluqi.uab = u;
            duanluqi.ubc = u;
            duanluqi.uca = u;
            duanluqi.ia = i;
            duanluqi.ib = i;
            duanluqi.f = f;
        }
        for n in all_node_in_path_vec.to_vec() {
            let vec : Vec<(usize, usize)> = node_ji_zu_online_vec.iter().filter(|n_j|n_j.0 == n).map(|&n_j|n_j).collect();
            if !vec.is_empty() {
                let mut pg : f64 = 0.0;
                for n_j in vec {
                    pg += self.ji_zu_vec[n_j.1].common_ji.p;
                }
                pg_node_vec.push((n, pg));
            }
            else {
                pg_node_vec.push((n, 0.0));
            }
        }
        //得到节点注入和流出功率Map
        for n in all_node_in_path_vec.to_vec() {
            let fuzaiid_vec = self.get_fuzaiid_vec_from_nodeid(n);
            let pd_online : f64 = pd_online_vec.iter().filter(|f_p|fuzaiid_vec.contains(&(f_p.0))).map(|f_p|f_p.1).sum();
            pd_node_vec.push((n, pd_online));
        }
        //4. 若nodeWeiJiSuan0ZhiLuList不为空，而其他均为空，则计算结束，返回1
        if !node_not_computed_0_zhi_lu_vec.is_empty() && node_not_computed_1_zhi_lu_vec.is_empty() && node_not_computed_2_zhi_lu_vec.is_empty() && node_not_computed_duo_zhi_lu_vec.is_empty()
        {
            return ;
        }
        //4. 为简化计算，对当前节点而言，设已计算的支路潮流方向均为流入，待计算的为流出
        //4. 若node_not_computed_vec不为空
        while !node_not_computed_vec.is_empty() {
            //5. node_not_computed_vec遍历，找出一个可计算节点
            //5. 可计算节点判断条件：若为1支路节点，则可计算
            //5. 若为2支路节点，则至少有一个支路已计算
            //5. 若为n支路节点，则至少有（n-1）个支路已计算
            for n in node_not_computed_vec.to_vec() {
                if node_not_computed_1_zhi_lu_vec.contains(&n) {
                    node_not_computed_vec.retain(|&n1|n1 != n);
                    node_computed_vec.push(n);
                    //5. 若有未计算支路，计算之
                    let zhi_lu_id : usize = node_zhi_lu_online_vec.iter().find(|&n_z|n_z.0 == n).unwrap().1;
                    if zhi_lu_not_computed_vec.contains(&zhi_lu_id) {
                        //计算支路参数
                        let u = self.node_vec[n].u;
                        let f = self.node_vec[n].f;
                        let p = pg_node_vec.iter().find(|&n_p| n_p.0 == n).unwrap().1 - pd_node_vec.iter().find(|&n_p| n_p.0 == n).unwrap().1;
                        let i = p * 1000.0f64 / (3.0f64.sqrt() * u * jizu::JI_ZU_P_FACTOR);
                        self.zhi_lu_vec[zhi_lu_id].p = p;
                        self.zhi_lu_vec[zhi_lu_id].q = p * jizu::JI_ZU_Q_P;
                        self.zhi_lu_vec[zhi_lu_id].i = i;
                        //计算断路器参数
                        let duanluqi = self.get_duanluqi_from_zhiluid(zhi_lu_id).unwrap();
                        duanluqi.uab = u;
                        duanluqi.ubc = u;
                        duanluqi.uca = u;
                        duanluqi.ia = i;
                        duanluqi.ib = i;
                        duanluqi.f = f;
                    }
                    //将支路从未计算列表移至已计算列表
                    zhi_lu_not_computed_vec.retain(|&z|z != zhi_lu_id);
                    zhi_lu_computed_vec.push(zhi_lu_id);
                }
                else if node_not_computed_2_zhi_lu_vec.contains(&n) {
                    let zhi_lu_related_vec = self.get_zhiluid_group_from_nodeid(n).unwrap();
                    let mut i_zhi_lu = usize::max_value();
                    let mut i_zhi_lu_g = usize::max_value();
                    if zhi_lu_computed_vec.contains(&(zhi_lu_related_vec[0])) {
                        i_zhi_lu = zhi_lu_related_vec[1];
                        i_zhi_lu_g = zhi_lu_related_vec[0];
                    }
                    else if zhi_lu_computed_vec.contains(&(zhi_lu_related_vec[1])) {
                        i_zhi_lu = zhi_lu_related_vec[0];
                        i_zhi_lu_g = zhi_lu_related_vec[1];
                    }
                    if i_zhi_lu != usize::max_value() {
                        node_not_computed_vec.retain(|&node|node != n);
                        node_computed_vec.push(n);
                        if !zhi_lu_computed_vec.contains(&i_zhi_lu) {
                            //计算支路参数
                            let u = self.node_vec[n].u;
                            let f = self.node_vec[n].f;
                            let p = pg_node_vec.iter().find(|&n_pg|n_pg.0 == n).unwrap().1 + self.zhi_lu_vec[i_zhi_lu_g].p - pd_node_vec.iter().find(|&n_pd|n_pd.0 == n).unwrap().1;
                            let i = p * 1000.0 / (3.0f64.sqrt() * u * jizu::JI_ZU_P_FACTOR);
                            self.zhi_lu_vec[i_zhi_lu].p = p;
                            self.zhi_lu_vec[i_zhi_lu].q = jizu::JI_ZU_Q_P * p;
                            self.zhi_lu_vec[i_zhi_lu].i = i;
                            //计算断路器参数
                            let duanluqi = self.get_duanluqi_from_zhiluid(i_zhi_lu).unwrap();
                            duanluqi.uab = u;
                            duanluqi.ubc = u;
                            duanluqi.uca = u;
                            duanluqi.ia = i;
                            duanluqi.ib = i;
                            duanluqi.f = f;
                            //将支路从未计算列表移至已计算列表
                            zhi_lu_not_computed_vec.retain(|&zhi_lu|zhi_lu != i_zhi_lu);
                            zhi_lu_computed_vec.push(i_zhi_lu);
                        }
                    }
                }
                else if node_not_computed_duo_zhi_lu_vec.contains(&n) {
                    let zhi_lu_related_vec = self.get_zhiluid_group_from_nodeid(n).unwrap();
                    let mut i_zhi_lu = usize::max_value();
                    let mut zhi_lu_not_computed_num = 0;
                    let mut all_zhi_lu_g_p = 0.0f64;
                    for z in  zhi_lu_related_vec {
                        if zhi_lu_not_computed_vec.contains(&z) {
                            i_zhi_lu = z;
                            zhi_lu_not_computed_num += 1;
                        }
                        else {
                            all_zhi_lu_g_p += self.zhi_lu_vec[z].p;
                        }
                    }
                    node_not_computed_vec.retain(|&node|node != n);
                    node_computed_vec.push(n);
                    if zhi_lu_not_computed_num == 1 {
                        //计算支路参数
                        let u = self.node_vec[n].u;
                        let f = self.node_vec[n].f;
                        let p = pg_node_vec.iter().find(|&n_pg|n_pg.0 == n).unwrap().1 + all_zhi_lu_g_p - pd_node_vec.iter().find(|&n_pd|n_pd.0 == n).unwrap().1;
                        let i = p * 1000.0 / (3.0f64.sqrt() * u * jizu::JI_ZU_P_FACTOR);
                        self.zhi_lu_vec[i_zhi_lu].p = p;
                        self.zhi_lu_vec[i_zhi_lu].q = jizu::JI_ZU_Q_P * p;
                        self.zhi_lu_vec[i_zhi_lu].i = i;
                        //计算断路器参数
                        let duanluqi = self.get_duanluqi_from_zhiluid(i_zhi_lu).unwrap();
                        duanluqi.uab = u;
                        duanluqi.ubc = u;
                        duanluqi.uca = u;
                        duanluqi.ia = i;
                        duanluqi.ib = i;
                        duanluqi.f = f;
                        //将支路从未计算列表移至已计算列表
                        zhi_lu_not_computed_vec.retain(|&zhi_lu|zhi_lu != i_zhi_lu);
                        zhi_lu_computed_vec.push(i_zhi_lu);
                    }
                }
            }
            //6. 根据已知条件计算可计算节点关联的唯一未知参数支路
            //7. 将已计算的节点和支路放入已计算列表，并从未计算列表中去除
            //8. 继续遍历，直到nodeWeiJiSuanList为空
        }
    }

    //计算系统的潮流
    pub fn compute_xi_tong_pf(&mut self) {
        //首先更新断路器状态，由于断路器在闭合和分断时有准备并车和准备解列两个子状态
        for i in 0..simctrl::ZONG_SHU_DUAN_LU_QI {
            if (self.duan_lu_qi_vec[i].is_ready_to_bing_che() || self.duan_lu_qi_vec[i].is_ready_to_jie_lie() ) && (self.is_ji_zu_duan_lu_qi(i) || self.is_an_dian_duan_lu_qi(i) ) {
                match self.duan_lu_qi_vec[i].status {
                    duanluqi::DuanLuQiStatus::Off{..} => self.duan_lu_qi_vec[i].set_on(),
                    duanluqi::DuanLuQiStatus::On{..} => self.duan_lu_qi_vec[i].set_off(),
                }
            }
            else if self.duan_lu_qi_vec[i].is_ready_to_bing_che() && !(self.is_ji_zu_duan_lu_qi(i) || self.is_an_dian_duan_lu_qi(i) ) {
                let mut xt_temp = self.clone();
                match xt_temp.duan_lu_qi_vec[i].status {
                    duanluqi::DuanLuQiStatus::Off{..} => xt_temp.duan_lu_qi_vec[i].set_on(),
                    duanluqi::DuanLuQiStatus::On{..} => {},
                }
                xt_temp.update_node_group_vec();
                let jizu_vec_vec_bing_che = self.compare_two_xi_tong_he_zha_ji_zu(&mut xt_temp);
                let mut is_all_ji_zu_wen = true;
                for j in jizu_vec_vec_bing_che {
                    for k in j {
                        if self.ji_zu_vec[k].common_ji.current_range != jizu::JiZuRangeLeiXing::Wen {
                            is_all_ji_zu_wen = false;
                            break;
                        }
                    }
                    if !is_all_ji_zu_wen {
                        break;
                    }
                }
                if is_all_ji_zu_wen {
                    match self.duan_lu_qi_vec[i].status {
                        duanluqi::DuanLuQiStatus::Off{..} => self.duan_lu_qi_vec[i].set_on(),
                        _ => {},
                    }
                }
            }
            else if self.duan_lu_qi_vec[i].is_ready_to_jie_lie() && !(self.is_ji_zu_duan_lu_qi(i) || self.is_an_dian_duan_lu_qi(i) ) {
                let mut xt_temp = self.clone();
                match xt_temp.duan_lu_qi_vec[i].status {
                    duanluqi::DuanLuQiStatus::On{..} => xt_temp.duan_lu_qi_vec[i].set_off(),
                    _ => {},
                }
                xt_temp.update_node_group_vec();
                let jizu_vec_vec_bing_che = xt_temp.compare_two_xi_tong_he_zha_ji_zu(self);
                let mut is_all_ji_zu_wen = true;
                for j in jizu_vec_vec_bing_che {
                    for k in j {
                        if self.ji_zu_vec[k].common_ji.current_range != jizu::JiZuRangeLeiXing::Wen {
                            is_all_ji_zu_wen = false;
                            break;
                        }
                    }
                    if !is_all_ji_zu_wen {
                        break;
                    }
                }
                if is_all_ji_zu_wen {
                    match self.duan_lu_qi_vec[i].status {
                        duanluqi::DuanLuQiStatus::On{..} => self.duan_lu_qi_vec[i].set_off(),
                        _ => {},
                    }
                }
            }
        }
        //更新系统拓扑
        self.update_node_group_vec();
        let mut pf_fang_fa_vec : Vec<(Vec<usize>, u32)> = Vec::new();
        for n_g in self.node_group_vec.to_vec() {
            let mut fang_fa = 1u32;
            for n in n_g.to_vec() {
                let option_ji_zu_id = self.get_jizuid_from_nodeid(n);
                match option_ji_zu_id {
                    Some(ji_zu_id) => {
                        let option_duan_lu_qi_id = self.get_duanluqiid_from_jizuid(ji_zu_id);
                        match option_duan_lu_qi_id {
                            Some(duan_lu_qi_id) => {
                                let ji_zu = self.ji_zu_vec[ji_zu_id];
                                let duan_lu_qi = self.duan_lu_qi_vec[duan_lu_qi_id];
                                if duan_lu_qi.is_on() && !(ji_zu.common_ji.current_range == jizu::JiZuRangeLeiXing::TingJi || ji_zu.common_ji.current_range == jizu::JiZuRangeLeiXing::BeiCheZanTai || ji_zu.common_ji.current_range == jizu::JiZuRangeLeiXing::BeiCheWanBi || ji_zu.common_ji.current_range == jizu::JiZuRangeLeiXing::Wen) {
                                    fang_fa = 0;
                                    break;
                                }
                            }
                            None => {}
                        }
                    }
                    None => {},
                }
                let option_dian_zhan = self.get_dianzhan_from_nodeid(n);
                match option_dian_zhan {
                    Some(&mut dian_zhan) => {
                        if dian_zhan.ctrl_mode == simctrl::CtrlMode::Manual {
                            fang_fa = 0;
                        }
                    }
                    None => {}
                }
            }
            pf_fang_fa_vec.push((n_g,fang_fa));
        }
        for fu_zai_id in 0..simctrl::ZONG_SHU_FU_ZAI {
            let nodeid = self.get_nodeid_from_fuzaiid(fu_zai_id).unwrap();
            let u = self.node_vec[nodeid].u;
            if u != 0.0 && self.fu_zai_vec[fu_zai_id].is_online {
                self.fu_zai_vec[fu_zai_id].i = self.fu_zai_vec[fu_zai_id].p * 1000f64 /(3f64.sqrt() * u * fuzai::FU_ZAI_P_FACTOR);
            }
            else {
                self.fu_zai_vec[fu_zai_id].is_online = false;
                self.fu_zai_vec[fu_zai_id].p = 0.0;
                self.fu_zai_vec[fu_zai_id].q = 0.0;
                self.fu_zai_vec[fu_zai_id].i = 0.0;
            }
        }
        for n_g_f in pf_fang_fa_vec {
             self.compute_path_pf(n_g_f.0, n_g_f.1);
        }
        //计算完毕为各电站电网参数赋值,同时重新设置电站控制方式
        for i in 0..simctrl::ZONG_SHU_DIAN_ZHAN {
            let n_vec = self.get_node_vec_from_dian_zhan_id(i);
            let mut u = 0.0f64;
            let mut f = 0.0f64;
            for n in n_vec {
                if u < self.node_vec[n].u {
                    u = self.node_vec[n].u;
                    f = self.node_vec[n].f;
                }
            }
            self.dian_zhan_vec[i].u = u;
            self.dian_zhan_vec[i].f = f;
            let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(i);
            self.dian_zhan_vec[i].p = 0.0;
            for jizuid in jizuid_vec{
                self.dian_zhan_vec[i].p += self.ji_zu_vec[jizuid].common_ji.p;
            }
            self.dian_zhan_vec[i].p_yu_du = 1.0f64 - self.dian_zhan_vec[i].p/(jizu::JI_ZU_E_DING_GONG_LV * self.dian_zhan_vec[i].ji_zu_num as f64);
        }
    }

    pub fn is_an_dian(&mut self, jizu : &JiZu<JiJi>) -> bool {
        match jizu.ji_can_shu_ji {
            JiJi::AD(..) => return true,
            _ => return false,
        }
    }

    pub fn is_chai_you(&mut self, jizu : &JiZu<JiJi>) -> bool {
        match jizu.ji_can_shu_ji {
            JiJi::CY(..) => return true,
            _ => return false,
        }
    }

    pub fn is_qi_lun(&mut self, jizu : &JiZu<JiJi>) -> bool {
        match jizu.ji_can_shu_ji {
            JiJi::QL(..) => return true,
            _ => return false,
        }
    }

    pub fn is_an_dian_by_id(&mut self, jizuid : usize) -> bool {
        match jizuid {
            0...simctrl::ZONG_SHU_JI_ZU =>{
                match self.ji_zu_vec[jizuid].ji_can_shu_ji {
                    JiJi::AD(..) => return true,
                    _ => return false,
                }
            }
            _ => return false,
        }
    }

    pub fn is_chai_you_by_id(&mut self, jizuid : usize) -> bool {
        match jizuid {
            0...simctrl::ZONG_SHU_JI_ZU =>{
                match self.ji_zu_vec[jizuid].ji_can_shu_ji {
                    JiJi::CY(..) => return true,
                    _ => return false,
                }
            }
            _ => return false,
        }
    }

    pub fn is_qi_lun_by_id(&mut self, jizuid : usize) -> bool {
        match jizuid {
            0...simctrl::ZONG_SHU_JI_ZU =>{
                match self.ji_zu_vec[jizuid].ji_can_shu_ji {
                    JiJi::QL(..) => return true,
                    _ => return false,
                }
            }
            _ => return false,
        }
    }

    pub fn set_ji_zu_vec_bian_zai_params(&mut self, v : Vec<usize>, p : f64) -> bool {
        let mut r = true;
        for j in v {
            let p_delta = p - self.ji_zu_vec[j].common_ji.p;
            match self.ji_zu_vec[j].common_ji.current_range {
                jizu::JiZuRangeLeiXing::Wen | jizu::JiZuRangeLeiXing::BianSu => r = self.ji_zu_vec[j].set_bian_zai_params(p_delta),
                _ =>  r = false,
            }
        }
        return r;
    }

    pub fn is_dev_id_valid(&mut self, dev_id : usize, dev_type : simctrl::DevType) -> bool {
        match dev_type {
            simctrl::DevType::JiZu => {
                match dev_id {
                    0...simctrl::ZONG_SHU_JI_ZU => {
                        if self.is_chai_you_by_id(dev_id) || self.is_qi_lun_by_id(dev_id) {
                            return true;
                        }
                        else {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }

            simctrl::DevType::DuanLuQi => {
                match dev_id {
                    0...simctrl::ZONG_SHU_DUAN_LU_QI => return true,
                    _ => return false,
                }
            }

            simctrl::DevType::FuZai => {
                match dev_id {
                    0...simctrl::ZONG_SHU_FU_ZAI => return true,
                    _ => return false,
                }
            }

            simctrl::DevType::DianZhan => {
                match dev_id {
                    0...simctrl::ZONG_SHU_DIAN_ZHAN => return true,
                    _ => return false,
                }
            }

            simctrl::DevType::AnDian => {
                match dev_id {
                    0...simctrl::ZONG_SHU_JI_ZU => {
                        if self.is_an_dian_by_id(dev_id) {
                            return true;
                        }
                        else {
                            return false;
                        }
                    }
                    _ => return false,
                }
            }

            simctrl::DevType::Node => {
                match dev_id {
                    0...simctrl::ZONG_SHU_NODE => return true,
                    _ => return false,
                }
            }

            simctrl::DevType::ZhiLu => {
                match dev_id {
                    0...simctrl::ZONG_SHU_ZHI_LU => return true,
                    _ => return false,
                }
            }
            simctrl::DevType::Wu => return false,
        }
    }

    pub fn is_zhi_ling_valid(&mut self, zl : &ZhiLing) -> bool {
        //let zhan_wei_type = zhiLing.zhan_wei_type;
        if !self.is_dev_id_valid(zl.dev_id, zl.dev_type) {
            self.handle_zhi_ling_result_msg(Err(zhiling::YingDaErr::IdNotMatch(*zl, String::from(zhiling::ID_NOT_MATCH_DESC), String::from(zhiling::CAUSE_ID_NOT_MATCH))));
            return false;
        }
        match zl.zhan_wei_type {
            simctrl::ZhanWeiType::Admin | simctrl::ZhanWeiType::JiaoLian | simctrl::ZhanWeiType::Internal => {
                self.handle_zhi_ling_result_msg(Ok(zhiling::YingDaType::Valid(*zl)));
                return true;
            }
            _ => {
                match zl.dev_type {
                    simctrl::DevType::JiZu => {
                        if zl.zhan_wei_type == simctrl::ZhanWeiType::JiPang {
                            self.handle_zhi_ling_result_msg(Ok(zhiling::YingDaType::Valid(*zl)));
                            return true;
                        }
                        else if zl.zhan_wei_type == simctrl::ZhanWeiType::ZhuKongZhiPing && self.ji_zu_vec[zl.dev_id].common_ji.operating_station == simctrl::OperatingStation::Remote {
                            self.handle_zhi_ling_result_msg(Ok(zhiling::YingDaType::Valid(*zl)));
                            return true;
                        }
                        else if (zl.zhan_wei_type == simctrl::ZhanWeiType::DianZhanJianKongTai || zl.zhan_wei_type == simctrl::ZhanWeiType::JiZuKongZhiQi || zl.zhan_wei_type == simctrl::ZhanWeiType::DianZhanKongZhiQi) && self.ji_zu_vec[zl.dev_id].common_ji.operating_station == simctrl::OperatingStation::Remote && self.get_dianzhan_from_jizuid(zl.dev_id).unwrap().operating_station == simctrl::OperatingStation::Local && ( self.get_dianzhan_from_jizuid(zl.dev_id).unwrap().ctrl_mode == simctrl::CtrlMode::SemiAuto || self.get_dianzhan_from_jizuid(zl.dev_id).unwrap().ctrl_mode == simctrl::CtrlMode::Auto ) {
                            self.handle_zhi_ling_result_msg(Ok(zhiling::YingDaType::Valid(*zl)));
                            return true;
                        }
                        else if (zl.zhan_wei_type == simctrl::ZhanWeiType::JiKongTai || zl.zhan_wei_type == simctrl::ZhanWeiType::BeiYongJiKongTai) && self.ji_zu_vec[zl.dev_id].common_ji.operating_station == simctrl::OperatingStation::Remote && self.get_dianzhan_from_jizuid(zl.dev_id).unwrap().operating_station == simctrl::OperatingStation::JiKong && ( self.get_dianzhan_from_jizuid(zl.dev_id).unwrap().ctrl_mode == simctrl::CtrlMode::SemiAuto || self.get_dianzhan_from_jizuid(zl.dev_id).unwrap().ctrl_mode == simctrl::CtrlMode::Auto ) {
                            self.handle_zhi_ling_result_msg(Ok(zhiling::YingDaType::Valid(*zl)));
                            return true;
                        }
                        else {
                            self.handle_zhi_ling_result_msg( Err(YingDaErr::CtrlModeAndOperatingStationFail(*zl, String::from(zhiling::CTRL_MODE_AND_OPERATING_STATION_FAIL_DESC), String::from(zhiling::CAUSE_CTRL_MODE_AND_OPERATING_STATION_INVALID))));
                            return false;
                        }
                    }
                    simctrl::DevType::DuanLuQi | simctrl::DevType::DianZhan => {
                        let mut dian_zhan_id = usize::max_value();
                        if zl.dev_type == simctrl::DevType::DuanLuQi {
                            if self.is_an_dian_duan_lu_qi(zl.dev_id) {
                                self.handle_zhi_ling_result_msg(Ok(zhiling::YingDaType::Valid(*zl)));
                            }
                            dian_zhan_id = self.get_dianzhanid_from_duanluqiid(zl.dev_id).unwrap();
                        }
                        else if zl.dev_type == simctrl::DevType::DianZhan {
                            dian_zhan_id = zl.dev_id;
                        }
                        if zl.zhan_wei_type == simctrl::ZhanWeiType::JiPang || zl.zhan_wei_type == simctrl::ZhanWeiType::ZhuKongZhiPing {
                            self.handle_zhi_ling_result_msg(Ok(zhiling::YingDaType::Valid(*zl)));
                            return true;
                        }
                        else if (zl.zhan_wei_type == simctrl::ZhanWeiType::DianZhanJianKongTai || zl.zhan_wei_type == simctrl::ZhanWeiType::JiZuKongZhiQi || zl.zhan_wei_type == simctrl::ZhanWeiType::DianZhanKongZhiQi) && self.dian_zhan_vec[dian_zhan_id].operating_station == simctrl::OperatingStation::Local && ( self.dian_zhan_vec[dian_zhan_id].ctrl_mode == simctrl::CtrlMode::SemiAuto || self.dian_zhan_vec[dian_zhan_id].ctrl_mode == simctrl::CtrlMode::Auto ) {
                            self.handle_zhi_ling_result_msg(Ok(zhiling::YingDaType::Valid(*zl)));
                            return true;
                        }
                        else if (zl.zhan_wei_type == simctrl::ZhanWeiType::JiKongTai || zl.zhan_wei_type == simctrl::ZhanWeiType::BeiYongJiKongTai) &&  self.dian_zhan_vec[dian_zhan_id].operating_station == simctrl::OperatingStation::JiKong && ( self.dian_zhan_vec[dian_zhan_id].ctrl_mode == simctrl::CtrlMode::SemiAuto || self.dian_zhan_vec[dian_zhan_id].ctrl_mode == simctrl::CtrlMode::Auto ) {
                            self.handle_zhi_ling_result_msg(Ok(zhiling::YingDaType::Valid(*zl)));
                            return true;
                        }
                        else {
                            self.handle_zhi_ling_result_msg( Err(YingDaErr::CtrlModeAndOperatingStationFail(*zl, String::from(zhiling::CTRL_MODE_AND_OPERATING_STATION_FAIL_DESC), String::from(zhiling::CAUSE_CTRL_MODE_AND_OPERATING_STATION_INVALID))));
                            return false;
                        }
                    }
                    simctrl::DevType::FuZai | simctrl::DevType::AnDian => {
                        self.handle_zhi_ling_result_msg(Ok(zhiling::YingDaType::Valid(*zl)));
                        return true;
                    }
                    _ => {
                        self.handle_zhi_ling_result_msg(Err(YingDaErr::CtrlModeAndOperatingStationFail(*zl, String::from(zhiling::CTRL_MODE_AND_OPERATING_STATION_FAIL_DESC), String::from(zhiling::CAUSE_CTRL_MODE_AND_OPERATING_STATION_INVALID) )));
                        return false;
                    }
                }
            }
        }
    }

    pub fn handle_zhi_ling(&mut self, zl : &ZhiLing) {
        self.handle_zhi_ling_result_msg(Ok(YingDaType::ACK(*zl)));
        match zl.dev_type {
            simctrl::DevType::JiZu => {
                match zl.zhi_ling_type {
                    ZhiLingType::BeiChe => self.handle_bei_che(zl),
                    ZhiLingType::OperatingStation(simctrl::OperatingStation::JiPang) | ZhiLingType::OperatingStation(simctrl::OperatingStation::Remote) => self.handle_operating_station(zl),
                    ZhiLingType::Prio => self.handle_prio(zl),
                    ZhiLingType::GenerateYiBanGuZhang(..) => self.handle_ji_zu_yi_ban_gu_zhang(zl),
                    ZhiLingType::EliminateYiBanGuZhang(..) => self.eliminate_ji_zu_yi_ban_gu_zhang(zl),

                    ZhiLingType::GenerateYiJiGuZhang(..) => self.handle_ji_zu_yi_ji_gu_zhang(zl),
                    ZhiLingType::EliminateYiJiGuZhang(..) => self.eliminate_ji_zu_yi_ji_gu_zhang(zl),

                    ZhiLingType::GenerateErJiGuZhang(..) => self.handle_ji_zu_er_ji_gu_zhang(zl),
                    ZhiLingType::EliminateErJiGuZhang(..) => self.eliminate_ji_zu_er_ji_gu_zhang(zl),

                    ZhiLingType::GenerateQiTaGuZhang(..) => self.handle_ji_zu_qi_ta_gu_zhang(zl),
                    ZhiLingType::EliminateQiTaGuZhang(..) => self.eliminate_ji_zu_qi_ta_gu_zhang(zl),

                    ZhiLingType::JinJiTingJi => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_jin_ji_ting_ji(zl);
                        }
                    }
                    ZhiLingType::QiDong => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_qi_dong(zl);
                        }
                    }
                    ZhiLingType::TingJi =>  {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_ting_ji(zl);
                        }
                    }
                    ZhiLingType::TouRu =>  {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_tou_ru(zl);
                        }
                    }
                    ZhiLingType::TuiChu => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_tui_chu(zl);
                        }
                    }
                    ZhiLingType::HeZhaBingChe =>  {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_he_zha_bing_che(zl);
                        }
                    }
                    ZhiLingType::FenZhaJieLie =>  {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_fen_zha_jie_lie(zl);
                        }
                    }
                    ZhiLingType::BianSu(..) =>  {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_bian_su(zl);
                        }
                    }
                    ZhiLingType::BianYa(..) =>  {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_bian_ya(zl);
                        }
                    }
                    ZhiLingType::XiaoSheng =>  {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_xiao_sheng(zl);
                        }
                    }
                    ZhiLingType::YingDa =>  {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_ying_da(zl);
                        }
                    }

                    _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::Invalid(*zl, String::from(zhiling::COMMON_INVALID_DESC), String::from(zhiling::CAUSE_COMMON_INVALID)))),
                }
            }
            simctrl::DevType::DianZhan => {
                match zl.zhi_ling_type {
                    ZhiLingType::BeiChe => self.handle_bei_che(zl),
                    ZhiLingType::OperatingStation(simctrl::OperatingStation::Local) | ZhiLingType::OperatingStation(simctrl::OperatingStation::JiKong) => self.handle_operating_station(zl),
                    ZhiLingType::CtrlMode(..) => self.handle_ctrl_mode(zl),
                    ZhiLingType::Prio => self.handle_prio(zl),

                    ZhiLingType::XiaoSheng => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_xiao_sheng(zl);
                        }
                    }
                    ZhiLingType::YingDa =>  {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_ying_da(zl);
                        }
                    }

                    _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::Invalid(*zl, String::from(zhiling::COMMON_INVALID_DESC), String::from(zhiling::CAUSE_COMMON_INVALID)))),
                }
            }
            simctrl::DevType::DuanLuQi => {
                match zl.zhi_ling_type {
                    ZhiLingType::HeZhaBingChe => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_he_zha_bing_che_duan_lu_qi(zl);
                        }
                    }
                    ZhiLingType::FenZhaJieLie => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_fen_zha_jie_lie_duan_lu_qi(zl);
                        }
                    }

                    ZhiLingType::XiaoSheng => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_xiao_sheng(zl);
                        }
                    }
                    ZhiLingType::YingDa => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_ying_da(zl);
                        }
                    }

                    _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::Invalid(*zl, String::from(zhiling::COMMON_INVALID_DESC), String::from(zhiling::CAUSE_COMMON_INVALID)))),
                }
            }
            simctrl::DevType::AnDian => {
                match zl.zhi_ling_type {
                    ZhiLingType::AnDianOn => self.handle_an_dian_on(zl),
                    ZhiLingType::AnDianOff => self.handle_an_dian_off(zl),
                    ZhiLingType::HeZhaBingChe => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_an_dian_he_zha(zl);
                        }
                    }
                    ZhiLingType::FenZhaJieLie => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_an_dian_fen_zha(zl);
                        }
                    }

                    ZhiLingType::XiaoSheng => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_xiao_sheng(zl);
                        }
                    }
                    ZhiLingType::YingDa => {
                        if self.is_zhi_ling_valid(zl) {
                            self.handle_ying_da(zl);
                        }
                    }

                    _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::Invalid(*zl, String::from(zhiling::COMMON_INVALID_DESC), String::from(zhiling::CAUSE_COMMON_INVALID)))),
                }
            }
            simctrl::DevType::FuZai => {
                match zl.zhi_ling_type {
                    ZhiLingType::BianZai(..) => self.handle_bian_zai(zl),
                    ZhiLingType::ZhongZaiJiaZai(..) => self.handle_zhong_zai_jia_zai(zl),

                    _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::Invalid(*zl, String::from(zhiling::COMMON_INVALID_DESC), String::from(zhiling::CAUSE_COMMON_INVALID)))),
                }
            }

            simctrl::DevType::Wu => {
                match zl.zhi_ling_type {
                    ZhiLingType::YueKong => self.handle_yue_kong(zl),

                    _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::Invalid(*zl, String::from(zhiling::COMMON_INVALID_DESC), String::from(zhiling::CAUSE_COMMON_INVALID)))),
                }
            }
            _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::Invalid(*zl, String::from(zhiling::COMMON_INVALID_DESC), String::from(zhiling::CAUSE_COMMON_INVALID)))),
        }
    }

    pub fn handle_zhi_ling_result_msg(&mut self, _result : Result<YingDaType, YingDaErr>) {

    }

    pub fn handle_bei_che(&mut self, zl : &ZhiLing) {
        match self.ji_zu_vec[zl.dev_id].common_ji.current_range {
            jizu::JiZuRangeLeiXing::TingJi => self.ji_zu_vec[zl.dev_id].common_ji.current_range = jizu::JiZuRangeLeiXing::BeiCheZanTai,
            _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::BeiCheFail(*zl, String::from(zhiling::BEI_CHE_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_1)))),
        }
    }
    pub fn handle_qi_dong(&mut self, zl : &ZhiLing) {
        if self.get_duanluqi_from_jizuid(zl.dev_id).unwrap().is_off() {
            match self.ji_zu_vec[zl.dev_id].common_ji.current_range {
                jizu::JiZuRangeLeiXing::BeiCheWanBi => {
                    self.ji_zu_vec[zl.dev_id].common_ji.current_range = jizu::JiZuRangeLeiXing::QiDong;
                }
                _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::QiDongFail(*zl,  String::from(zhiling::QI_DONG_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_2)))),
            }
        }
        else {
            self.handle_zhi_ling_result_msg( Err(YingDaErr::QiDongFail(*zl, String::from(zhiling::QI_DONG_FAIL_DESC),  String::from(zhiling::CAUSE_DUAN_LU_QI_BI_HE_HUO_GU_ZHANG))));
        }
    }
    pub fn handle_he_zha_bing_che(&mut self, zl : &ZhiLing) {
        if self.get_duanluqi_from_jizuid(zl.dev_id).unwrap().is_off() {
            let duanluqi_id = self.get_duanluqiid_from_jizuid(zl.dev_id).unwrap();
            let mut xt_temp = self.clone();
            xt_temp.duan_lu_qi_vec[duanluqi_id].set_on();
            xt_temp.update_node_group_vec();
            let ji_zu_vec_bing_lian_temp : Vec<usize> = xt_temp.get_jizuid_vec_bing_lian_from_id(zl.dev_id);
            xt_temp.compute_xi_tong_pf();
            self.compute_xi_tong_pf();
            let mut p_q_yi_qian_vec : Vec<(f64, f64)> = Vec::new();
            for j in ji_zu_vec_bing_lian_temp.to_vec() {
                p_q_yi_qian_vec.push((self.ji_zu_vec[j].common_ji.p, self.ji_zu_vec[j].common_ji.q));
            }
            //对每台并联的机组变到变载过程
            //如果jiZuListBingLian为空则为合闸
            if ji_zu_vec_bing_lian_temp.is_empty() {
                match self.ji_zu_vec[zl.dev_id].common_ji.current_range {
                    jizu::JiZuRangeLeiXing::Wen => {
                        let mut v = Vec::new();
                        v.push(zl.dev_id);
                        self.set_ji_zu_vec_bian_zai_params(v, xt_temp.ji_zu_vec[zl.dev_id].common_ji.p);
                        match self.duan_lu_qi_vec[duanluqi_id].status {
                           duanluqi::DuanLuQiStatus::Off{..} => self.duan_lu_qi_vec[duanluqi_id].set_on(),
                           duanluqi::DuanLuQiStatus::On{..} => {},
                       }
                       self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    }
                    _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling:: CAUSE_JI_ZU_RANGE_DISMATCH_6)))),
                }
            }
            //若不为空则为并车
            else {
                //判断所有并联机组是否处于稳态，如果处于则并车，如果不处于则不并车
                // is_bing_lian_ji_zu_bu_wen_ding = false;
                let mut ji_zu_vec_bing_lian = ji_zu_vec_bing_lian_temp.to_vec();
                ji_zu_vec_bing_lian.push(zl.dev_id);
                if ji_zu_vec_bing_lian.iter().all(|&j|self.ji_zu_vec[j].common_ji.current_range == jizu::JiZuRangeLeiXing::Wen) {
                    //如果为手动合闸，则判断是否经过手动并车
                    if self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::Manual {
                        if self.ji_zu_vec[zl.dev_id].common_ji.f_ext > self.ji_zu_vec[ji_zu_vec_bing_lian_temp[0]].common_ji.f_ext && self.ji_zu_vec[zl.dev_id].common_ji.f_ext - self.ji_zu_vec[ji_zu_vec_bing_lian_temp[0]].common_ji.f_ext <= jizu::JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI && self.ji_zu_vec[zl.dev_id].common_ji.uab_ext <= jizu::JI_ZU_DIAN_YA_WEN_TAI_ZUI_DA_YU_ZHI && self.ji_zu_vec[zl.dev_id].common_ji.uab_ext >= jizu::JI_ZU_DIAN_YA_WEN_TAI_ZUI_XIAO_YU_ZHI {
                           match self.duan_lu_qi_vec[duanluqi_id].status {
                               duanluqi::DuanLuQiStatus::Off{..} => self.duan_lu_qi_vec[duanluqi_id].set_on(),
                               duanluqi::DuanLuQiStatus::On{..} => {},
                           }
                           self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                        }
                    }
                    //否则自动合闸
                    else if self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::Auto || self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::SemiAuto{
                        match self.duan_lu_qi_vec[duanluqi_id].status {
                           duanluqi::DuanLuQiStatus::Off{fault : f, ready_to_bing_che : false} => self.duan_lu_qi_vec[duanluqi_id].status = duanluqi::DuanLuQiStatus::Off{fault : f, ready_to_bing_che : true},
                           _ => {},
                       }
                       //设定待并机组频率为电网频率+JI_ZU_BING_CHE_PIN_LV_DELTA
                       let f_delta = self.ji_zu_vec[ji_zu_vec_bing_lian_temp[0]].common_ji.f_ext + jizu::JI_ZU_BING_CHE_PIN_LV_DELTA - self.ji_zu_vec[zl.dev_id].common_ji.f_ext;
                       self.ji_zu_vec[zl.dev_id].set_bian_pin_params(f_delta, false);
                       self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    }
                }
                else {
                    //如果有机组不处于稳态，则报错
                    self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_6))));
                }
            }

        }
        else {
            self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling::CAUSE_DUAN_LU_QI_BI_HE_HUO_GU_ZHANG))));
        }
    }
    pub fn handle_ting_ji(&mut self, zl : &ZhiLing) {
        if self.get_duanluqi_from_jizuid(zl.dev_id).unwrap().is_off() {
            match self.ji_zu_vec[zl.dev_id].common_ji.current_range {
                jizu::JiZuRangeLeiXing::Wen | jizu::JiZuRangeLeiXing::BianSu | jizu::JiZuRangeLeiXing::BianYa => {
                    self.ji_zu_vec[zl.dev_id].common_ji.current_range = jizu::JiZuRangeLeiXing::TingJiZanTai;
                }
                _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::BeiCheFail(*zl,  String::from(zhiling::TING_JI_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_3)))),
            }
        }
        else {
            self.handle_zhi_ling_result_msg( Err(YingDaErr::TingJiFail(*zl, String::from(zhiling::TING_JI_FAIL_DESC),  String::from(zhiling::CAUSE_DUAN_LU_QI_BI_HE_HUO_GU_ZHANG))));
        }
    }

    pub fn handle_yue_kong(&mut self, zl : &ZhiLing) {
        self.yue_kong = true;
        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
    }

    pub fn handle_tou_ru(&mut self, _zl : &ZhiLing) {
    }

    pub fn handle_tui_chu(&mut self, _zl : &ZhiLing) {
    }

    pub fn handle_he_zha_bing_che_duan_lu_qi(&mut self, zl : &ZhiLing) {
        if self.duan_lu_qi_vec[zl.dev_id].is_off() {
            let duanluqi_id = zl.dev_id;
            let mut xt_temp = self.clone();
            xt_temp.duan_lu_qi_vec[duanluqi_id].set_on();
            xt_temp.update_node_group_vec();
            if xt_temp.is_hui_lu() {
                self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling:: CAUSE_XI_TONG_HUI_LU_EXIST))));
            }
            else {
                // let ji_zu_vec_bing_lian_temp : Vec<usize> = xt_temp.get_jizuid_vec_bing_lian_from_id(zl.dev_id);
                xt_temp.compute_xi_tong_pf();
                self.compute_xi_tong_pf();
                // let mut p_q_yi_qian_vec : Vec<(f64, f64)> = Vec::new();
                // for j in ji_zu_vec_bing_lian_temp.to_vec() {
                //     p_q_yi_qian_vec.push((self.ji_zu_vec[j].common_ji.p, self.ji_zu_vec[j].common_ji.q));
                // }

                let jizu_vec_vec_bing_che = self.compare_two_xi_tong_he_zha_ji_zu(&mut xt_temp);
                let jizu_vec_vec_bing_che_len = jizu_vec_vec_bing_che.len();
                if jizu_vec_vec_bing_che_len == 2 {
                    if jizu_vec_vec_bing_che[0].len()==0 || jizu_vec_vec_bing_che[1].len()==0 {
                        self.duan_lu_qi_vec[duanluqi_id].set_on();
                        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                        // self.update_node_group_vec();
                        self.compute_xi_tong_pf();
                    }
                    //如果两组中有一个元素个数为1，另一个>=1，则为并车
                    else if jizu_vec_vec_bing_che[0].len()==1 && jizu_vec_vec_bing_che[1].len()>=1 {
                        let jizuid = jizu_vec_vec_bing_che[0][0];
                        let mut ji_zu_vec_bing_lian = jizu_vec_vec_bing_che[1].clone();

                        // self.update_node_group_vec();
                        self.compute_xi_tong_pf();
                        // xt_temp.update_node_group_vec();
                        xt_temp.compute_xi_tong_pf();
                        ji_zu_vec_bing_lian.push(jizuid);
                        if ji_zu_vec_bing_lian.iter().all(|&j|self.ji_zu_vec[j].common_ji.current_range == jizu::JiZuRangeLeiXing::Wen) {
                            //如果为手动合闸，则判断是否经过手动并车
                            if self.ji_zu_vec[jizuid].common_ji.ctrl_mode == simctrl::CtrlMode::Manual {
                                if self.ji_zu_vec[jizuid].common_ji.f_ext > self.ji_zu_vec[jizuid].common_ji.f_ext && self.ji_zu_vec[jizuid].common_ji.f_ext - self.ji_zu_vec[jizuid].common_ji.f_ext <= jizu::JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI && self.ji_zu_vec[jizuid].common_ji.uab_ext <= jizu::JI_ZU_DIAN_YA_WEN_TAI_ZUI_DA_YU_ZHI && self.ji_zu_vec[jizuid].common_ji.uab_ext >= jizu::JI_ZU_DIAN_YA_WEN_TAI_ZUI_XIAO_YU_ZHI {
                                    match self.duan_lu_qi_vec[duanluqi_id].status {
                                       duanluqi::DuanLuQiStatus::Off{..} => self.duan_lu_qi_vec[duanluqi_id].set_on(),
                                       duanluqi::DuanLuQiStatus::On{..} => {},
                                   }
                                   self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                                }
                            }
                            //否则自动合闸
                            else if self.ji_zu_vec[jizuid].common_ji.ctrl_mode == simctrl::CtrlMode::Auto || self.ji_zu_vec[jizuid].common_ji.ctrl_mode == simctrl::CtrlMode::SemiAuto{
                                match self.duan_lu_qi_vec[duanluqi_id].status {
                                   duanluqi::DuanLuQiStatus::Off{fault : f, ready_to_bing_che : false} => self.duan_lu_qi_vec[duanluqi_id].status = duanluqi::DuanLuQiStatus::Off{fault : f, ready_to_bing_che : true},
                                   _ => {},
                               }
                               //设定待并机组频率为电网频率+JI_ZU_BING_CHE_PIN_LV_DELTA
                               let f_delta = self.ji_zu_vec[jizuid].common_ji.f_ext + jizu::JI_ZU_BING_CHE_PIN_LV_DELTA - self.ji_zu_vec[jizuid].common_ji.f_ext;
                               self.ji_zu_vec[jizuid].set_bian_pin_params(f_delta, false);
                               self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                            }
                        }
                        else {
                            //如果有机组不处于稳态，则报错
                            self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_6))));
                        }

                    }
                    else if jizu_vec_vec_bing_che[0].len()>=1 &&
                            jizu_vec_vec_bing_che[1].len()==1 {
                        let jizuid = jizu_vec_vec_bing_che[1][0];
                        let mut ji_zu_vec_bing_lian = jizu_vec_vec_bing_che[0].clone();

                        // self.update_node_group_vec();
                        self.compute_xi_tong_pf();
                        // xt_temp.update_node_group_vec();
                        xt_temp.compute_xi_tong_pf();
                        ji_zu_vec_bing_lian.push(jizuid);
                        if ji_zu_vec_bing_lian.iter().all(|&j|self.ji_zu_vec[j].common_ji.current_range == jizu::JiZuRangeLeiXing::Wen) {
                            //如果为手动合闸，则判断是否经过手动并车
                            if self.ji_zu_vec[jizuid].common_ji.ctrl_mode == simctrl::CtrlMode::Manual {
                                if self.ji_zu_vec[jizuid].common_ji.f_ext > self.ji_zu_vec[jizuid].common_ji.f_ext && self.ji_zu_vec[jizuid].common_ji.f_ext - self.ji_zu_vec[jizuid].common_ji.f_ext <= jizu::JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI && self.ji_zu_vec[jizuid].common_ji.uab_ext <= jizu::JI_ZU_DIAN_YA_WEN_TAI_ZUI_DA_YU_ZHI && self.ji_zu_vec[jizuid].common_ji.uab_ext >= jizu::JI_ZU_DIAN_YA_WEN_TAI_ZUI_XIAO_YU_ZHI {
                                    match self.duan_lu_qi_vec[duanluqi_id].status {
                                       duanluqi::DuanLuQiStatus::Off{..} => self.duan_lu_qi_vec[duanluqi_id].set_on(),
                                       duanluqi::DuanLuQiStatus::On{..} => {},
                                   }
                                   self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                                }
                            }
                            //否则自动合闸
                            else if self.ji_zu_vec[jizuid].common_ji.ctrl_mode == simctrl::CtrlMode::Auto || self.ji_zu_vec[jizuid].common_ji.ctrl_mode == simctrl::CtrlMode::SemiAuto{
                                match self.duan_lu_qi_vec[duanluqi_id].status {
                                   duanluqi::DuanLuQiStatus::Off{fault : f, ready_to_bing_che : false} => self.duan_lu_qi_vec[duanluqi_id].status = duanluqi::DuanLuQiStatus::Off{fault : f, ready_to_bing_che : true},
                                   _ => {},
                               }
                               //设定待并机组频率为电网频率+JI_ZU_BING_CHE_PIN_LV_DELTA
                               let f_delta = self.ji_zu_vec[jizuid].common_ji.f_ext + jizu::JI_ZU_BING_CHE_PIN_LV_DELTA - self.ji_zu_vec[jizuid].common_ji.f_ext;
                               self.ji_zu_vec[jizuid].set_bian_pin_params(f_delta, false);
                               self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                            }
                        }
                        else {
                            //如果有机组不处于稳态，则报错
                            self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_6))));
                        }

                    }
                    //如果两组中两个元素个数都大于1，则并车失败，提示消息返回“试图一次并联多个机组”
                    else if jizu_vec_vec_bing_che[0].len()>1 &&
                            jizu_vec_vec_bing_che[1].len()>1 {
                        self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling::CAUSE_DUO_JI_ZU_TONG_SHI_BING_LIAN))));
                    }

                }
                else {
                    self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling:: CAUSE_POWER_FLOW_ERR))));
                }
            }
            // let ji_zu_vec_bing_lian_temp : Vec<usize> = xt_temp.get_jizuid_vec_bing_lian_from_id(zl.dev_id);
            // xt_temp.update_node_group_vec();
            // xt_temp.compute_xi_tong_pf();
            // self.update_node_group_vec();
            // self.compute_xi_tong_pf();
            // let mut p_q_yi_qian_vec : Vec<(f64, f64)> = Vec::new();
            // for j in ji_zu_vec_bing_lian_temp.to_vec() {
            //     p_q_yi_qian_vec.push((self.ji_zu_vec[j].common_ji.p, self.ji_zu_vec[j].common_ji.q));
            // }
            // //对每台并联的机组变到变载过程
            // let mut is_bing_lian_ji_zu_bu_wen_ding = false;
            // //如果jiZuListBingLian为空则为合闸
            // if ji_zu_vec_bing_lian_temp.is_empty() {
            //     match self.ji_zu_vec[zl.dev_id].common_ji.current_range {
            //         jizu::JiZuRangeLeiXing::Wen => {
            //             let mut v = Vec::new();
            //             v.push(zl.dev_id);
            //             self.set_ji_zu_vec_bian_zai_params(v, xt_temp.ji_zu_vec[zl.dev_id].common_ji.p);
            //         }
            //         _ => is_bing_lian_ji_zu_bu_wen_ding = true,
            //     }
            //     match is_bing_lian_ji_zu_bu_wen_ding {
            //         true => {
            //             match self.duan_lu_qi_vec[duanluqi_id].status {
            //                duanluqi::DuanLuQiStatus::Off{fault : f, ..} => self.duan_lu_qi_vec[duanluqi_id].status = duanluqi::DuanLuQiStatus::On{fault : f, ready_to_jie_lie : false},
            //                duanluqi::DuanLuQiStatus::On{..} => {},
            //            }
            //            self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
            //         }
            //         false => {
            //             self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling:: CAUSE_JI_ZU_RANGE_DISMATCH_6))));
            //         }
            //     }
            // }
            // //若不为空则为并车
            // else {
            //     //判断所有并联机组是否处于稳态，如果处于则并车，如果不处于则不并车
            //     // is_bing_lian_ji_zu_bu_wen_ding = false;
            //     let mut ji_zu_vec_bing_lian = ji_zu_vec_bing_lian_temp.to_vec();
            //     ji_zu_vec_bing_lian.push(zl.dev_id);
            //     if ji_zu_vec_bing_lian.iter().all(|&j|self.ji_zu_vec[j].common_ji.current_range == jizu::JiZuRangeLeiXing::Wen) {
            //         //如果为手动合闸，则判断是否经过手动并车
            //         if self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::Manual {
            //             if self.ji_zu_vec[zl.dev_id].common_ji.f_ext > self.ji_zu_vec[ji_zu_vec_bing_lian_temp[0]].common_ji.f_ext && self.ji_zu_vec[zl.dev_id].common_ji.f_ext - self.ji_zu_vec[ji_zu_vec_bing_lian_temp[0]].common_ji.f_ext <= jizu::JI_ZU_PIN_LV_BIAN_HUA_YU_ZHI_WEN_TAI && self.ji_zu_vec[zl.dev_id].common_ji.uab_ext <= jizu::JI_ZU_DIAN_YA_WEN_TAI_ZUI_DA_YU_ZHI && self.ji_zu_vec[zl.dev_id].common_ji.uab_ext >= jizu::JI_ZU_DIAN_YA_WEN_TAI_ZUI_XIAO_YU_ZHI {
            //                 match self.duan_lu_qi_vec[duanluqi_id].status {
            //                    duanluqi::DuanLuQiStatus::Off{fault : f, ..} => self.duan_lu_qi_vec[duanluqi_id].status = duanluqi::DuanLuQiStatus::On{fault : f, ready_to_jie_lie : false},
            //                    duanluqi::DuanLuQiStatus::On{..} => {},
            //                }
            //                self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
            //             }
            //         }
            //         //否则自动合闸
            //         else if self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::Auto || self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::SemiAuto{
            //             match self.duan_lu_qi_vec[duanluqi_id].status {
            //                duanluqi::DuanLuQiStatus::Off{fault : f, ready_to_bing_che : false} => self.duan_lu_qi_vec[duanluqi_id].status = duanluqi::DuanLuQiStatus::Off{fault : f, ready_to_bing_che : true},
            //                _ => {},
            //            }
            //            //设定待并机组频率为电网频率+JI_ZU_BING_CHE_PIN_LV_DELTA
            //            let f_delta = self.ji_zu_vec[ji_zu_vec_bing_lian_temp[0]].common_ji.f_ext + jizu::JI_ZU_BING_CHE_PIN_LV_DELTA - self.ji_zu_vec[zl.dev_id].common_ji.f_ext;
            //            self.ji_zu_vec[zl.dev_id].set_bian_pin_params(f_delta, false);
            //            self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
            //         }
            //     }
            //     else {
            //         //如果有机组不处于稳态，则报错
            //         self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_6))));
            //     }
            // }

        }
        else {
            self.handle_zhi_ling_result_msg( Err(YingDaErr::HeZhaBingCheFail(*zl, String::from(zhiling::HE_ZHA_BING_CHE_FAIL_DESC), String::from(zhiling::CAUSE_DUAN_LU_QI_BI_HE_HUO_GU_ZHANG))));
        }

    }

    pub fn handle_fen_zha_jie_lie(&mut self, zl : &ZhiLing) {
        let jizuid = zl.dev_id;
        let duanluqiid = self.get_duanluqiid_from_jizuid(jizuid).unwrap();
        if self.duan_lu_qi_vec[duanluqiid].is_on(){
            let ji_zu_vec_bing_lian = self.get_jizuid_vec_bing_lian_from_id(jizuid);

            if ji_zu_vec_bing_lian.is_empty() {
                //如果功率小于0.1额定功率，则直接分闸
                //如果功率大于0.1额定功率，半自动和自动则减载至10%，然后分闸
                //手动则应答合闸失败
                if self.ji_zu_vec[jizuid].common_ji.current_range == JiZuRangeLeiXing::Wen {
                    if self.ji_zu_vec[jizuid].common_ji.p <= jizu::JI_ZU_JIE_LIE_GONG_LV_YU_ZHI {
                        self.duan_lu_qi_vec[duanluqiid].set_off();
                        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                        let p_delta = 0.0 - self.ji_zu_vec[jizuid].common_ji.p;
                        self.ji_zu_vec[jizuid].set_bian_zai_params(p_delta);
                    }
                    else if self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::Manual {
                        self.handle_zhi_ling_result_msg( Err(YingDaErr::FenZhaJieLieFail(*zl, String::from(zhiling::FEN_ZHA_JIE_LIE_FAIL_DESC), String::from(zhiling::CAUSE_FEN_ZHA_JIE_LIE_WEI_SHOU_DONG_JIAN_ZAI))));
                    }
                    //否则自动解列
                    else if self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::Auto || self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::SemiAuto{
                        let is_fault = self.duan_lu_qi_vec[duanluqiid].is_fault();
                        self.duan_lu_qi_vec[duanluqiid].status = duanluqi::DuanLuQiStatus::On{fault: is_fault, ready_to_jie_lie : true};
                        //设定待并机组频率为电网频率+JI_ZU_BING_CHE_PIN_LV_DELTA
                        let p_delta = jizu::JI_ZU_JIE_LIE_GONG_LV_YU_ZHI - self.ji_zu_vec[jizuid].common_ji.p;
                        self.ji_zu_vec[zl.dev_id].set_bian_zai_params(p_delta);
                        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    }

                }
                else{
                    self.handle_zhi_ling_result_msg( Err(YingDaErr::FenZhaJieLieFail(*zl, String::from(zhiling::FEN_ZHA_JIE_LIE_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_6))));
                }
            }
            else{//若不为空则为解列
                //判断所有并联机组是否处于稳态，如果处于则解列，如果不处于则不解列
                if ji_zu_vec_bing_lian.to_vec().iter().all(|&j|self.ji_zu_vec[j].common_ji.current_range == jizu::JiZuRangeLeiXing::Wen){
                    //首先判断解列后功率是否够用，够用则解列，不够用则解列失败
                    let p_ji_zu_bing_lian : f64 = ji_zu_vec_bing_lian.len() as f64 * jizu::JI_ZU_E_DING_GONG_LV;
                    let p_fu_zai_bing_lian = self.get_pd_from_jizuid(jizuid) + self.get_pd_from_jizuid_vec(ji_zu_vec_bing_lian.to_vec());
                    if p_ji_zu_bing_lian>=p_fu_zai_bing_lian {//功率够用
                        let mut xt_temp = self.clone();
                        xt_temp.duan_lu_qi_vec[duanluqiid].set_off();
                        self.update_node_group_vec();
                        self.compute_xi_tong_pf();
                        xt_temp.update_node_group_vec();
                        xt_temp.compute_xi_tong_pf();
                        //如果功率小于0.1额定功率，则直接解列,其余并联机组变载
                        //如果功率大于0.1额定功率，半自动和自动则变载至10%，然后分闸,其余并联机组变载
                        //手动则应答合闸失败
                        if self.ji_zu_vec[jizuid].common_ji.p <= jizu::JI_ZU_JIE_LIE_GONG_LV_YU_ZHI {
                            self.duan_lu_qi_vec[duanluqiid].set_off();
                            let p_delta = 0.0 - self.ji_zu_vec[jizuid].common_ji.p;
                            self.ji_zu_vec[zl.dev_id].set_bian_zai_params(p_delta);
                            self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                            //其余并联机组根据潮流计算结果进行变载
                            for i in ji_zu_vec_bing_lian.to_vec() {
                                let p_delta = xt_temp.ji_zu_vec[i].common_ji.p - self.ji_zu_vec[i].common_ji.p;
                                self.ji_zu_vec[i].set_bian_zai_params(p_delta);
                            }
                        }
                        else {
                            //需要减载解列
                            if self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::Manual {
                                self.handle_zhi_ling_result_msg( Err(YingDaErr::FenZhaJieLieFail(*zl, String::from(zhiling::FEN_ZHA_JIE_LIE_FAIL_DESC), String::from(zhiling::CAUSE_FEN_ZHA_JIE_LIE_WEI_SHOU_DONG_JIAN_ZAI))));
                            }
                            //否则自动解列
                            else if self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::Auto || self.ji_zu_vec[zl.dev_id].common_ji.ctrl_mode == simctrl::CtrlMode::SemiAuto{
                                let is_fault = self.duan_lu_qi_vec[duanluqiid].is_fault();
                                self.duan_lu_qi_vec[duanluqiid].status = duanluqi::DuanLuQiStatus::On{fault: is_fault, ready_to_jie_lie : true};
                                //设定待并机组频率为电网频率+JI_ZU_BING_CHE_PIN_LV_DELTA
                                let p_delta = jizu::JI_ZU_JIE_LIE_GONG_LV_YU_ZHI - self.ji_zu_vec[jizuid].common_ji.p;
                                self.ji_zu_vec[zl.dev_id].set_bian_zai_params(p_delta);
                                self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                                //其余并联机组根据潮流计算结果进行变载
                                for i in ji_zu_vec_bing_lian.to_vec() {
                                    let p_delta = xt_temp.ji_zu_vec[i].common_ji.p - self.ji_zu_vec[i].common_ji.p;
                                    self.ji_zu_vec[i].set_bian_zai_params(p_delta);
                                }
                            }
                        }
                    }
                    else{//功率不够用
                        self.handle_zhi_ling_result_msg( Err(YingDaErr::FenZhaJieLieFail(*zl, String::from(zhiling::FEN_ZHA_JIE_LIE_FAIL_DESC), String::from(zhiling::CAUSE_FEN_ZHA_JIE_LIE_ZONG_GONG_LV_BU_GOU_YONG))));
                    }
                }
                else{
                    self.handle_zhi_ling_result_msg( Err(YingDaErr::FenZhaJieLieFail(*zl, String::from(zhiling::FEN_ZHA_JIE_LIE_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_6))));
                }
            }
        }
        else{
            self.handle_zhi_ling_result_msg( Err(YingDaErr::FenZhaJieLieFail(*zl, String::from(zhiling::FEN_ZHA_JIE_LIE_FAIL_DESC), String::from(zhiling::CAUSE_DUAN_LU_QI_FEN_DUAN_HUO_GU_ZHANG))));
        }
    }

    pub fn handle_fen_zha_jie_lie_duan_lu_qi(&mut self, zl : &ZhiLing) {
        let duanluqiid = zl.dev_id;
        if self.duan_lu_qi_vec[duanluqiid].is_on(){
            let mut xt_temp = self.clone();
            xt_temp.duan_lu_qi_vec[duanluqiid].set_off();
            self.update_node_group_vec();
            self.compute_xi_tong_pf();
            xt_temp.update_node_group_vec();
            xt_temp.compute_xi_tong_pf();
            let fen_zha_ji_zu_group = xt_temp.compare_two_xi_tong_he_zha_ji_zu(self);
            let fen_zha_fu_zai_group = xt_temp.compare_two_xi_tong_he_zha_fu_zai(self);
            if fen_zha_ji_zu_group.len() == 2 {
                //如果两组中有一个元素个数为零，则拓扑无变化，直接分闸
                if fen_zha_ji_zu_group[0].len() == 0 || fen_zha_ji_zu_group[1].len() == 0 {
                    self.duan_lu_qi_vec[duanluqiid].set_off();
                    self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    self.update_node_group_vec();
                    self.compute_xi_tong_pf();
                }
                else {
                    //否则为解列
                    //判断分闸后的功率关系
                    let mut fen_zha_zong_gong_lv_ji_zu_group_0 = 0.0;
                    // let mut fen_zha_zong_gong_lv_ji_zu_group_1 = 0.0;
                    let mut fen_zha_zong_gong_lv_fu_zai_group_0 = 0.0;
                    let mut fen_zha_zong_gong_lv_fu_zai_group_1 = 0.0;
                    let fen_zha_zong_gong_lv_max_fu_zai_group_0 = fen_zha_ji_zu_group[0].len() as f64 * jizu::JI_ZU_E_DING_GONG_LV;
                    let fen_zha_zong_gong_lv_max_fu_zai_group_1 =  fen_zha_ji_zu_group[1].len() as f64 * jizu::JI_ZU_E_DING_GONG_LV;
                    for i in fen_zha_ji_zu_group[0].to_vec() {
                        fen_zha_zong_gong_lv_ji_zu_group_0 += self.ji_zu_vec[i].common_ji.p;
                    }
                    // for i in fen_zha_ji_zu_group[1].to_vec() {
                    //     fen_zha_zong_gong_lv_ji_zu_group_1 += self.ji_zu_vec[i].common_ji.p;
                    // }
                    for i in fen_zha_fu_zai_group[0].to_vec() {
                        fen_zha_zong_gong_lv_fu_zai_group_0 += self.fu_zai_vec[i].p;
                    }
                    for i in fen_zha_fu_zai_group[1].to_vec() {
                        fen_zha_zong_gong_lv_fu_zai_group_1 += self.fu_zai_vec[i].p;
                    }
                    //如果功率够用
                    if fen_zha_zong_gong_lv_max_fu_zai_group_0 >= fen_zha_zong_gong_lv_fu_zai_group_0 || fen_zha_zong_gong_lv_max_fu_zai_group_1 >= fen_zha_zong_gong_lv_fu_zai_group_1 {
                        //如果某个组的机组总输出功率与某个组的负载总输出功率之差
                        //小于等于0.1倍机组额定功率,则直接分闸
                        if (fen_zha_zong_gong_lv_ji_zu_group_0 - fen_zha_zong_gong_lv_fu_zai_group_0).abs() <= jizu::JI_ZU_JIE_LIE_GONG_LV_YU_ZHI {
                            self.duan_lu_qi_vec[duanluqiid].set_off();
                            self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                            self.update_node_group_vec();
                            self.compute_xi_tong_pf();
                        }
                        else {
                            let is_fault = xt_temp.duan_lu_qi_vec[duanluqiid].is_fault();
                            self.duan_lu_qi_vec[duanluqiid].status = duanluqi::DuanLuQiStatus::On{fault: is_fault, ready_to_jie_lie : true};
                            //各相关机组变载至与负载功率相差10%,暂时实现为相等
                            for i in fen_zha_ji_zu_group.to_vec() {
                                for j in i {
                                    let p_delta = xt_temp.ji_zu_vec[j].common_ji.p - self.ji_zu_vec[j].common_ji.p;
                                    self.ji_zu_vec[j].set_bian_zai_params(p_delta);
                                }
                            }
                            self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                        }
                    }
                    else {
                        self.handle_zhi_ling_result_msg( Err(YingDaErr::FenZhaJieLieFail(*zl, String::from(zhiling::FEN_ZHA_JIE_LIE_FAIL_DESC), String::from(zhiling::CAUSE_FEN_ZHA_JIE_LIE_ZONG_GONG_LV_BU_GOU_YONG))));
                    }
                }
            }
        }
        else{
            self.handle_zhi_ling_result_msg( Err(YingDaErr::FenZhaJieLieFail(*zl, String::from(zhiling::FEN_ZHA_JIE_LIE_FAIL_DESC), String::from(zhiling::CAUSE_DUAN_LU_QI_FEN_DUAN_HUO_GU_ZHANG))));
        }
    }

    pub fn handle_ji_zu_yi_ban_gu_zhang(&mut self, zl : &ZhiLing) {
        match zl.zhi_ling_type {
            zhiling::ZhiLingType::GenerateYiBanGuZhang(f) => {
                match f {
                    zhiling::FaultType::RanYouXieLou => {
                        if self.is_chai_you_by_id(zl.dev_id) {
                            self.ji_zu_vec[zl.dev_id].common_ji.ran_you_xie_lou = true;
                            self.is_xiao_sheng = false;
                            self.is_ying_da = false;
                            self.ji_zu_vec[zl.dev_id].common_ji.gu_zhang_lei_xing = jizu::GuZhangLeiXing::YiBan;
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn handle_ji_zu_yi_ji_gu_zhang(&mut self, _zl : &ZhiLing) {
    }

    pub fn handle_ji_zu_er_ji_gu_zhang(&mut self, _zl : &ZhiLing) {
    }

    pub fn handle_ji_zu_qi_ta_gu_zhang(&mut self, _zl : &ZhiLing) {
    }

    pub fn eliminate_ji_zu_yi_ban_gu_zhang(&mut self, _zl : &ZhiLing) {
    }

    pub fn eliminate_ji_zu_yi_ji_gu_zhang(&mut self, _zl : &ZhiLing) {
    }

    pub fn eliminate_ji_zu_er_ji_gu_zhang(&mut self, _zl : &ZhiLing) {
    }

    pub fn eliminate_ji_zu_qi_ta_gu_zhang(&mut self, _zl : &ZhiLing) {
    }

    pub fn handle_bian_zai(&mut self, zl : &ZhiLing) {
        let mut xt_temp = self.clone();
        match zl.zhi_ling_type {
            zhiling::ZhiLingType::BianZai(p, q) => {
                xt_temp.fu_zai_vec[zl.dev_id].p += p;
                if xt_temp.fu_zai_vec[zl.dev_id].p > xt_temp.fu_zai_vec[zl.dev_id].p_max {
                    xt_temp.fu_zai_vec[zl.dev_id].p = xt_temp.fu_zai_vec[zl.dev_id].p_max;
                }
                else if xt_temp.fu_zai_vec[zl.dev_id].p < 0.0 {
                    xt_temp.fu_zai_vec[zl.dev_id].p = 0.0;
                }
                xt_temp.fu_zai_vec[zl.dev_id].q += q;
                if xt_temp.fu_zai_vec[zl.dev_id].q > xt_temp.fu_zai_vec[zl.dev_id].q_max {
                    xt_temp.fu_zai_vec[zl.dev_id].q = xt_temp.fu_zai_vec[zl.dev_id].q_max;
                }
                else if xt_temp.fu_zai_vec[zl.dev_id].q < 0.0 {
                    xt_temp.fu_zai_vec[zl.dev_id].q = 0.0;
                }
                let node_id = xt_temp.get_nodeid_from_fuzaiid(zl.dev_id).unwrap();
                if xt_temp.fu_zai_vec[zl.dev_id].p == 0.0 && xt_temp.fu_zai_vec[zl.dev_id].q == 0.0 || xt_temp.node_vec[node_id].u == 0.0 {
                    xt_temp.fu_zai_vec[zl.dev_id].is_online = false;
                    xt_temp.fu_zai_vec[zl.dev_id].p = 0.0;
                    xt_temp.fu_zai_vec[zl.dev_id].q = 0.0;
                }
                else {
                    xt_temp.fu_zai_vec[zl.dev_id].is_online = true;
                }
            }
            _ => {}
        }
        xt_temp.compute_xi_tong_pf();
        self.fu_zai_vec[zl.dev_id].p = xt_temp.fu_zai_vec[zl.dev_id].p;
        self.fu_zai_vec[zl.dev_id].q = xt_temp.fu_zai_vec[zl.dev_id].q;
        self.fu_zai_vec[zl.dev_id].is_online = xt_temp.fu_zai_vec[zl.dev_id].is_online;
        self.fu_zai_vec[zl.dev_id].i = xt_temp.fu_zai_vec[zl.dev_id].i;
        let ji_zu_group = self.get_ji_zu_group_from_fu_zai_id(zl.dev_id);
        for j in ji_zu_group {
            if self.ji_zu_vec[j].common_ji.current_range == jizu::JiZuRangeLeiXing::BianSu {
                self.ji_zu_vec[j].common_ji.current_range = jizu::JiZuRangeLeiXing::Wen;
            }
            self.ji_zu_vec[j].set_bian_zai_params(xt_temp.ji_zu_vec[j].common_ji.p);
        }
        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
    }

    pub fn handle_zhong_zai_jia_zai(&mut self, zl : &ZhiLing) {
        let ji_zu_group = self.get_ji_zu_group_from_fu_zai_id(zl.dev_id);
        let group_len = ji_zu_group.len();
        let mut xt_temp = self.clone();
        xt_temp.compute_xi_tong_pf();
        match zl.zhi_ling_type {
            zhiling::ZhiLingType::ZhongZaiJiaZai(p, q) => {
                let mut p_sum = 0.0f64;
                let mut q_sum = 0.0f64;
                for j in ji_zu_group {
                    p_sum += self.ji_zu_vec[j].common_ji.p;
                    q_sum += self.ji_zu_vec[j].common_ji.q;
                }
                if jizu::JI_ZU_E_DING_GONG_LV * group_len as f64 >= p_sum + p && jizu::JI_ZU_E_DING_WU_GONG_GONG_LV * group_len as f64 >= q_sum + q {
                    let mut zl_temp = zl.clone();
                    zl_temp.zhi_ling_type = zhiling::ZhiLingType::BianZai(p, q);
                    zl_temp.actor_id = usize::max_value();
                    zl_temp.zhan_wei_id = usize::max_value();
                    zl_temp.zhan_wei_type = simctrl::ZhanWeiType::Internal;
                    self.handle_bian_zai(&zl_temp);
                    self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                }
                else {
                    self.handle_zhi_ling_result_msg( Err(YingDaErr::ZhongZaiAskFail(*zl, String::from(zhiling::ZHONG_ZAI_ASK_FAIL_DESC), String::from(zhiling::CAUSE_ZHONG_ZAI_ASK_FAIL))));
                }
            }
            _ => {}
        }
    }

    pub fn handle_operating_station(&mut self, zl : &ZhiLing) {
        match zl.dev_type {
            simctrl::DevType::JiZu => {
                match zl.zhi_ling_type {
                    ZhiLingType::OperatingStation(simctrl::OperatingStation::JiPang) => {
                        self.ji_zu_vec[zl.dev_id].common_ji.operating_station_she_zhi = simctrl::OperatingStation::JiPang;
                        self.ji_zu_vec[zl.dev_id].common_ji.operating_station = simctrl::OperatingStation::JiPang;
                        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    }
                    ZhiLingType::OperatingStation(simctrl::OperatingStation::Remote) => {
                        self.ji_zu_vec[zl.dev_id].common_ji.operating_station_she_zhi = simctrl::OperatingStation::Remote;
                        self.ji_zu_vec[zl.dev_id].common_ji.operating_station = simctrl::OperatingStation::Remote;
                        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    }
                    _ =>  self.handle_zhi_ling_result_msg( Err(YingDaErr::OperatingStationFail(*zl, String::from(zhiling::OPERATING_STATION_FAIL_DESC), String::from(zhiling::CAUSE_OPERATING_STATION_INVALID)))),
                }
            }
            simctrl::DevType::DianZhan => {
                match zl.zhi_ling_type {
                    ZhiLingType::OperatingStation(simctrl::OperatingStation::Local) => {
                        self.dian_zhan_vec[zl.dev_id].operating_station_she_zhi = simctrl::OperatingStation::Local;
                        self.dian_zhan_vec[zl.dev_id].operating_station = simctrl::OperatingStation::Local;
                        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    }
                    ZhiLingType::OperatingStation(simctrl::OperatingStation::JiKong) => {
                        self.dian_zhan_vec[zl.dev_id].operating_station_she_zhi = simctrl::OperatingStation::JiKong;
                        self.dian_zhan_vec[zl.dev_id].operating_station = simctrl::OperatingStation::JiKong;
                        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    }
                    _ =>  self.handle_zhi_ling_result_msg( Err(YingDaErr::OperatingStationFail(*zl, String::from(zhiling::OPERATING_STATION_FAIL_DESC), String::from(zhiling::CAUSE_OPERATING_STATION_INVALID)))),
                }
            }
            _ => {}
        }
    }
    pub fn handle_ctrl_mode(&mut self, zl : &ZhiLing) {
        match zl.dev_type {
            simctrl::DevType::DianZhan => {
                match zl.zhi_ling_type {
                    ZhiLingType::CtrlMode(simctrl::CtrlMode::Manual) => {
                        self.dian_zhan_vec[zl.dev_id].ctrl_mode = simctrl::CtrlMode::Manual;
                        self.dian_zhan_vec[zl.dev_id].ctrl_mode_she_zhi = simctrl::CtrlMode::Manual;
                        if self.dian_zhan_vec[zl.dev_id].operating_station == simctrl::OperatingStation::JiKong {
                            self.dian_zhan_vec[zl.dev_id].operating_station = simctrl::OperatingStation::Local;
                        }
                        let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(zl.dev_id);
                        for j in jizuid_vec {
                            self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::Manual;
                        }
                        let dian_zhan_r_vec = self.get_dian_zhan_guan_lian_vec_from_dian_zhan_id(zl.dev_id);
                        for d in dian_zhan_r_vec {
                            self.dian_zhan_vec[d].ctrl_mode = simctrl::CtrlMode::Manual;
                            if self.dian_zhan_vec[d].operating_station == simctrl::OperatingStation::JiKong {
                                self.dian_zhan_vec[d].operating_station = simctrl::OperatingStation::Local;
                            }
                            let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(d);
                            for j in jizuid_vec {
                                self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::Manual;
                            }
                        }
                        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    }
                    ZhiLingType::CtrlMode(simctrl::CtrlMode::SemiAuto) => {
                        self.dian_zhan_vec[zl.dev_id].ctrl_mode_she_zhi = simctrl::CtrlMode::SemiAuto;
                        let dian_zhan_r_vec = self.get_dian_zhan_guan_lian_vec_from_dian_zhan_id(zl.dev_id);
                        let mut is_manual_exist = false;
                        //let mut is_auto_exist = false;
                        for d in dian_zhan_r_vec.to_vec() {
                            if self.dian_zhan_vec[d].ctrl_mode_she_zhi == simctrl::CtrlMode::Manual {
                                is_manual_exist = true;
                            }
                            // else if self.dian_zhan_vec[d].ctrl_mode_she_zhi == simctrl::CtrlMode::Auto {
                            //     is_auto_exist = true;
                            // }
                        }
                        if is_manual_exist /*|| !self.is_mu_lian_lian_tong(zl.dev_id) */ {
                            self.dian_zhan_vec[zl.dev_id].ctrl_mode = simctrl::CtrlMode::Manual;
                            if self.dian_zhan_vec[zl.dev_id].operating_station == simctrl::OperatingStation::JiKong {
                                self.dian_zhan_vec[zl.dev_id].operating_station = simctrl::OperatingStation::Local;
                            }
                            let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(zl.dev_id);
                            for j in jizuid_vec {
                                self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::Manual;
                            }
                            for d in dian_zhan_r_vec {
                                self.dian_zhan_vec[d].ctrl_mode = simctrl::CtrlMode::Manual;
                                if self.dian_zhan_vec[d].operating_station == simctrl::OperatingStation::JiKong {
                                    self.dian_zhan_vec[d].operating_station = simctrl::OperatingStation::Local;
                                }
                                let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(d);
                                for j in jizuid_vec {
                                    self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::Manual;
                                }
                            }
                        }
                        else {
                            self.dian_zhan_vec[zl.dev_id].ctrl_mode = simctrl::CtrlMode::SemiAuto;
                            let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(zl.dev_id);
                            for j in jizuid_vec {
                                self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::SemiAuto;
                            }
                            for d in dian_zhan_r_vec {
                                self.dian_zhan_vec[d].ctrl_mode = simctrl::CtrlMode::Manual;
                                if self.dian_zhan_vec[d].operating_station == simctrl::OperatingStation::JiKong {
                                    self.dian_zhan_vec[d].operating_station = simctrl::OperatingStation::Local;
                                }
                                let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(d);
                                for j in jizuid_vec {
                                    self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::Manual;
                                }
                            }
                        }
                        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    }
                    ZhiLingType::CtrlMode(simctrl::CtrlMode::Auto) => {
                        self.dian_zhan_vec[zl.dev_id].ctrl_mode_she_zhi = simctrl::CtrlMode::Auto;
                        let dian_zhan_r_vec = self.get_dian_zhan_guan_lian_vec_from_dian_zhan_id(zl.dev_id);
                        let mut is_manual_exist = false;
                        let mut is_semiauto_exist = false;
                        for d in dian_zhan_r_vec.to_vec() {
                            if self.dian_zhan_vec[d].ctrl_mode_she_zhi == simctrl::CtrlMode::Manual {
                                is_manual_exist = true;
                            }
                            else if self.dian_zhan_vec[d].ctrl_mode_she_zhi == simctrl::CtrlMode::Auto {
                                is_semiauto_exist = true;
                            }
                        }
                        if is_manual_exist /* || !self.is_mu_lian_lian_tong(zl.dev_id)*/ {
                            self.dian_zhan_vec[zl.dev_id].ctrl_mode = simctrl::CtrlMode::Manual;
                            if self.dian_zhan_vec[zl.dev_id].operating_station == simctrl::OperatingStation::JiKong {
                                self.dian_zhan_vec[zl.dev_id].operating_station = simctrl::OperatingStation::Local;
                            }
                            let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(zl.dev_id);
                            for j in jizuid_vec {
                                self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::Manual;
                            }
                            for d in dian_zhan_r_vec {
                                self.dian_zhan_vec[d].ctrl_mode = simctrl::CtrlMode::Manual;
                                if self.dian_zhan_vec[d].operating_station == simctrl::OperatingStation::JiKong {
                                    self.dian_zhan_vec[d].operating_station = simctrl::OperatingStation::Local;
                                }
                                let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(d);
                                for j in jizuid_vec {
                                    self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::Manual;
                                }
                            }
                        }
                        else if is_semiauto_exist {
                            self.dian_zhan_vec[zl.dev_id].ctrl_mode = simctrl::CtrlMode::SemiAuto;
                            let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(zl.dev_id);
                            for j in jizuid_vec {
                                self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::SemiAuto;
                            }
                            for d in dian_zhan_r_vec {
                                self.dian_zhan_vec[d].ctrl_mode = simctrl::CtrlMode::SemiAuto;
                                let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(d);
                                for j in jizuid_vec {
                                    self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::SemiAuto;
                                }
                            }
                        }
                        else {
                            self.dian_zhan_vec[zl.dev_id].ctrl_mode = simctrl::CtrlMode::Auto;
                            let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(zl.dev_id);
                            for j in jizuid_vec {
                                self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::Auto;
                            }
                            let dian_zhan_r_vec = self.get_dian_zhan_guan_lian_vec_from_dian_zhan_id(zl.dev_id);
                            for d in dian_zhan_r_vec {
                                self.dian_zhan_vec[d].ctrl_mode = simctrl::CtrlMode::Auto;
                                let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(d);
                                for j in jizuid_vec {
                                    self.ji_zu_vec[j].common_ji.ctrl_mode = simctrl::CtrlMode::Auto;
                                }
                            }
                        }
                        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                    }
                    _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::CtrlModeFail(*zl, String::from(zhiling::CTRL_MODE_FAIL_DESC), String::from(zhiling::CAUSE_CTRL_MODE_INVALID)))),
                }
            }
            _ => {}
        }
    }

    pub fn handle_prio(&mut self, zl : &ZhiLing) {
        match zl.dev_type {
            simctrl::DevType::DianZhan => {
                for d in 0..simctrl::ZONG_SHU_DIAN_ZHAN {
                    if d != zl.dev_id {
                        self.dian_zhan_vec[d].prio = false;
                    }
                    else {
                        self.dian_zhan_vec[d].prio = true;
                    }
                }
                self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
            }
            simctrl::DevType::JiZu => {
                let jizuid_vec = self.get_jizuid_vec_from_dianzhanid(zl.dev_id);
                for j in jizuid_vec {
                    if j != zl.dev_id {
                        self.ji_zu_vec[j].common_ji.prio = false;
                    }
                    else {
                        self.ji_zu_vec[j].common_ji.prio = true;
                    }
                }
                self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
            }
            _ => {}
        }
    }

    pub fn handle_an_dian_on(&mut self, zl : &ZhiLing) {
        self.ji_zu_vec[zl.dev_id].common_ji.current_range = jizu::JiZuRangeLeiXing::Wen;
        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
    }

    pub fn handle_an_dian_off(&mut self, zl : &ZhiLing) {
        self.ji_zu_vec[zl.dev_id].common_ji.current_range = jizu::JiZuRangeLeiXing::TingJi;
        self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
    }

    pub fn handle_an_dian_he_zha(&mut self, zl : &ZhiLing) {
        self.handle_he_zha_bing_che(zl);
    }

    pub fn handle_an_dian_fen_zha(&mut self, zl : &ZhiLing) {
        self.handle_fen_zha_jie_lie(zl);
    }

    pub fn handle_bian_su(&mut self, zl : &ZhiLing) {
        let duanluqi_status = self.get_duanluqi_from_jizuid(zl.dev_id).unwrap().is_on();
        match zl.zhi_ling_type {
            ZhiLingType::BianSu(delta) => {
                match self.ji_zu_vec[zl.dev_id].common_ji.current_range {
                    jizu::JiZuRangeLeiXing::Wen | jizu::JiZuRangeLeiXing::BianSu => {
                        if !self.ji_zu_vec[zl.dev_id].set_bian_su_params(delta, duanluqi_status) {
                            self.handle_zhi_ling_result_msg( Err(YingDaErr::BianSuFail(*zl, String::from(zhiling::BIAN_SU_FAIL_DESC), String::from(zhiling::CAUSE_BIAN_SU_FAIL_OUT_OF_LIMIT))));
                        }
                        else {
                            self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                        }
                    }
                    _ =>  self.handle_zhi_ling_result_msg( Err(YingDaErr::BianSuFail(*zl, String::from(zhiling::BIAN_SU_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_4)))),
                }
            }
            _ => self.handle_zhi_ling_result_msg( Err(YingDaErr::BianSuFail(*zl, String::from(zhiling::BIAN_SU_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_4)))),
        }
    }

    pub fn handle_bian_ya(&mut self, zl : &ZhiLing) {
        match zl.zhi_ling_type {
            ZhiLingType::BianYa(delta) => {
                match self.ji_zu_vec[zl.dev_id].common_ji.current_range {
                    jizu::JiZuRangeLeiXing::Wen | jizu::JiZuRangeLeiXing::BianYa => {
                        if !self.ji_zu_vec[zl.dev_id].set_bian_ya_params(delta) {
                            self.handle_zhi_ling_result_msg( Err(YingDaErr::BianYaFail(*zl, String::from(zhiling::BIAN_YA_FAIL_DESC), String::from(zhiling::CAUSE_BIAN_YA_FAIL_OUT_OF_LIMIT))));
                        }
                        else {
                            self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
                        }
                    }
                    _ =>  self.handle_zhi_ling_result_msg( Err(YingDaErr::BianYaFail(*zl, String::from(zhiling::BIAN_YA_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_5)))),
                }
            }
            _ =>  self.handle_zhi_ling_result_msg( Err(YingDaErr::BianYaFail(*zl, String::from(zhiling::BIAN_YA_FAIL_DESC), String::from(zhiling::CAUSE_JI_ZU_RANGE_DISMATCH_5)))),
        }
    }

    pub fn handle_jin_ji_ting_ji(&mut self, zl : &ZhiLing) {
        match self.ji_zu_vec[zl.dev_id].common_ji.current_range {
            jizu::JiZuRangeLeiXing::TingJi | jizu::JiZuRangeLeiXing::BeiCheZanTai | jizu::JiZuRangeLeiXing::BeiCheWanBi | jizu::JiZuRangeLeiXing::TingJiZanTai | jizu::JiZuRangeLeiXing::JinJiTingJiZanTai => {
                self.handle_zhi_ling_result_msg( Err(YingDaErr::JinJiTingJiFail(*zl, String::from(zhiling::JIN_JI_TING_JI_FAIL_DESC), String::from(zhiling:: CAUSE_JIN_JI_TING_JI_FAIL))));
                return ;
            }
            _ => {}
        }
        let duanluqiid = self.get_duanluqiid_from_jizuid(zl.dev_id).unwrap();
        if self.duan_lu_qi_vec[duanluqiid].is_on() {
            match self.duan_lu_qi_vec[duanluqiid].status {
                duanluqi::DuanLuQiStatus::On {..} => self.duan_lu_qi_vec[duanluqiid].set_off(),
                _ => {}
            }
        }
        self.ji_zu_vec[zl.dev_id].common_ji.current_range = jizu::JiZuRangeLeiXing::TingJiZanTai;
    }

    pub fn handle_xiao_sheng(&mut self, zl : &ZhiLing) {
        if self.is_xiao_sheng {
            self.handle_zhi_ling_result_msg( Err(YingDaErr::XiaoShengFail(*zl, String::from(zhiling::XIAO_SHENG_FAIL_DESC), String::from(zhiling:: CAUSE_XIAO_SHENG_FAIL))));
        }
        else{
            self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
        }
    }

    pub fn handle_ying_da(&mut self, zl : &ZhiLing) {
        if self.is_ying_da {
            self.handle_zhi_ling_result_msg( Err(YingDaErr::YingDaFail(*zl, String::from(zhiling::XIAO_SHENG_FAIL_DESC), String::from(zhiling:: CAUSE_XIAO_SHENG_FAIL))));
        }
        else{
            self.handle_zhi_ling_result_msg(Ok(YingDaType::Success(*zl)));
        }
    }
    pub fn query_xi_tong(&self, _req: &mut Request) -> IronResult<Response> {
        let x_ser = serde_json::to_string(&self).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, x_ser)))
    }

    pub fn query_xi_tong_pretty(&self, _req: &mut Request) -> IronResult<Response> {
        let x_ser = serde_json::to_string_pretty(&self).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, x_ser)))
    }

}

impl iron::middleware::Handler for XiTong {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        let x_ser = serde_json::to_string(&self).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, x_ser)))
    }
}

// impl mio::Handler for XiTong {
//     type Timeout = u64;
//     type Message = ();
//     fn timeout(&mut self, _event_loop: &mut EventLoop<Self>, _timeout: Self::Timeout) {
//         self.update();
//         let _ = _event_loop.timeout_ms(_timeout, _timeout).unwrap();
//     }
// }
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
pub struct XiTongHandler {
    pub xt : Arc<RwLock<XiTong>>,
}
impl iron::middleware::Handler for XiTongThread {
    fn handle(&self, _req: &mut Request) -> IronResult<Response> {
        use jsoninf::XiTongInf;
        use std::ops::Deref;
        let xt_shared = self.xt.clone();
        let xt_raw = xt_shared.read().unwrap();
        let x_ser = serde_json::to_string(&(XiTongInf::from_ob(xt_raw.deref()))).unwrap();
        let content_type = "application/json".parse::<Mime>().unwrap();
        Ok(Response::with((content_type, status::Ok, x_ser)))
    }
}
pub struct XiTongThread {
    pub xt : Arc<RwLock<XiTong>>,
    pub flow : Arc<RwLock<Vec<ZhiLing>>>,
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
        use zhiling::Condition;
        let xt_shared = self.xt.clone();
        let flow_share = self.flow.clone();
        let _ = thread::spawn(move || {
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
                }
                thread::sleep(Duration::from_millis(simctrl::FANG_ZHEN_BU_CHANG as u64));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use ::xitong::XiTong;
    use ::simctrl;
    // use ::zhiling::{ZhiLing, YingDaErr};
    use ::simctrl::{ Timer, TimerHandler};
    use std::thread;

    #[test]
    fn test_xi_tong() {
        let _xt = XiTong::new(0);
    }

    #[test]
    fn test_xi_tong_update() {
        let e_thread = thread::spawn(move || {
            Timer::new().start(&mut TimerHandler::new(XiTong::new(0)), simctrl::FANG_ZHEN_BU_CHANG as u64, simctrl::FANG_ZHEN_BU_CHANG as u64)
        });
         e_thread.join().unwrap();
    }
}
