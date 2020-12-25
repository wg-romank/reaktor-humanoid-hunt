use std;


fn main() {
    let contents = std::fs::read_to_string("tatoo").unwrap();

    for l in contents.lines() {
        let message = l
            .chars()
            .collect::<Vec<char>>()
            .iter()
            .map(|c| if *c == '0' { 0 } else { 1 }) // todo: any way to make it easier?
            .collect::<Vec<u8>>()
            .chunks(8)
            .map(|c| c.iter().fold(0 as u8, |acc, n| {(acc << 1) ^ *n})) // big endian
            .collect::<Vec<u8>>();

        let mut idx: u8 = 0;
        let mut cursor: u8;

        loop {
            cursor = message[idx as usize];
            if (cursor as usize) < message.len() {
                break
            }

            idx += 1;
        }

        loop {
            if (cursor as usize) < message.len() {
                cursor = message[cursor as usize];
            } else {
                print!("{}", cursor as char);
                break
            }
        }
    }
    println!("");
}
