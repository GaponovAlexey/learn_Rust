use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    let path = "text.txt";
    match read_file(path) {
        Ok(data) => println!("result in file => {:?}", data),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(_) => println!("create file"),
                Err(e) => panic!("{}", e),
            },
            other => panic!("my Error {}", other),
        },
    };
}

fn read_file(path: &str) -> io::Result<String> {
    let mut data = String::new();
    File::open(path)?.read_to_string(&mut data)?;
    Ok(data)
}
