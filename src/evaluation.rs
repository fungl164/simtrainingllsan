pub const WEIGHT_CORRECTNESS : f64 = 0.6;
pub const WEIGHT_COMMAND : f64 = 0.2;
pub const WEIGHT_TIME : f64 = 0.2;

#[derive(PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct User {
  pub name : String,
  pub realname : String,
  pub age : u32,
  pub sex : Sex,
}

impl User {
    pub fn new(_id : usize, _name : String) -> User {
        User{
            uid : _id,
            name : _name,
            realname : String::from(""),
            age : 0,
            sex : Unisex,
        }
    }
}
