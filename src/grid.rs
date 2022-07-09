
use crate::Tile;
use crate::Node;

pub struct Grid {
     
    pub size_x: i32,
    pub size_y: i32,
    //pub nodes: Vec<Vec<Option<Node>>>,  
    pub tiles: Vec<Vec<Tile>>,  
                                       
                                       
    // Should I add a "Space" or "Tile" type for grid? I
    // like the idea, this could be like the "background"
    // which doesn't change if the node in the space/tile
    // changes
    
    // should the grid know anything about the display? -> no 
    // Maybe grid and node should only contain array values and all translation be managed by
    // display
    //  -> yea
}
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Grid {
    pub fn new(size_x: i32, size_y: i32) -> Grid {
        let mut tiles: Vec<Vec<Tile>> = Vec::new();
        for x in 0..size_x {
            let mut row = Vec::new();
            for y in 0..size_y {
                row.push(Tile::new(x, y));
            }
            tiles.push(row); 
        }
        Grid {
            size_x,
            size_y,
            tiles,
        } 
    }

    pub fn is_valid_location(&self, location: (i32, i32)) -> bool {
        location.0 < self.size_x && 
        location.0 >= 0 &&  
        location.1 < self.size_y && 
        location.1 >= 0
    }

    pub fn add_node(&mut self, x: i32, y: i32) -> Result<(), String> {
        // TODO make a custom error type/enum 
        if !self.is_valid_location((x, y)) {
            return Err("Provided position out of range".to_string()); 
        }
        let mut target_tile = &mut self.tiles[x as usize][y as usize];
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
        if !self.tiles[src.0 as usize][src.1 as usize].contains_node() {
            return Err("Source tile does not contain a node".to_string()); 
        }
        if self.tiles[dst.0 as usize][dst.1 as usize].contains_node() {
            return Err("Destination tile already contains a node".to_string()); 
        }
        let node = self.tiles[src.0 as usize][src.1 as usize].node.clone();
        self.tiles[dst.0 as usize][dst.1 as usize].node = node;
        self.tiles[src.0 as usize][src.1 as usize].node = None;
        //let src_tile = &self.tiles[src.0 as usize][src.1 as usize];
        //let dst_tile = &self.tiles[dst.0 as usize][dst.1 as usize];
        
        


        return Ok(());


    }

    pub fn display() {}
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIZE_X: i32 = 10;
    const SIZE_Y: i32 = 10;
    #[test]
    fn is_testable() {
        assert!(1 < 2);
    }

    #[test]
    fn can_initialize_empty_grid() {
        let grid = Grid::new(SIZE_X, SIZE_Y);
        assert!(grid.tiles[0][0].node.is_none());
    }
   
    #[test]
    fn can_add_node() { 
        let mut grid = Grid::new(SIZE_X, SIZE_Y);
        assert!(grid.add_node(2,3).is_ok());
        assert!(grid.tiles[2][3].node.is_some());
    }
    
    #[test]
    fn cannot_add_node_outside_range() {
        let mut grid = Grid::new(SIZE_X, SIZE_Y);
        assert!(grid.add_node(SIZE_X, SIZE_X).is_err());
        assert!(grid.add_node(-1, -1).is_err());
        // TODO test for specific error type
    }
    
    #[test]
    fn cannot_add_node_to_full_location() {
        let mut grid = Grid::new(SIZE_X, SIZE_Y);
        assert!(grid.add_node(0,0).is_ok());
        assert!(grid.add_node(0,0).is_err());
    }

    #[test]
    fn can_move_node() {
        let mut grid = Grid::new(SIZE_X, SIZE_Y);
        assert!(grid.add_node(2,3).is_ok());
        assert!(grid.tiles[2][3].node.is_some());
        assert!(grid.tiles[4][5].node.is_none());
        assert!(grid.move_node((2,3),(4,5)).is_ok());
        assert!(grid.tiles[2][3].node.is_none());
        assert!(grid.tiles[4][5].node.is_some());
    }
    fn cannot_move_none_node() {}
    fn cannot_move_node_to_full_space() {} 
}
