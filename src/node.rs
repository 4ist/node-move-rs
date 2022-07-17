#[derive(Clone)]
pub struct Node {
    pub node_type: NodeType,
    pub pos_x: i32,
    pub pos_y: i32,
}

#[derive(Clone)]
pub enum NodeType {
    None,
    Bush,
    Rock,
}

impl Node {
    pub fn new(pos_x: i32, pos_y: i32) -> Node {
        Node {
            node_type: NodeType::None,
            pos_x,
            pos_y,
        }
    }
}

