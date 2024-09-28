// This file contains structs and enums in Rust

pub enum Direction {
    North,
    South,
    East,
    West,
}
 enum Shape {
    Circle(f64),
    Rectangle(f64,f64),
    Square(f64)
}

pub fn calculate_area(shape:Shape)->f64{
   let ans= match shape{
    Shape::Circle(radius)=>3.14*radius*radius,
     Shape::Rectangle(width,height )=>{
        width*height
     },
     Shape::Square(side)=>side*side,
   };
   return ans;
}




// Function to move around based on direction
pub fn move_around(direction: Direction) {
   //match keyword helps in reducing effors of if else //PATTERN MATCHING
    match direction {
        Direction::North => println!("Moving North"),
        Direction::South => println!("Moving South"),
        Direction::East => println!("Moving East"),
        Direction::West => println!("Moving West"),
    }
}


