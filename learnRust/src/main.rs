fn main() {
    let (a, b, c) = sum(2, 4, 5);
    println!("{}{}{}", a, b, c);
}

fn sum(a: i32, b: i32, c: i32) -> (i32, i32, i32) {
    (a + a, b - b, c * c)
}
