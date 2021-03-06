pub const FU_ZAI_P_MAX : f64 = 1200.0;
pub const FU_ZAI_Q_MAX : f64 = 960.0;
pub const FU_ZAI_P_FACTOR : f64 = 0.8;
pub const FU_ZAI_Q_FACTOR : f64 = 0.6;
pub const FU_ZAI_Q_P : f64 = 0.75;
pub const ZHONG_ZAI_YU_ZHI_P : f64 = 480.0;
pub const ZHONG_ZAI_YU_ZHI_Q : f64 = 384.0;

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct FuZai {
  pub uid : usize,
  pub u : f64,
  pub p : f64,
  pub q : f64,
  pub i : f64,
  pub p_max : f64,
  pub q_max : f64,
  pub is_online : bool,
}

impl FuZai {
    pub fn new(_id : usize) -> FuZai {
        FuZai{
            uid : _id,
            u : 0.0,
            p : 0.0,
            q : 0.0,
            i : 0.0,
            p_max : FU_ZAI_P_MAX,
            q_max : FU_ZAI_Q_MAX,
            is_online: false,
        }
    }
}
