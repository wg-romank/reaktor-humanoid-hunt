use std;

fn decode_bytes(l: &str) -> Vec<u8> {
    l
        .chars()
        .collect::<Vec<char>>()
        .iter()
        .map(|c| if *c == '0' { 0 } else { 1 }) // todo: any way to make it easier?
        .collect::<Vec<u8>>()
        .chunks(8)
        .map(|c| c.iter().fold(0 as u8, |acc, n| {(acc << 1) ^ *n})) // big endian
        .collect::<Vec<u8>>()
}

fn find_first_byte(message: &Vec<u8>) -> u8 {
    let mut idx: u8 = 0;
    let mut cursor: u8;

    loop {
        cursor = message[idx as usize];
        if (cursor as usize) < message.len() {
            break
        }

        idx += 1;
    }
    
    cursor
}

fn get_password_char(message: &Vec<u8>, cursor: u8) -> char {
    let mut result = cursor;
    loop {
        if (result as usize) < message.len() {
            result = message[result as usize];
        } else {
            return result as char
        }
    }
}


fn main() {
    let contents = std::fs::read_to_string("tatoo").unwrap();

    for l in contents.lines() {
        let message = decode_bytes(l);
        let cursor = find_first_byte(&message);
        print!("{}", get_password_char(&message, cursor));
    }

    println!();
}
