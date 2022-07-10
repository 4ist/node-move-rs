use crate::Tile;
use crate::TileType;
use crate::Node;
use crate::NodeType;

pub trait TimedEvent {
    fn addEvent() {}
}

pub struct Grid { 
    //pub tiless: Vec<Vec<Tile>>,  
    tiles: Vec<Tile>,  
    pub size_x: i32,
    pub size_y: i32,
    pub cursor_pos: Option<(i32, i32)>,
}

impl Grid {
    pub fn new(size_x: i32, size_y: i32) -> Grid {
        let mut tiles: Vec<Tile> = Vec::new();
        for x in 0..size_x {
            for y in 0..size_y {
                tiles.push(Tile::new(x, y));
            }
        }
        Grid {
            size_x,
            size_y,
            tiles,
            cursor_pos: None,
        } 
    }


    pub fn get_tile<'a>(&'a self, x: i32, y: i32) -> &'a Tile {
        let index = ( x * self.size_y + y) as usize;
        &self.tiles[index]
    }
    
    pub fn get_tile_mut<'a>(&'a mut self, x: i32, y: i32) -> &'a mut Tile {
        let index = ( x * self.size_y + y) as usize;
        &mut self.tiles[index]
    }
 
    pub fn setup_add_tile_types(&mut self) {
        let size_x = self.size_x;
        let size_y = self.size_y;
        for x in 0..size_x {
            for y in 0..size_y {
                let tile = self.get_tile_mut(x, y);
                tile.tile_type = TileType::Ground;
                if x == 0 || y == 0 || x == size_x - 1 || y == size_y - 1 {
                    tile.tile_type = TileType::Water;
                }
            }
        }
    }
   
    pub fn setup_add_nodes(&mut self) {
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                match self.get_tile(x, y).tile_type {
                    TileType::Water => (),
                    _ => {
                        let mut node = Node::new(x, y);
                        if y % 3 == 0 && x % 2 == 0 {
                            node.node_type = NodeType::Bush;
                        }
                        if x == 1 || y == 1 || x == self.size_x - 2 || y == self.size_y - 2 {
                            node.node_type = NodeType::Rock;
                        }
                        if x == self.size_x/2 && y == self.size_y/2 {
                            node.node_type = NodeType::Cursor;
                            self.cursor_pos = Some((x, y));
                        }
                        match node.node_type {
                            NodeType::None => (),
                            _ => self.get_tile_mut(x, y).node = Some(node),
                        }
                    }
                }
            }
        }
    }

    pub fn is_valid_location(&self, location: (i32, i32)) -> bool {
        location.0 < self.size_x && 
        location.0 >= 0 &&  
        location.1 < self.size_y && 
        location.1 >= 0
    }

    pub fn add_node(&mut self, x: i32, y: i32) -> Result<(), String> {
        if !self.is_valid_location((x, y)) {
            return Err("Provided position out of range".to_string()); 
        }
        let target_tile = self.get_tile_mut(x, y);
        match target_tile.node {
            Some(_) => return Err("Position already contains a node".to_string()),
            None => {
                target_tile.node = Some(Node::new(x, y));
                return Ok(());
            }
        }
    }

    pub fn move_node(&mut self, src: (i32, i32), dst: (i32, i32)) -> Result<(), String> {
        if !self.is_valid_location(src) {
            return Err("Provided source position out of range".to_string()); 
        }
        if !self.is_valid_location(dst) {
            return Err("Provided source position out of range".to_string()); 
        } 
        if !self.get_tile(src.0, src.1).contains_node() {
            return Err("Source tile does not contain a node".to_string()); 
        }
        if self.get_tile(dst.0, dst.1).contains_node() {
            return Err("Destination tile already contains a node".to_string()); 
        }
        let node = self.get_tile(src.0, src.1).node.clone();
        self.get_tile_mut(dst.0, dst.1).node = node;
        self.get_tile_mut(src.0, src.1).node = None; 

        Ok(())
    }

    pub fn display() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE_X: i32 = 10;
    const SIZE_Y: i32 = 10;

    mod add {
        use super::*;

        #[test]
        fn can_initialize_empty_grid() {
            let grid = Grid::new(SIZE_X, SIZE_Y);
            assert!(grid.get_tile(0,0).node.is_none());
        }
       
        #[test]
        fn can_add_node() { 
            let mut grid = Grid::new(SIZE_X, SIZE_Y);
            assert!(grid.get_tile(2,3).node.is_none());
            assert!(grid.add_node(2,3).is_ok());
            assert!(grid.get_tile(2,3).node.is_some());
        }
        
        #[test]
        fn cannot_add_node_outside_range() {
            let mut grid = Grid::new(SIZE_X, SIZE_Y);
            assert!(grid.add_node(SIZE_X, SIZE_X).is_err());
            assert!(grid.add_node(-1, -1).is_err());
        }
        
        #[test]
        fn cannot_add_node_to_full_location() {
            let mut grid = Grid::new(SIZE_X, SIZE_Y);
            assert!(grid.add_node(0,0).is_ok());
            assert!(grid.add_node(0,0).is_err());
        }
    }

    mod movement {
        use super::*;

        #[test]
        fn can_move_node() {
            let mut grid = Grid::new(SIZE_X, SIZE_Y);
            grid.add_node(2,3).unwrap();
            assert!(grid.move_node((2,3),(4,5)).is_ok());
            assert!(grid.get_tile(2,3).node.is_none());
            assert!(grid.get_tile(4,5).node.is_some());
        }

        #[test]
        fn cannot_move_node_from_empty_tile() {
            let mut grid = Grid::new(SIZE_X, SIZE_Y);
            assert!(grid.move_node((2,3),(4,5)).is_err());
        }

        #[test]
        fn cannot_move_node_to_full_tile() {
            let mut grid = Grid::new(SIZE_X, SIZE_Y);
            grid.add_node(2,3).unwrap();
            grid.add_node(4,5).unwrap();
            assert!(grid.move_node((2,3),(4,5)).is_err());
        } 
        
    }
}

