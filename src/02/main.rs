mod a;
mod b;

const EXAMPLE: &[u8] = include_bytes!("example.txt");
const INPUT: &[u8] = include_bytes!("input");
const DAY: &str = "Day 1";

fn main() {
    println!("{} task A:",DAY);
    println!("Example: {}",a::resolve(EXAMPLE));
    println!("Input: {}",a::resolve(INPUT));

    println!("\nTask B:");
    println!("Example: {}",b::resolve(EXAMPLE));
    println!("Input: {}",b::resolve(INPUT));
}
