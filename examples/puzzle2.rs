use std::collections::HashMap;

fn main() {
    let contents = std::fs::read_to_string("signal").unwrap();

    let count: HashMap<char, i32> = contents
        .chars()
        .collect::<Vec<char>>()
        .into_iter()
        .fold(HashMap::new(), |mut map, c| {
            let previous = map.remove(&c).unwrap_or(0);
            map.insert(c, previous + 1);
            map
        });

    let mut sorted_count = count
        .into_iter()
        .collect::<Vec<(char, i32)>>();

    sorted_count.sort_by_key(|k| -k.1);

    println!("{:#?}", &sorted_count);

    let password = sorted_count.into_iter().map(|(k, v)| k).collect::<String>();

    println!("{}", password);
}