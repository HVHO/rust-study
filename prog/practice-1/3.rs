use std::io::Read;
use std::fs::File;


fn main () {

    let mut file_ptr = match File::open("hello.txt") {
        Ok(t) => t,
        Err(_) => {
            panic!("fail to open file")
        }
    };

    let mut buf = String::new();
    match file_ptr.read_to_string(&mut buf) {
        Ok(_) => println!("{}",buf),
        Err(_) => panic!("fail to read file")
    };

}