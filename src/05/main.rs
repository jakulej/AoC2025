mod a;
mod b;

use std::fs;

const DAY: &str = "Day 5";

fn main() {
    
    let example = fs::read_to_string("src/05/example.txt").unwrap();
    let input = fs::read_to_string("src/05/input").unwrap();
    
    println!("{} task A:",DAY);
    println!("Example: {}",a::resolve(&example));
    println!("Input: {}",a::resolve(&input));

    println!("\nTask B:");
    println!("Example: {}",b::resolve(&example));
    println!("Input: {}",b::resolve(&input));
}
