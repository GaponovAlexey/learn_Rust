fn main() {
    let nums = [1, 2, 3, 4, 4, 3, 5, 5, 5, 6, 6, 7, 7, 9, 10];
    let res = find_duplicate(&nums);
    println!("{:?}", res);
}

fn find_duplicate(list: &[i32]) -> Vec<i32> {
    let mut dup = Vec::new();

    for i in 0..list.len() {
        for j in i + 1..list.len() {
            if list[i] == list[j] && !dup.contains(&list[i]) {
                dup.push(list[i]);
            }
        }
    }

    dup
}
