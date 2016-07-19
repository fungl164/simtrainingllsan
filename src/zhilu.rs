#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ZhiLuStatus {
    On,
    Off,
}
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct ZhiLu {
  pub uid : usize,
  pub p : f64,
  pub q : f64,
  pub i : f64,
  pub status : ZhiLuStatus,
  pub is_lian_tong : bool,
}

impl ZhiLu {
    pub fn new(_id : usize) -> ZhiLu {
        ZhiLu{
            uid : _id,
            p : 0.0,
            q : 0.0,
            i : 0.0,
            status : ZhiLuStatus::Off,
            is_lian_tong : false,
        }
    }
}
