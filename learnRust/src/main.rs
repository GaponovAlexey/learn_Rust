fn main() {
    let mut list: Vec<i32> = Vec::with_capacity(5);

    list.push(2);

    println!("{:?}", list);
    println!("{:?}", list.capacity());
}
