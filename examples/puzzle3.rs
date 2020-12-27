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

#[derive(Debug, Clone)]
struct FieldCoordinate {
    x: u32,
    y: u32,
}

impl FieldCoordinate {
    fn new(s: &str, w: usize, h: usize) -> FieldCoordinate {
        let items = s.split(',').collect::<Vec<&str>>();
        FieldCoordinate {
            x: items[0].parse::<u32>().unwrap() + w as u32 / 2,
            y: items[1].parse::<u32>().unwrap() + h as u32 / 2,
        }
    }

    fn move_self(&mut self, to: String) -> &FieldCoordinate {
        match to {
            "U" => self.y -= 1,
            "D" => self.y += 1,
            "L" => self.x -= 1,
            "R" => self.x += 1,
        };

        self
    }

    fn direction(&self, to: &FieldCoordinate) -> char {
        match (to.x as i32 - self.x as i32, to.y as i32 - self.y as i32) {
            (1, 0) => 'R',
            (-1, 0) => 'L',
            (0, 1) => 'D',
            (0, -1) => 'U',
            _ => panic!("Should not be here {:#?} {:#?}", self, to),
        }
    }
}

struct Field {
    field: Vec<Contents>,
    w: usize,
    h: usize,
}

impl Field {
    fn new(contents: String) -> (Field, FieldCoordinate) {
        let w = 1000;
        let h = 1000;
    
        let mut start = FieldCoordinate { x: 0, y: 0 };
    
        let mut field = Vec::new();
        field.resize(w * h, Contents::Wall);
    
        for l in contents.lines() {
            let items = l.split(' ').collect::<Vec<&str>>();
            let point = FieldCoordinate::new(items[0]);
    
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

        Field { field, w, h, start: FieldCoordinate { x: start_x, x: start_y } }
    }

    fn index(&self, u: FieldCoordinate) -> Contents {
        let (x, y) = (u.x, u.y);
        self[(x as usize) * w + y as usize]
    }

    fn neighboors(&self, u: FieldCoordinate) -> Vec<FieldCoordinate> {
        let (x, y) = (u.x, u.y);

        vec![
            FieldCoordinate { x: x + 1, y },
            FieldCoordinate { x: x - 1, y },
            FieldCoordinate { x, y: y + 1 },
            FieldCoordinate { x, y: y - 1 },
        ].into_iter().filter(|(x, y)| { self.index(*x, *y) != Contents::Wall }).collect()
    }
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

fn display_field(field: &Vec<Contents>, w: usize) {
    for i in 0..w {
        for j in 0..w {
            print!("{}", field[index(i as u32, j as u32, w)]);
        }
        println!();
    }
}

fn bfs(field: Field, start: FieldCoordinate) -> Vec<FieldCoordinate> {
    let mut Q = vec![start];
    let mut v = start;

    let mut seen = HashSet::new();

    let mut backtrack = HashMap::new();

    while field.index(v) != Contents::Finish {
        v = Q.pop().unwrap();

        seen.insert(v);

        let mut nb = field.neighboors(v)
            .into_iter()
            .filter(|a| !seen.contains(a))
            .collect::<Vec<FieldCoordinate>>();

        for n in nb.clone() {
            backtrack.insert(n, v);
        }

        Q.append(&mut nb);
    }

    let mut path = Vec::new();

    path.insert(0, v);

    while field.index(v) != Contents::Start {
        v = backtrack[&v];
        path.insert(0, v);
    }

    path
}

fn main() {
    let contents = std::fs::read_to_string("neural-strands").unwrap();

    let (field, start) = Field::new(contents);

    let path = bfs(field, start);

    let mut result = Vec::new();

    for i in 0..path.len() - 1 {
        result.push(path[i].direction(path[i + 1]));
    }

    println!("{}", result.iter().collect::<String>());
}