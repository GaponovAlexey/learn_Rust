use std::fs::File;


fn main() {
    let path = "lol.txt";
    let f = File::open(path).expect("my err");
   
    
}
