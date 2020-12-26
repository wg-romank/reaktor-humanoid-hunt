use std;

fn main() {
    let contents = std::fs::read_to_string("neural-strands").unwrap();

    println!("{}", contents);
}