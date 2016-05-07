use simctrl;
#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct DianZhan {
  pub id : usize,
  pub ctrl_mode : simctrl::CtrlMode,
  pub operating_station : simctrl::OperatingStation,
  pub ctrl_mode_she_zhi : simctrl::CtrlMode,
  pub operating_station_she_zhi : simctrl::OperatingStation,
  pub prio : bool,
  pub u : f64,
  pub f : f64,
  pub p : f64,
  pub p_yu_du : f64,
}

impl DianZhan {
    pub fn new(_id : usize) -> DianZhan {
        DianZhan{
            id : _id,
            ctrl_mode : simctrl::CtrlMode::Manual,
            operating_station : simctrl::OperatingStation::Local,
            ctrl_mode_she_zhi : simctrl::CtrlMode::Manual,
            operating_station_she_zhi : simctrl::OperatingStation::Local,
            prio : match _id {
                0 => true,
                _ => false,
            },
            u : 0.0f64,
            f : 0.0f64,
            p : 0.0f64,
            p_yu_du : 0.0f64,
        }
    }
}
