use crate::Tile;
use crate::TileType;
use crate::Node;
use crate::NodeType;
use crate::Cursor;

use rand::Rng; // 0.8.0

pub struct Grid { 
    tiles: Vec<Tile>,  
    pub size_x: i32,
    pub size_y: i32,
    pub cursor : Option<Cursor>,
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
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
            cursor: None,
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
    
    pub fn setup_add_gold_tile(&mut self) {
        let x = rand::thread_rng().gen_range(0..self.size_x);
        let y = rand::thread_rng().gen_range(0..self.size_y);
        let tile = self.get_tile_mut(x, y);
        tile.tile_type = TileType::Gold;
    }
   
    pub fn setup_add_node_cursor(&mut self) {
        let x = self.size_x/2;
        let y = self.size_y/2;
        self.cursor = Some(Cursor::new(x, y));
    }
    
    pub fn setup_add_nodes_terrain(&mut self) {
        for x in 0..self.size_x {
            for y in 0..self.size_y {
                match self.get_tile(x, y).tile_type {
                    TileType::Water => (),
                    _ => {
                        match self.get_tile(x, y).node{
                            None => {
                                let mut node = Node::new(x, y);
                                if y % 3 == 0 && x % 2 == 0 {
                                    node.node_type = NodeType::Bush;
                                }
                                if x == 1 || y == 1 || x == self.size_x - 2 || y == self.size_y - 2 {
                                    node.node_type = NodeType::Rock;
                                }
                                match node.node_type {
                                    NodeType::None => (),
                                    _ => self.get_tile_mut(x, y).node = Some(node),
                                }
                            },
                            Some(_) => (),
                        }
                    }
                }
            }
        }
    }

    pub fn get_size(&self) -> (i32, i32) {
        (self.size_x, self.size_y)
    }

    fn get_index_from_position(&self, x: i32, y:i32) -> usize{
        (x * self.size_y + y) as usize
    }

    pub fn use_cursor(&mut self) -> Result<(), String> {
        match &mut self.cursor {
            Some(cursor) => {
                let (x, y) = (cursor.pos_x, cursor.pos_y);
                let index = ( x * self.size_y + y) as usize;
                //let tile = self.get_tile_mut(cursor.pos_x, cursor.pos_y);
                let tile = &mut self.tiles[index]; // I really don't want to access this directly,
                                                   // but It may be fine for now since this is a
                                                   // private field? Will redesign later
                match (&cursor.node, &tile.node) {
                    (None, None) => {
                        // Nothing 
                        return Ok(());
                    }
                    (None, Some(node)) => {
                        // Pickup 
                        let cnode = node.clone();
                        cursor.node = Some(cnode);
                        tile.node = None;
                        return Ok(());
                    }
                    (Some(node), None) => {
                        // Drop 
                        let cnode = node.clone();
                        tile.node = Some(cnode);
                        cursor.node = None;
                        return Ok(());
                    }
                    (Some(_), Some(_)) => {
                        // Collision
                        return Err("No action available; cursor and tile both contain a node".to_string()); 
                    }
                }
            },
            None => Err("Cursor does not exist".to_string())
        }
    }

    pub fn move_cursor(&mut self, direction: Direction) -> Result<(), String> {
        match &mut self.cursor {
            Some(cursor) => {
                let location: (i32, i32); 
                match direction {
                    Direction::UP => {
                        location = (cursor.pos_x + 0, cursor.pos_y + 1);
                    },
                    Direction::DOWN => {
                        location = (cursor.pos_x + 0, cursor.pos_y - 1);
                    },
                    Direction::LEFT => {
                        location = (cursor.pos_x - 1, cursor.pos_y + 0);
                    },
                    Direction::RIGHT => {
                        location = (cursor.pos_x + 1, cursor.pos_y + 0);
                    },
                }
                if !Grid::is_valid_location((self.size_x, self.size_x), location) {
                    return Err("Invalid cursor move".to_string()); 
                }
               
                if cursor.node.is_some() {
                    let index = (location.0 * self.size_y + location.1) as usize;
                    if self.tiles[index].node.is_some() {
                        return Err("Invalid cursor move; cursor and destination both contain a node".to_string()); 
                    }
                }

               
                if !Grid::is_valid_location((self.size_x, self.size_y), location) {
                    return Err("Invalid cursor move; out of bounds".to_string()); 
                }
                else {
                    cursor.set_location(location.0, location.1);
                    return Ok(());
                }
            },
            None => Err("Cursor does not exist".to_string())
        }
    }

    pub fn is_valid_location(grid_size: (i32, i32), location: (i32, i32)) -> bool {
        location.0 < grid_size.0 && 
        location.0 >= 0 &&  
        location.1 < grid_size.1 && 
        location.1 >= 0
    }

    pub fn add_node(&mut self, x: i32, y: i32) -> Result<(), String> {
        if !Grid::is_valid_location((self.size_x, self.size_y), (x, y)) {
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
        if !Grid::is_valid_location(self.get_size(), src) {
            return Err("Provided source position out of range".to_string()); 
        }
        if !Grid::is_valid_location(self.get_size(), dst) {
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

