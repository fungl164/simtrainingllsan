use simctrl::ZhanWeiType;

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct Station {
    pub uid: usize,
    pub name: String,
    pub type: ZhanWeiType,
}

impl Station {
    pub fn new(_id: usize, _name: String, _type: ZhanWeiType) -> Station {
        Station {
            uid: _id,
            name: _name,
            type: _type,
        }
    }
}
