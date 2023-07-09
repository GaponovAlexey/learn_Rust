use std::collections::HashMap;

fn main() {
    let s = "Learn learn Rust with with me me me".to_lowercase();
    let mut count_map = HashMap::new();

    for s in s.split_whitespace() {
        let count = count_map.entry(s).or_insert(0);
        *count += 1;
    }

    println!("{:?}",count_map);
}
