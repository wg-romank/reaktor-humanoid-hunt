use std::collections::HashMap;

fn most_frequent_char(candidates: &Vec<char>) -> char {
    let mut max_key = -1;
    let cmap = candidates
        .iter()
        .fold(HashMap::new(), |mut map, c| {
            let previous = map.remove(&c).unwrap_or(0);
            let new_val = previous + 1;
            map.insert(c, new_val);

            if new_val > max_key {
                max_key = new_val
            };

            map
        });

    let mut result: char = ' ';

    for (k, v) in cmap {
        if max_key == v {
            result = *k;
            break;
        };
    };

    result
}

fn most_frequent_after(signal: &Vec<char>, c: char) -> char {
    let (_, candidates): (char, Vec<char>) = signal
        .iter()
        .fold((' ', Vec::new()), |(prev, mut acc), cc| {
            if prev == c {
                acc.push(*cc);
            };

            (*cc, acc)
        });

    most_frequent_char(&candidates)
}

fn main() {
    let contents = std::fs::read_to_string("signal").unwrap();

    let signal = contents
        .chars()
        .collect::<Vec<char>>();

    let first = most_frequent_char(&signal);

    let mut next = first;
    loop {
        print!("{}", next);
        next = most_frequent_after(&signal, next);

        if next == ';' {
            break
        }
    }

    println!();
}