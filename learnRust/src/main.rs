fn main() {
    let nums = [1, 2, 3, 4, 4, 3, 5, 5, 5, 6, 6, 7, 7, 9, 10];
    println!("{:?}", find_duplicate(&nums));
}

fn find_duplicate(list: &[i32]) -> Vec<i32> {
    let mut dup: Vec<i32> = Vec::new();

    for i in 0..(list.len()) {
        for j in (i + 1)..(list.len()) {
            if list[i] == list[j] {
                dup.push(list[i]);
            }
        }
    }
    dup
}
