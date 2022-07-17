use crate::Node;
use crate::Tile;


pub struct Cursor {
    pub node: Option<Node>, 
    pub pos_x: i32,
    pub pos_y: i32,
}


impl Cursor {
    pub fn new(pos_x: i32, pos_y: i32) -> Cursor {
        Cursor {
            node: None,
            pos_x,
            pos_y,
        }

    }


    pub fn set_location(&mut self, new_pos_x: i32, new_pos_y: i32){
        self.pos_x = new_pos_x;
        self.pos_y = new_pos_y;
        self.sync_node_position(new_pos_x, new_pos_y);
    }

    fn sync_node_position(&mut self, new_pos_x: i32, new_pos_y: i32){
        match &mut self.node {
            Some(node) => {
                node.pos_x = new_pos_x;
                node.pos_y = new_pos_y;
            }
            None => ()
        }
    }

}

