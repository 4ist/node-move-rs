use nannou::prelude::*;
use rand::Rng;


const GRID_MODE: bool = true;
const TILE_SIZE_PX: u32 = 32;
const DEFAULT_SIZE: u32 = TILE_SIZE_PX - 1;

#[derive(Clone)]
pub struct Node {
    pub value: String, // TODO eventually replace this with a type
    pub pos_x: i32,
    pub pos_y: i32,
    pub size: u32, // TODO move this to display module
}

impl Default for Node {
    fn default() -> Node {
        Node {
            value: String::from("@"),
            pos_x: 0,
            pos_y: 0,
            size: DEFAULT_SIZE,
        }
    }
}

impl Node {
    pub fn new(pos_x: i32, pos_y: i32) -> Node {
        Node {
            value: String::from("@"),
            pos_x,
            pos_y,
            size: DEFAULT_SIZE,
        }

    }
    pub fn new_random(window_size_x: u32, window_size_y: u32) -> Node{
        if !GRID_MODE{
            let half_x: i32 = (window_size_x/2) as i32;
            let half_y: i32 = (window_size_y/2) as i32;
           
            let rand_x_min: i32 = -half_x;
            let rand_x_max: i32 = half_x - DEFAULT_SIZE as i32;
            let rand_y_min: i32 = -half_y;
            let rand_y_max: i32 = half_y - DEFAULT_SIZE as i32;

            let rand_pos_x = rand::thread_rng().gen_range(rand_x_min..rand_x_max);
            let rand_pos_y = rand::thread_rng().gen_range(rand_y_min..rand_y_max);
            
            Node {
                pos_x: rand_pos_x,
                pos_y: rand_pos_y,
                ..Default::default()
            }
        }
        else {
            // find number of tiles in grid
            // generate tile number for node
            // multiply tile number by tile size 
            let half_tile_size = (TILE_SIZE_PX/2) as i32;
            let tiles_count_x = ((window_size_x/2)/TILE_SIZE_PX) as i32;
            let tiles_count_y = ((window_size_y/2)/TILE_SIZE_PX) as i32;
            let rand_tile_x = rand::thread_rng().gen_range(-tiles_count_x..tiles_count_x);
            let rand_tile_y = rand::thread_rng().gen_range(-tiles_count_y..tiles_count_y);
            
            let rand_pos_x = (rand_tile_x * TILE_SIZE_PX as i32) + half_tile_size;
            let rand_pos_y = (rand_tile_y * TILE_SIZE_PX as i32) + half_tile_size;
            
            Node {
                pos_x: rand_pos_x,
                pos_y: rand_pos_y,
                ..Default::default()
            }
        }
    }


    pub fn show(&self, draw: &Draw) {
        draw.ellipse()
            .w_h(self.size as f32, self.size as f32)
            .x_y(self.pos_x as f32, self.pos_y as f32)
            //.rotate(self.velocity.angle())
            .rgba(255.0, 0.0, 0.0, 0.85);
    }


}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_testable() {
        assert!(1 < 2);
    }

//    #[test]
    fn can_index_empty_vec() {
        let muh_vec: Vec<String> = Vec::new();
        for i in 0..10 {
            //muh_vec.push(null);
        }
        let something: &String = &muh_vec[4];
        assert!(1 < 2);
    }







}
