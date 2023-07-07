#[derive(Debug)]
struct Person {
    name: String,
    s_name: String,
    age: f32,
}
fn main() {
    let name = "iv".to_string();
    let s_name = "sa".to_string();

    let mas = Person {
        name,
        s_name,
        age: 22.0,
    };
    let mas2 = Person {
        age: 2.0,
        ..mas
    };

    println!("{:#?}", mas2);
    
}
