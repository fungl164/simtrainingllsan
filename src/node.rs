#[derive(PartialEq, Copy, Clone, Debug)]
//节点状态，当节点电压幅值不为0，则为On，否则为Off
pub enum NodeStatus {
    On,
    Off,
}
#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Node {
  pub id : usize,
  pub vm : f64,
  pub f : f64,
  pub status : NodeStatus,
}

impl Node {
    pub fn new(_id : usize) -> Node {
        Node{
            id : _id,
            vm : 0.0,
            f : 0.0,
            status : NodeStatus::Off,
        }
    }
}
