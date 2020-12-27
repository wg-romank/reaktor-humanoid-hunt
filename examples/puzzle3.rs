use std;
use std::fmt;
use std::collections::HashSet;
use std::collections::HashMap;

#[derive(Clone, PartialEq)]
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

fn neighboors(fields: &Vec<Contents>, w: usize, x: u32, y: u32) -> Vec<(u32, u32)> {
    vec![
        (x + 1, y),
        (x - 1, y),
        (x, y + 1),
        (x, y - 1),
    ].into_iter().filter(|(x, y)| { fields[index(*x, *y, w)] != Contents::Wall }).collect()
}

fn direction(from: (u32, u32), to: (u32, u32)) -> char {
    // x y
    match (to.0 as i32 - from.0 as i32, to.1 as i32 - from.1 as i32) {
        (1, 0) => 'R',
        (-1, 0) => 'L',
        (0, 1) => 'D',
        (0, -1) => 'U',
        _ => panic!("Should not be here {:#?} {:#?}", to, from),
    }
}

fn main() {
    let contents = std::fs::read_to_string("neural-strands").unwrap();

    let mut field = Vec::new();
    let w = 1000;
    let h = 1000;

    let mut start_x = 0;
    let mut start_y = 0;

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
                    "S" => { start_x = x; start_y = y; Contents::Start },
                    "F" => Contents::Finish,
                    "U" => { y -= 1; Contents::Empty },
                    "D" => { y += 1; Contents::Empty },
                    "L" => { x -= 1; Contents::Empty },
                    "R" => { x += 1; Contents::Empty },
                    _ => Contents::Wall,
                };

                field[index(x, y, w)] = to_set;
            }
        }
    }

    let mut Q = vec![(start_x, start_y)];
    let mut v = (start_x, start_y);
    let mut seen = HashSet::new();
    let mut backtrack = HashMap::new();
    let mut path = Vec::new();

    while field[index(v.0, v.1, w)] != Contents::Finish {
        v = Q.pop().unwrap();

        seen.insert(v);

        let mut nb = neighboors(&field, w, v.0, v.1)
            .into_iter()
            .filter(|a| !seen.contains(a))
            .collect::<Vec<(u32, u32)>>();

        for n in nb.clone() {
            backtrack.insert(n, v);
        }

        Q.append(&mut nb);
    }

    path.insert(0, v);

    while field[index(v.0, v.1, w)] != Contents::Start {
        v = backtrack[&v];
        path.insert(0, v);
    }

    let mut result = Vec::new();

    for i in 0..path.len() - 1 {
        result.push(direction(path[i], path[i + 1]));
    }

    println!("{}", result.iter().collect::<String>());
}