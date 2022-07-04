use nannou::prelude::*;
use rand::Rng;


const DEFAULT_SIZE: u32 = 32;
pub struct Node {
    pub value: String,
    pub pos_x: i32,
    pub pos_y: i32,
    pub size: u32,
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
    pub fn new_random(window_size_x: u32, window_size_y: u32) -> Node{
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







}
