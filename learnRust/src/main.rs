fn main() {
    let s1 = "la".to_string();
    let s2 = "w".to_string();
    for el in s1.bytes() {
        println!("{}", el);
    }

    println!("{}", &s1[..2]);
}
