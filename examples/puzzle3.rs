use std;
use std::fmt;

#[derive(Clone)]
enum Contents {
    Empty,
    Wall,
    Start,
    Finish,
}

impl fmt::Display for Contents {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Contents::Empty => write!(f, "."),
            Contents::Wall => write!(f, "x"),
            Contents::Start => write!(f, "s"),
            Contents::Finish => write!(f, "f"),
        }
    }
}

fn index(x: u32, y: u32, w: usize) -> usize {
    (x as usize) * w + y as usize
}

fn inverted_index(i: usize, w: usize) -> (u32, u32) {
    ((i / w) as u32, (i % w) as u32)
}

fn parse_coordinates(s: &str) -> (u32, u32) {
    let items = s.split(',').collect::<Vec<&str>>();
    (items[0].parse::<u32>().unwrap(), items[1].parse::<u32>().unwrap())
}

fn display_field(field: &Vec<Contents>, w: usize) {
    for i in 0..w {
        for j in 0..w {
            print!("{}", field[index(i as u32, j as u32, w)]);
        }
        println!();
    }
}

fn main() {
    let contents = std::fs::read_to_string("neural-strands").unwrap();

    let mut field = Vec::new();
    let w = 1000;
    let h = 1000;

    field.resize(w * h, Contents::Wall);

    for l in contents.lines() {
        let items = l.split(' ').collect::<Vec<&str>>();
        let (mut x, mut y) = parse_coordinates(items[0]);
        x += w as u32 / 2; y += h as u32 / 2;

        field[index(x, y, w)] = Contents::Empty;

        if items.len() > 1 {
            let steps = items[1].split(',').collect::<Vec<&str>>();

            for s in steps {
                let to_set = match s {
                    "S" => Contents::Start,
                    "F" => Contents::Finish,
                    "U" => { y += 1; Contents::Empty },
                    "D" => { y -= 1; Contents::Empty },
                    "L" => { x -= 1; Contents::Empty },
                    "R" => { x += 1; Contents::Empty },
                    _ => Contents::Wall,
                };

                field[index(x, y, w)] = to_set;
            }
        }
    }

    display_field(&field, w);
    // println!("{}", contents);
}