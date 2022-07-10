
use crate::Node;

pub enum TileType {
    None,
    Ground,
    Water, 
}

pub struct Tile {
    pub node: Option<Node>, 
    pub pos_x: i32,
    pub pos_y: i32,
    pub tile_type: TileType,
    // can extend to include additional props
}

impl Tile {
    pub fn new(pos_x: i32, pos_y: i32) -> Tile {
        Tile {
            pos_x,
            pos_y,
            node: None,
            tile_type: TileType::None,
        }
    }

    pub fn contains_node(&self) -> bool {
        match self.node {
            None => return false,
            Some(_) => return true
        }
    }
}
