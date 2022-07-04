mod node;
use crate::node::Node;

mod display;
use crate::display::display;
// use crate::node::Node;

fn main() {
    println!("Hello, worlt!");

    let grid: Vec<Vec<Node>> = vec![
        vec![
            Node {value: String::from("a"), ..Default::default()},
            Node {value: String::from("b"), ..Default::default()}
            ],
        vec![
            Node {value: String::from("c"), ..Default::default()},
            Node {value: String::from("d"), ..Default::default()}
            ]
        ];
    // display_grid(grid);
    display();
    println!("Goodbye");
}

fn display_grid(grid: Vec<Vec<Node>>) {
    println!("Displaying Grid");
    println!("{}", get_grid_display(grid));
}

fn get_grid_display(grid: Vec<Vec<Node>>) -> String {
   let mut display = String::from(""); 
    for line in grid {
        for node in line{
            display.push_str(&node.value);
        }
    }
    return display;
}




#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_grid() -> Vec<Vec<Node>>{
        let grid: Vec<Vec<Node>> = vec![
            vec![
                Node {value: String::from("a"), ..Default::default()},
                Node {value: String::from("b"), ..Default::default()}
                ],
            vec![
                Node {value: String::from("c"), ..Default::default()},
                Node {value: String::from("d"), ..Default::default()}
                ]
            ]; 
        return grid;
    }

    #[test]
    fn is_testable() {
        assert!(1 < 2);
    }
    
    #[test]
    fn can_get_test_grid() {
        let test_grid: Vec<Vec<Node>> = get_test_grid();
        assert!(test_grid.len() == 2);
        assert!(test_grid[0].len() == 2);
    }
    
    #[test]
    fn can_get_grid_display() {
        let test_grid: Vec<Vec<Node>> = get_test_grid();
        let display: String = get_grid_display(test_grid);
        assert!(display.len() == 4);
    }
}
