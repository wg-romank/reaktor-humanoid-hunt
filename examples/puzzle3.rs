use std;


#[derive(Clone)]
enum Contents {
    Empty,
    Wall,
    Start,
    Finish,
}

fn index(x: u32, y: u32, w: u32) -> usize {
    (x * w + y) as usize
}

fn inverted_index(i: usize, w: u32) -> (u32, u32) {
    (i as u32/ w, i as u32 % w)
}

fn parse_coordinates(s: &str) -> (u32, u32) {
    let items = s.split(',').collect::<Vec<&str>>();
    (items[0].parse::<u32>().unwrap(), items[1].parse::<u32>().unwrap())
}

fn main() {
    let contents = std::fs::read_to_string("neural-strands").unwrap();

    let mut field = Vec::new();
    let w = 1000;
    let h = 1000;

    field.resize(w * h, Contents::Wall);

    for l in contents.lines() {
        let items = l.split(' ').collect::<Vec<&str>>();
        let (x, y) = parse_coordinates(items[0]);
        println!("{} {}", x, y);
    }

    // println!("{}", contents);
}