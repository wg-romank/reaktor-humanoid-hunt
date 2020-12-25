use std;


fn main() {
    let contents = std::fs::read_to_string("tatoo").expect("Failed to read input");

    for l in contents.lines() {
        let bytes = l
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|c| if *c == '0' { 0 } else { 1 })
            .collect::<Vec<u8>>()
            .chunks(8)
            .map(|c| c.iter().fold(0 as u8, |acc, n| {(acc << 1) ^ *n}))
            .collect::<Vec<u8>>();

        let mut cursor: u8;
        let mut idx: u8 = 0;

        loop {
            cursor = bytes[idx as usize];
            if cursor as usize <= bytes.len() {
                break
            }

            idx += 1;
        }

        loop {
            if (cursor as usize) < bytes.len() {
                cursor = bytes[cursor as usize];
            } else {
                print!("{}", cursor as char);
                break
            }
        }
    }
    println!("");
}
