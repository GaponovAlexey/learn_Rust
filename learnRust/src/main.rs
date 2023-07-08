#[derive(Debug)]
enum Types {
    Int(i32),
    Float(f64),
    Bool(bool),
    Text(String),
}
fn main() {
    let list: Vec<Types> = vec![
        Types::Int(8),
        Types::Float(2.2),
        Types::Bool(true),
        Types::Text("ds".to_string()),
    ];
    match &list[1] {
        Types::Int(el) => {
            println!("{}", el);
        }
        Types::Float(el) => {
            println!("{}", el);
        }
        Types::Bool(el) => {
            println!("{}", el);
        }
        Types::Text(el) => {
            println!("{}", el);
        }
    }
}
