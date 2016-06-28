#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum DuanLuQiStatus {
    Off {fault : bool, ready_to_bing_che : bool},
    On {fault : bool, ready_to_jie_lie : bool},
}
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DuanLuQi {
  pub uid : usize,
  pub status : DuanLuQiStatus,
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

impl DuanLuQi {
    pub fn new(_id : usize) -> DuanLuQi {
        DuanLuQi{
            uid : _id,
            status : DuanLuQiStatus::Off {fault : false, ready_to_bing_che : false},
            uab : 0.0,
            ubc : 0.0,
            uca : 0.0,
            ia : 0.0,
            ib : 0.0,
            ic : 0.0,
            f : 0.0,
            gong_lv_yu_bao_jing : false,
            zong_he_gu_zhang : false,
            xiang_jian_bu_ping_heng : false,
            tong_bu_shi_bai : false,
            fen_duan_shi_bai : false,
            guo_dian_liu : false,
            he_zha_shi_bai : false,
        }
    }
    pub fn is_on(&self) -> bool {
        match self.status {
            DuanLuQiStatus::On {..} => return true,
            DuanLuQiStatus::Off {..} => return false,
        }
    }
    pub fn is_off(&self) -> bool{
        !self.is_on()
    }
    pub fn is_fault(&self) -> bool{
        match self.status {
            DuanLuQiStatus::On{fault, ..} => {
                if fault {
                    return true;
                }
                else {
                    return false;
                }
            }
            DuanLuQiStatus::Off{fault, ..} => {
                if fault {
                    return true;
                }
                else {
                    return false;
                }
            }
        }
    }
    pub fn is_ready_to_bing_che(&self) -> bool{
        match self.status {
            DuanLuQiStatus::On{..} => return false,
            DuanLuQiStatus::Off{ready_to_bing_che, ..} => {
                if ready_to_bing_che {
                    return true;
                }
                else {
                    return false;
                }
            }
        }
    }
    pub fn is_ready_to_jie_lie(&self) -> bool{
        match self.status {
            DuanLuQiStatus::Off{..} => return false,
            DuanLuQiStatus::On{ready_to_jie_lie, ..} => {
                if ready_to_jie_lie {
                    return true;
                }
                else {
                    return false;
                }
            }
        }
    }
    pub fn set_on(&mut self) {
        match self.status {
            DuanLuQiStatus::Off{fault : f, ..} => self.status = DuanLuQiStatus::On{fault : f, ready_to_jie_lie : false},
            DuanLuQiStatus::On{..} => {},
        }
    }

    pub fn set_off(&mut self) {
        match self.status {
            DuanLuQiStatus::On{fault : f, ..} => self.status = DuanLuQiStatus::Off{fault : f, ready_to_bing_che : false},
            DuanLuQiStatus::Off{..} => {},
        }
    }

}
