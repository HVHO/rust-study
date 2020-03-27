use std::io::Write;
use std::fs::File;

fn main () {

    let mut file_ptr = match File::open("hello.txt") {
        Ok(t) => t,
        Err(_) => {
            println!("Fail to open file");
            let file_create = File::create("hello.txt");
            match file_create {
                Ok(t) => t,
                Err(_) => panic!("Fail to create file"),
            }
        }
    };

    file_ptr.write_all(b"hello, world!");

}