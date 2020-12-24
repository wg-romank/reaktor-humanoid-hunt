use std;


fn main() {
    let contents = std::fs::read_to_string("tatoo").expect("Failed to read input");
    for l in contents.lines() {
        println!("{}", l);
    }
}
